#include "fare_calculation.h"

#include <algorithm>
#include <iomanip>
#include <iostream>

// Отримання параметрів обраного тарифу
TariffPlan getTariffPlan(int choice)
{
    switch (choice) {
        case 1:
            return {"Економ", 55.0, 9.50, 1.80, 80.0};
        case 3:
            return {"Комфорт", 85.0, 14.00, 2.50, 120.0};
        case 4:
            return {"Бізнес", 140.0, 22.00, 4.50, 200.0};
        case 2:
        default:
            return {"Стандарт", 70.0, 11.50, 2.00, 95.0};
    }
}

bool inputTripData(TripRequest &req)
{
    std::cout << "========================================================\n";
    std::cout << " АС «ВИКЛИК ТАКСІ» — МОДУЛЬ ТАРИФІКАЦІЇ ТА ОПЛАТИ (UC6) \n";
    std::cout << "========================================================\n\n";

    std::cout << "Введіть номер замовлення (наприклад, ORD-7789): ";
    if (!(std::cin >> req.orderId)) {
        return false;
    }

    std::cout << "Введіть телефон пасажира (+380...): ";
    if (!(std::cin >> req.passengerPhone)) {
        return false;
    }

    std::cout << "\nОберіть тарифний план поїздки:\n";
    std::cout << "  1 — Економ   (подача 55 грн, 9.50 грн/км, 1.80 грн/хв)\n";
    std::cout << "  2 — Стандарт (подача 70 грн, 11.50 грн/км, 2.00 грн/хв)\n";
    std::cout << "  3 — Комфорт  (подача 85 грн, 14.00 грн/км, 2.50 грн/хв)\n";
    std::cout << "  4 — Бізнес   (подача 140 грн, 22.00 грн/км, 4.50 грн/хв)\n";
    std::cout << "Ваш вибір (1-4): ";
    if (!(std::cin >> req.tariffChoice) || req.tariffChoice < 1 ||
        req.tariffChoice > 4) {
        req.tariffChoice = 2; // Default Standard
    }

    std::cout << "Введіть розраховану дистанцію маршруту (км): ";
    if (!(std::cin >> req.distanceKm) || req.distanceKm <= 0.0) {
        std::cout << "[ПОМИЛКА] Дистанція поїздки має бути додатною.\n";
        return false;
    }

    std::cout << "Введіть розрахунковий час у дорозі (хв): ";
    if (!(std::cin >> req.durationMinutes) || req.durationMinutes <= 0.0) {
        std::cout << "[ПОМИЛКА] Тривалість поїздки має бути додатною.\n";
        return false;
    }

    std::cout
        << "Введіть коефіцієнт завантаженості / попиту (1.0 — норма, до 2.5): ";
    if (!(std::cin >> req.surgeMultiplier) || req.surgeMultiplier < 1.0) {
        req.surgeMultiplier = 1.0;
    }

    std::cout << "Введіть промокод на знижку (наприклад, KNU2026, або '-' якщо "
                 "немає): ";
    std::cin >> req.promoCode;

    std::cout << "\nОберіть спосіб оплати:\n";
    std::cout << "  1 — Банківська картка (онлайн токен)\n";
    std::cout << "  2 — Готівка водію\n";
    std::cout << "  3 — Apple Pay / Google Pay\n";
    std::cout << "Ваш вибір (1-3): ";
    if (!(std::cin >> req.paymentMethodChoice) || req.paymentMethodChoice < 1 ||
        req.paymentMethodChoice > 3) {
        req.paymentMethodChoice = 1;
    }

    return true;
}

// Обробка промокоду
double evaluatePromoDiscount(const std::string &code, double currentTotal,
                             std::string &msg)
{
    std::string c = code;
    transform(c.begin(), c.end(), c.begin(), ::toupper);

    if (c == "KNU2026" || c == "KNU") {
        double discount = currentTotal * 0.20; // Знижка 20%
        msg = "Промокод [KNU2026] застосовано: знижка 20%";
        return discount;
    } else if (c == "START" || c == "NEWUSER") {
        double discount = 40.0; // Фіксована знижка 40 грн
        if (discount > currentTotal * 0.5) {
            discount = currentTotal * 0.5;
        }
        msg = "Промокод [" + c + "] застосовано: фіксована знижка 40 грн";
        return discount;
    } else if (c == "-" || c == "NONE" || c == "НІ") {
        msg = "Промокод не використовувався";
        return 0.0;
    } else {
        msg = "Промокод [" + code +
              "] не знайдено або термін дії вичерпано (знижка 0 грн)";
        return 0.0;
    }
}

