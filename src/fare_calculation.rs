use crate::input::read_line;

#[cxx::bridge]
mod ffi {
    // Тарифний план поїздки
    enum Tariff {
        Economy = 1,
        Standard = 2,
        Comfort = 3,
        Business = 4,
    }

    // Спосіб оплати поїздки
    enum PaymentMethod {
        Card = 1,
        Cash = 2,
        MobilePay = 3,
    }

    // Вхідні параметри розрахунку поїздки
    struct TripRequest {
        tariff: Tariff,
        distance_km: f64,      // Дистанція за GPS
        duration_minutes: f64, // Тривалість поїздки
        surge_multiplier: f64, // Коефіцієнт навантаження (1.0 .. 2.5)
        promo_code: String,    // Промокод на знижку
        payment_method: PaymentMethod,
    }

    // Параметри тарифу; усі суми в копійках
    struct TariffPlan {
        name: String,
        base_cost_car: i64,  // базова ціна за машину
        cost_per_km: i64,    // ціна за км
        cost_per_min: i64,   // ціна за хвилину
        min_total_fare: i64, // мінімальна ціна
    }

    // Розраховані фінансові складові поїздки; усі суми в копійках
    struct FareCalculationResult {
        plan: TariffPlan,
        base_distance_cost: i64,
        base_time_cost: i64,
        subtotal_before_surge: i64,
        surge_added_amount: i64,
        fare_with_surge: i64,
        promo_discount_amount: i64,
        final_payable_amount: i64,
        vat_percent: i64,         // Ставка ПДВ, %
        vat_tax_amount: i64,      // ПДВ у складі суми до оплати
        service_fee_percent: i64, // Ставка комісії сервісу, %
        service_fee_share: i64,   // Частка комісії сервісу
        driver_net_earnings: i64, // Чистий заробіток перевізника
        payment_method_name: String,
        promo_status_message: String,
    }

    unsafe extern "C++" {
        include!("cpp/fare_calculation.h");

        #[cxx_name = "processFareCalculation"]
        fn process_fare_calculation(req: &TripRequest) -> Result<FareCalculationResult>;
    }
}

pub(crate) use ffi::Tariff;

// Дані замовлення разом із параметрами для розрахунку
pub(crate) struct TripOrder {
    pub(crate) order_id: String,
    pub(crate) passenger_phone: String,
    pub(crate) request: ffi::TripRequest,
}

pub fn run() {
    println!("========================================================");
    println!(" АС «ВИКЛИК ТАКСІ» — МОДУЛЬ ТАРИФІКАЦІЇ ТА ОПЛАТИ (UC6) ");
    println!("========================================================\n");

    let Some(order) = read_trip_order() else {
        eprintln!("\n[ЗБІЙ] Помилка вводу параметрів розрахунку вартості.");
        return;
    };

    settle(&order);
}

// Розраховує вартість і друкує чек; повертає суму до оплати в копійках
pub(crate) fn settle(order: &TripOrder) -> Option<i64> {
    match ffi::process_fare_calculation(&order.request) {
        Ok(result) => {
            print_receipt(order, &result);
            Some(result.final_payable_amount)
        }
        Err(err) => {
            eprintln!("\n[ПОМИЛКА] Розрахунок вартості неможливий: {err}.");
            None
        }
    }
}

fn read_trip_order() -> Option<TripOrder> {
    let order_id = read_line("Введіть номер замовлення (наприклад, ORD-7789): ")?;
    if order_id.is_empty() {
        return None;
    }
    let passenger_phone = read_line("Введіть номер телефону пасажира (+380...): ")?;

    println!("\nОберіть тарифний план поїздки:");
    println!("  1 — Економ   (подача 55 грн, 9.50 грн/км, 1.80 грн/хв, мін. 80 грн)");
    println!("  2 — Стандарт (подача 70 грн, 11.50 грн/км, 2.00 грн/хв, мін. 95 грн)");
    println!("  3 — Комфорт  (подача 85 грн, 14.00 грн/км, 2.50 грн/хв, мін. 120 грн)");
    println!("  4 — Бізнес   (подача 140 грн, 22.00 грн/км, 4.50 грн/хв, мін. 200 грн)");
    let tariff = match read_line("Ваш вибір (1-4): ")?.parse::<u32>() {
        Ok(1) => Tariff::Economy,
        Ok(3) => Tariff::Comfort,
        Ok(4) => Tariff::Business,
        _ => Tariff::Standard,
    };

    let distance_km = read_positive(
        "Введіть розраховану дистанцію маршруту (км): ",
        "[ПОМИЛКА] Дистанція поїздки має бути додатною.",
    )?;
    let duration_minutes = read_positive(
        "Введіть розрахунковий час у дорозі (хв): ",
        "[ПОМИЛКА] Тривалість поїздки має бути додатною.",
    )?;

    Some(TripOrder {
        order_id,
        passenger_phone,
        request: read_trip_request(tariff, distance_km, duration_minutes)?,
    })
}

