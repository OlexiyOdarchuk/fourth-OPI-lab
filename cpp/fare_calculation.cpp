#include "fare_calculation.h"

#include <algorithm>
#include <cctype>
#include <cmath>
#include <cstdint>
#include <iomanip>
#include <sstream>
#include <stdexcept>
#include <string>

#include "src/fare_calculation.rs.h"

// Межі вхідних даних. Якщо значення більше/менше - переповнення
constexpr double MAX_DISTANCE_KM = 1000.0;
constexpr double MAX_DURATION_MINUTES = 24 * 60.0;

// Ставка ПДВ і комісія платформи у відсотках
constexpr uint8_t VAT_PERCENT = 20;
constexpr uint8_t PLATFORM_COMMISSION = 20;

// Частка від суми у відсотках з округленням до копійки
Money percentOf(Money amount, Money percent)
{
    return (amount * percent + 50) / 100;
}

// Отримання параметрів обраного тарифу
TariffPlan getTariffPlan(Tariff tariff)
{
    switch (tariff) {
        case Tariff::Economy:
            return {"Економ", 5500, 950, 180, 8000};
        case Tariff::Comfort:
            return {"Комфорт", 8500, 1400, 250, 12'000};
        case Tariff::Business:
            return {"Бізнес", 14'000, 2200, 450, 20'000};
        case Tariff::Standard:
        default:
            return {"Стандарт", 7000, 1150, 200, 9500};
    }
}

// Обробка промокоду
Money evaluatePromoDiscount(const std::string &code, Money currentTotal,
                            std::string &msg)
{
    std::string c = code;
    std::transform(c.begin(), c.end(), c.begin(), [](unsigned char ch) {
        return std::toupper(ch);
    });

    if (c == "KNU2026" || c == "KNU") {
        Money discount = percentOf(currentTotal, 20); // Знижка 20%
        msg = "Промокод [KNU2026] застосовано: знижка 20%";
        return discount;
    } else if (c == "START" || c == "NEWUSER") {
        Money discount = 4000; // Фіксована знижка 40 грн
        if (discount > currentTotal / 2) {
            discount = currentTotal / 2;
            std::ostringstream oss;
            oss << "Промокод [" << c << "] застосовано: знижка "
                << (discount / 100) << '.' << std::setfill('0') << std::setw(2)
                << std::abs(discount % 100) << " грн";

            msg = oss.str();
        } else {
            msg = "Промокод [" + c + "] застосовано: фіксована знижка 40 грн";
        }
        return discount;
    } else if (c == "-" || c == "NONE" || c == "НІ") {
        msg = "Промокод не використовувався";
        return 0;
    } else {
        msg = "Промокод [" + code +
              "] не знайдено або термін дії вичерпано (знижка 0 грн)";
        return 0;
    }
}

FareCalculationResult processFareCalculation(const TripRequest &req)
{
    // Заперечення в умовах убирає ще NaN
    if (!(req.distance_km > 0 && req.distance_km <= MAX_DISTANCE_KM)) {
        throw std::invalid_argument(
            "дистанція поїздки має бути в межах від 0 до 1000 км");
    }
    if (!(req.duration_minutes > 0 &&
          req.duration_minutes <= MAX_DURATION_MINUTES)) {
        throw std::invalid_argument(
            "тривалість поїздки має бути в межах від 0 до 1440 хв");
    }

    FareCalculationResult res{};
    res.plan = getTariffPlan(req.tariff);

    // Базові складові
    res.base_distance_cost =
        std::llround(req.distance_km * res.plan.cost_per_km);
    res.base_time_cost =
        std::llround(req.duration_minutes * res.plan.cost_per_min);
    res.subtotal_before_surge =
        res.plan.base_cost_car + res.base_distance_cost + res.base_time_cost;

    // Врахування динамічного коефіцієнта попиту
    res.fare_with_surge =
        std::llround(res.subtotal_before_surge * req.surge_multiplier);
    res.surge_added_amount = res.fare_with_surge - res.subtotal_before_surge;

    // Застосування знижки
    std::string promoMessage;
    res.promo_discount_amount = evaluatePromoDiscount(
        std::string(req.promo_code), res.fare_with_surge, promoMessage);
    res.promo_status_message = promoMessage;

    Money costAfterDiscount = res.fare_with_surge - res.promo_discount_amount;
    res.final_payable_amount =
        std::max(costAfterDiscount, res.plan.min_total_fare);

    // Фіскальні та партнерські розщеплення платежу
    // ПДВ у складі ціни з округленням до копійки
    res.vat_percent = VAT_PERCENT;
    res.vat_tax_amount =
        (res.final_payable_amount * VAT_PERCENT + (100 + VAT_PERCENT) / 2) /
        (100 + VAT_PERCENT);
    res.service_fee_percent = PLATFORM_COMMISSION;
    res.service_fee_share = percentOf(res.final_payable_amount,
                                      PLATFORM_COMMISSION); // комісія платформи
    res.driver_net_earnings = res.final_payable_amount - res.service_fee_share;

    switch (req.payment_method) {
        case PaymentMethod::Cash:
            res.payment_method_name = "Готівковий розрахунок";
            break;
        case PaymentMethod::MobilePay:
            res.payment_method_name = "Apple Pay / Google Pay";
            break;
        case PaymentMethod::Card:
        default:
            res.payment_method_name = "Безготівковий (банківська картка)";
            break;
    }

    return res;
}
