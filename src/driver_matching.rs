use crate::input::read_line;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CarClass {
    Economy,
    Standard,
    Comfort,
    Business,
}

impl fmt::Display for CarClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CarClass::Economy => "Економ",
            CarClass::Standard => "Стандарт",
            CarClass::Comfort => "Комфорт",
            CarClass::Business => "Бізнес",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Position {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Position {
    pub(crate) fn distance(&self, other: &Position) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

pub(crate) fn estimated_time(dist: f64) -> f64 {
    const MINUTES_IN_HOUR: f64 = 60.0;
    let avg_speed: f64 = 35.0;
    (dist / avg_speed) * MINUTES_IN_HOUR
}

pub(crate) struct Driver {
    pub(crate) id: String,
    pub(crate) name: String,
    car_model: String,
    car_plate: String,
    car_class: CarClass,
    is_available: bool,
    pub(crate) rating: f64,
    pub(crate) total_trips: u32, // Кількість оцінених поїздок
    pos: Position,
}

pub(crate) struct Order {
    pub(crate) order_id: String,
    pub(crate) pos: Position,
    pub(crate) desired_class: CarClass,
}

// Радіус пошуку вільного водія
pub(crate) const SEARCH_RADIUS_KM: f64 = 10.0;

pub(crate) fn candidate_drivers() -> Vec<Driver> {
    vec![
        Driver {
            id: String::from("DRV-1001"),
            name: String::from("Олександр Шевченко"),
            car_model: String::from("Skoda Octavia"),
            car_plate: String::from("AA1234BC"),
            car_class: CarClass::Economy,
            is_available: true,
            rating: 4.92,
            total_trips: 318,
            pos: Position { x: 2.5, y: 3.1 },
        },
        Driver {
            id: String::from("DRV-1002"),
            name: String::from("Володимир Зеленський"),
            car_model: String::from("Potuzhno Leet"),
            car_plate: String::from("AA7777AA"),
            car_class: CarClass::Business,
            is_available: false,
            rating: 4.37,
            total_trips: 57,
            pos: Position { x: 1.2, y: 0.8 },
        },
        Driver {
            id: String::from("DRV-1003"),
            name: String::from("Кирило Буданов"),
            car_model: String::from("Skoda Kodiaq"),
            car_plate: String::from("KA1337KA"),
            car_class: CarClass::Comfort,
            is_available: true,
            rating: 4.67,
            total_trips: 146,
            pos: Position { x: 4.2, y: 4.9 },
        },
        Driver {
            id: String::from("DRV-1004"),
            name: String::from("Михайло Запобіжний"),
            car_model: String::from("BMW i8"),
            car_plate: String::from("AA9931BC"),
            car_class: CarClass::Comfort,
            is_available: true,
            rating: 4.72,
            total_trips: 89,
            pos: Position { x: 8.0, y: 1.3 },
        },
        Driver {
            id: String::from("DRV-1005"),
            name: String::from("Мухамед Браун"),
            car_model: String::from("LADA Priora Sedan"),
            car_plate: String::from("AA1344BC"),
            car_class: CarClass::Economy,
            is_available: true,
            rating: 4.52,
            total_trips: 23,
            pos: Position { x: 4.5, y: 3.1 },
        },
        Driver {
            id: String::from("DRV-1006"),
            name: String::from("Евгеній Руйда"),
            car_model: String::from("Volkswagen Tiguan"),
            car_plate: String::from("AA1214BC"),
            car_class: CarClass::Standard,
            is_available: true,
            rating: 4.99,
            total_trips: 512,
            pos: Position { x: 1.5, y: 7.1 },
        },
    ]
}

pub(crate) fn match_driver<'a>(
    candidates: &'a [Driver],
    order: &Order,
    max_range: f64,
) -> Option<(&'a Driver, f64, f64)> {
    let mut matching_candidates = Vec::new();

    for candidate in candidates {
        // Водій повинен бути вільним
        if !candidate.is_available {
            continue;
        }

        // Клас авто повинен відповідати потрібному
        if candidate.car_class != order.desired_class {
            continue;
        }

        // Географічне положення
        let dist = order.pos.distance(&candidate.pos);
        if dist > max_range {
            continue;
        }

        let eta = estimated_time(dist);
        matching_candidates.push((candidate, dist, eta));
    }

    matching_candidates
        .into_iter()
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

// Читає число (координата може бути від'ємною); Не коректне введення - None
pub(crate) fn read_coordinate(prompt: &str) -> Option<f64> {
    let value = read_line(prompt)?
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite());
    if value.is_none() {
        println!("[ПОМИЛКА] Координата має бути числом.");
    }
    value
}

pub(crate) fn read_order() -> Option<Order> {
    let order_id = read_line("Введіть номер замовлення (наприклад, ORD-7789): ")?;
    if order_id.is_empty() {
        return None;
    }

    println!("\n--- Місцезнаходження пасажира ---");
    let x = read_coordinate("  X: ")?;
    let y = read_coordinate("  Y: ")?;

    println!("\nОберіть клас авто:");
    println!("  1 — Економ");
    println!("  2 — Стандарт");
    println!("  3 — Комфорт");
    println!("  4 — Бізнес");
    let desired_class = match read_line("Ваш вибір (1-4): ")?.parse::<u32>() {
        Ok(1) => CarClass::Economy,
        Ok(2) => CarClass::Standard,
        Ok(3) => CarClass::Comfort,
        Ok(4) => CarClass::Business,
        _ => {
            println!("[ПОМИЛКА] Невідомий клас авто.");
            return None;
        }
    };

    Some(Order {
        order_id,
        pos: Position { x, y },
        desired_class,
    })
}

pub(crate) fn print_match_result(order: &Order, result: Option<(&Driver, f64, f64)>) {
    println!("\n========================================================");
    println!("               РЕЗУЛЬТАТ ПІДБОРУ ВОДІЯ                  ");
    println!("========================================================");
    println!(" Замовлення №:        {}", order.order_id);
    println!(
        " Координати:          ({:.2}; {:.2})",
        order.pos.x, order.pos.y
    );
    println!(" Клас авто:           {}", order.desired_class);
    println!("--------------------------------------------------------");

    match result {
        Some((driver, dist, eta)) => {
            println!(" Водій:               {} ({})", driver.name, driver.id);
            println!(" Рейтинг:             {:.2}", driver.rating);
            println!(
                " Авто:                {} | {}",
                driver.car_model, driver.car_plate
            );
            println!(" Відстань до пасажира: {:.2} км", dist);
            println!(" Час подачі:          ~{:.0} хв", eta);
            println!("\n [OK] Водія призначено на замовлення.");
        }
        None => {
            println!("\n [УВАГА!] Немає вільних водіїв цього класу поблизу.");
            println!(" Спробуйте інший клас авто або повторіть запит пізніше.");
        }
    }
    println!("========================================================");
    println!(" Дякуємо, що обрали АС «ВИКЛИК ТАКСІ»!\n");
}

pub fn run() {
    println!("==================================================");
    println!("     АС «ВИКЛИК ТАКСІ» - МОДУЛЬ ПІДБОРУ ВОДІЯ     ");
    println!("==================================================\n");

    let Some(order) = read_order() else {
        eprintln!("\n[ПОМИЛКА] Некоректні вхідні дані замовлення.");
        return;
    };

    let drivers = candidate_drivers();
    let result = match_driver(&drivers, &order, SEARCH_RADIUS_KM);
    print_match_result(&order, result);
}