// Читає коефіцієнт попиту, промокод і спосіб оплати для вже відомого маршруту
pub(crate) fn read_trip_request(
    tariff: Tariff,
    distance_km: f64,
    duration_minutes: f64,
) -> Option<ffi::TripRequest> {
    let surge_multiplier =
        read_line("Введіть коефіцієнт завантаженості / попиту (1.0 — норма, до 2.5): ")?
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && *v >= 1.0 && *v <= 2.5)
            .unwrap_or(1.0);

    // Регістр зводиться тут: toupper у C++ не змінює кирилицю
    let mut promo_code =
        read_line("Введіть промокод на знижку (наприклад, KNU2026, або '-' якщо немає): ")?
            .to_uppercase();
    if promo_code.is_empty() {
        promo_code = "-".to_string();
    }

    println!("\nОберіть спосіб оплати:");
    println!("  1 — Банківська картка (онлайн токен)");
    println!("  2 — Готівка водію");
    println!("  3 — Apple Pay / Google Pay");
    let payment_method = match read_line("Ваш вибір (1-3): ")?.parse::<u32>() {
        Ok(2) => ffi::PaymentMethod::Cash,
        Ok(3) => ffi::PaymentMethod::MobilePay,
        _ => ffi::PaymentMethod::Card,
    };

    Some(ffi::TripRequest {
        tariff,
        distance_km,
        duration_minutes,
        surge_multiplier,
        promo_code,
        payment_method,
    })
}

// Читає додатне скінченне число; інакше виводить помилку і повертає None
fn read_positive(prompt: &str, error: &str) -> Option<f64> {
    let value = read_line(prompt)?
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite() && *v > 0.0);
    if value.is_none() {
        println!("{error}");
    }
    value
}

// Сума в копійках у вигляді "123.45"
fn format_money(amount: i64) -> String {
    let sign = if amount < 0 { "-" } else { "" };
    let abs = amount.unsigned_abs();
    format!("{sign}{}.{:02}", abs / 100, abs % 100)
}

fn print_receipt(order: &TripOrder, res: &ffi::FareCalculationResult) {
    let req = &order.request;

    println!("\n================================================================");
    println!("             ФІСКАЛЬНИЙ ЕЛЕКТРОННИЙ ЧЕК ТА ОПЛАТА               ");
    println!("                   АС «ВИКЛИК ТАКСІ» КИЇВ                       ");
    println!("================================================================");
    println!(" Замовлення №:        {}", order.order_id);
    println!(" Клієнт:              {}", order.passenger_phone);
    println!(" Обраний тариф:       {}", res.plan.name);
    println!(
        " Маршрут:             {:.2} км | Час: {:.2} хв",
        req.distance_km, req.duration_minutes
    );
    println!("----------------------------------------------------------------");
    println!(" КАЛЬКУЛЯЦІЯ ВАРТОСТІ:");
    println!(
        "   - Базова посадка та подача авто:       {:>9} грн",
        format_money(res.plan.base_cost_car)
    );
    println!(
        "   {:<39}{:>9} грн",
        format!(
            "- Дистанція ({:.2} км x {} грн):",
            req.distance_km,
            format_money(res.plan.cost_per_km)
        ),
        format_money(res.base_distance_cost)
    );
    println!(
        "   {:<39}{:>9} грн",
        format!(
            "- Час у дорозі ({:.2} хв x {} грн):",
            req.duration_minutes,
            format_money(res.plan.cost_per_min)
        ),
        format_money(res.base_time_cost)
    );
    println!(
        "   Базова сума без коефіцієнтів:          {:>9} грн",
        format_money(res.subtotal_before_surge)
    );

    if req.surge_multiplier > 1.0 {
        println!(
            "   {:<39}{:>9} грн",
            format!("Коефіцієнт попиту (x{:.2}):", req.surge_multiplier),
            format!("+{}", format_money(res.surge_added_amount))
        );
    }

    if res.promo_discount_amount > 0 {
        println!(
            "   Знижка за промокодом:                  {:>9} грн",
            format!("-{}", format_money(res.promo_discount_amount))
        );
    }
    println!("   Примітка промокоду: {}", res.promo_status_message);

    // Доплата, якщо сума після знижки менша за мінімальну вартість тарифу
    let cost_after_discount = res.fare_with_surge - res.promo_discount_amount;
    let min_fare_surcharge = res.final_payable_amount - cost_after_discount;
    if min_fare_surcharge > 0 {
        println!(
            "   Проміжний підсумок:                    {:>9} грн",
            format_money(cost_after_discount)
        );
        println!(
            "   {:<39}{:>9} грн",
            format!(
                "Доплата до мін. тарифу ({} грн):",
                format_money(res.plan.min_total_fare)
            ),
            format!("+{}", format_money(min_fare_surcharge))
        );
    }
    println!("----------------------------------------------------------------");
    println!(
        " ДО ОПЛАТИ:                               {:>9} ГРН",
        format_money(res.final_payable_amount)
    );
    // ПДВ уже входить у суму до оплати, тому виводиться окремо від розподілу платежу
    println!(
        "   {:<39}{:>9} грн",
        format!("у т. ч. ПДВ ({}%):", res.vat_percent),
        format_money(res.vat_tax_amount)
    );
    println!(" Спосіб оплати:       {}", res.payment_method_name);
    println!(" Статус транзакції:   УСПІШНО АВТОРИЗОВАНО (AUTH_OK)");
    println!("----------------------------------------------------------------");
    println!(" РОЗПОДІЛ ОПЛАТИ:");
    println!(
        "   {:<39}{:>9} грн",
        format!("- Комісія сервісу ({}%):", res.service_fee_percent),
        format_money(res.service_fee_share)
    );
    println!(
        "   - Нараховано перевізнику:              {:>9} грн",
        format_money(res.driver_net_earnings)
    );
    println!("================================================================");
    println!(" Електронний чек дійсний для бухгалтерії та гарантійних випадків.\n");
}
