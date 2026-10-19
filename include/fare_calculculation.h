#include <string>
#pragma once

struct TariffPlan {
    std::string name;    // econom class
    double costPerKm;    // ціна за км
    double baseCostCar;  // базова ціна за машину
    double costPerMin;   // ціна за хвилину
    double minTotalFare; // мінімальна ціна
};

// Вхідні параметри розрахунку поїздки
struct TripRequest {
    std::string orderId;
    std::string passengerPhone;
    int tariffChoice;       // 1 - Економ, 2 - Стандарт, 3 - Комфорт, 4 - Бізнес
    double distanceKm;      // Дистанція за GPS
    double durationMinutes; // Тривалість поїздки
    double surgeMultiplier; // Коефіцієнт навантаження (1.0 .. 2.5)
    std::string promoCode;  // Промокод на знижку
    int paymentMethodChoice; // 1 - Картка, 2 - Готівка, 3 - Apple/Google Pay
};

// Розраховані фінансові складові поїздки
struct FareCalculationResult {
    TariffPlan plan;
    double baseDistanceCost;
    double baseTimeCost;
    double subtotalBeforeSurge;
    double surgeAddedAmount;
    double fareWithSurge;
    double promoDiscountAmount;
    double finalPayableAmount;
    double vatTaxAmount;      // ПДВ 20%
    double serviceFeeShare;   // Частка комісії сервісу 15%
    double driverNetEarnings; // Чистий заробіток перевізника
    std::string paymentMethodName;
    std::string promoStatusMessage;
};

bool inputTripData(TripRequest &req);
FareCalculationResult processFareCalculation(const TripRequest &req);
void printTripReceipt(const TripRequest &req, const FareCalculationResult &res);