FareCalculationResult processFareCalculation(const TripRequest &req)
{
    FareCalculationResult res;
    res.plan = getTariffPlan(req.tariffChoice);

    // Базові складові
    res.baseDistanceCost = req.distanceKm * res.plan.costPerKm;
    res.baseTimeCost = req.durationMinutes * res.plan.costPerMin;
    res.subtotalBeforeSurge =
        res.plan.baseCostCar + res.baseDistanceCost + res.baseTimeCost;

    // Врахування динамічного коефіцієнта попиту
    res.fareWithSurge = res.subtotalBeforeSurge * req.surgeMultiplier;
    res.surgeAddedAmount = res.fareWithSurge - res.subtotalBeforeSurge;

    // Застосування знижки
    res.promoDiscountAmount = evaluatePromoDiscount(
        req.promoCode, res.fareWithSurge, res.promoStatusMessage);

    double costAfterDiscount = res.fareWithSurge - res.promoDiscountAmount;
    res.finalPayableAmount = std::max(costAfterDiscount, res.plan.minTotalFare);

    // Фіскальні та партнерські розщеплення платежу
    res.vatTaxAmount =
        res.finalPayableAmount * 0.20 / 1.20; // ПДВ у складі ціни
    res.serviceFeeShare =
        res.finalPayableAmount * 0.15; // 15% комісія платформи
    res.driverNetEarnings = res.finalPayableAmount - res.serviceFeeShare;

    switch (req.paymentMethodChoice) {
        case 2:
            res.paymentMethodName = "Готівковий розрахунок";
            break;
        case 3:
            res.paymentMethodName = "Apple Pay / Google Pay";
            break;
        case 1:
        default:
            res.paymentMethodName = "Безготівковий (банківська картка)";
            break;
    }

    return res;
}

void printTripReceipt(const TripRequest &req, const FareCalculationResult &res)
{
    std::cout << "\n==========================================================="
                 "=====\n";
    std::cout
        << "             ФІСКАЛЬНИЙ ЕЛЕКТРОННИЙ ЧЕК ТА ОПЛАТА               \n";
    std::cout
        << "                   АС «ВИКЛИК ТАКСІ» КИЇВ                       \n";
    std::cout
        << "================================================================\n";
    std::cout << " Замовлення №:        " << req.orderId << "\n";
    std::cout << " Клієнт:              " << req.passengerPhone << "\n";
    std::cout << " Обраний тариф:       " << res.plan.name << "\n";
    std::cout << std::fixed << std::setprecision(2);
    std::cout << " Маршрут:             " << req.distanceKm
              << " км | Час: " << req.durationMinutes << " хв\n";
    std::cout
        << "----------------------------------------------------------------\n";
    std::cout << " КАЛЬКУЛЯЦІЯ ВАРТОСТІ:\n";
    std::cout << "   - Базова посадка та подача авто:       " << std::setw(9)
              << res.plan.baseCostCar << " грн\n";
    std::cout << "   - Дистанція (" << req.distanceKm << " км x "
              << res.plan.costPerKm << " грн):          " << std::setw(9)
              << res.baseDistanceCost << " грн\n";
    std::cout << "   - Час у дорозі (" << req.durationMinutes << " хв x "
              << res.plan.costPerMin << " грн):       " << std::setw(9)
              << res.baseTimeCost << " грн\n";
    std::cout << "   Базова сума без коефіцієнтів:          " << std::setw(9)
              << res.subtotalBeforeSurge << " грн\n";

    if (req.surgeMultiplier > 1.0) {
        std::cout << "   Коефіцієнт підвищеного попиту (x"
                  << std::setprecision(2) << req.surgeMultiplier
                  << "):   " << std::setw(9)
                  << ("+" + std::to_string((int)res.surgeAddedAmount) + "." +
                      std::to_string((int)(res.surgeAddedAmount * 100) % 100))
                  << " грн\n";
    }

    if (res.promoDiscountAmount > 0.0) {
        std::cout << "   Знижка за промокодом:                  "
                  << std::setw(9)
                  << ("-" + std::to_string((int)res.promoDiscountAmount) + "." +
                      std::to_string((int)(res.promoDiscountAmount * 100) %
                                     100))
                  << " грн\n";
    }
    std::cout << "   Примітка промокоду: " << res.promoStatusMessage << "\n";
    std::cout
        << "----------------------------------------------------------------\n";
    std::cout << " ДО ОПЛАТИ:                               " << std::setw(9)
              << res.finalPayableAmount << " ГРН\n";
    std::cout << " Спосіб оплати:       " << res.paymentMethodName << "\n";
    std::cout << " Статус транзакції:   УСПІШНО АВТОРИЗОВАНО (AUTH_OK)\n";
    std::cout
        << "----------------------------------------------------------------\n";
    std::cout << " В ТОМУ ЧИСЛІ:\n";
    std::cout << "   - ПДВ (20%):                          " << std::setw(9)
              << res.vatTaxAmount << " грн\n";
    std::cout << "   - Комісія сервісу (15%):               " << std::setw(9)
              << res.serviceFeeShare << " грн\n";
    std::cout << "   - Нараховано перевізнику:              " << std::setw(9)
              << res.driverNetEarnings << " грн\n";
    std::cout
        << "================================================================\n";
    std::cout << " Електронний чек дійсний для бухгалтерії та гарантійних "
                 "випадків.\n\n";
}
