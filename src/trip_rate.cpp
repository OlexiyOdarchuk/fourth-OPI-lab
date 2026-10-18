#include "../include/trip_rate.h"

#include <cmath>
#include <iomanip>
#include <iostream>

std::string complimentToString(Compliment c)
{
    switch (c) {
        case Compliment::Cleanliness:
            return "Чисте та охайне авто";
        case Compliment::Politeness:
            return "Ввічливий та уважний водій";
        case Compliment::ComfortableDriving:
            return "Плавне та безпечне водіння";
        case Compliment::Speed:
            return "Швидке прибуття та оптимальний маршрут";
        case Compliment::NoComments:
            return "Без особливих приміток";
    }
    return "";
}

TierMeta getTierMeta(DriverTier tier)
{
    switch (tier) {
        case DriverTier::Platinum:
            return {"Платиновий водій (Platinum Tier)", "Вищий пілотаж сервісу",
                    +15.0, false};
        case DriverTier::Gold:
            return {"Золотий водій (Gold Tier)", "Надійний перевізник", +8.0,
                    false};
        case DriverTier::Standard:
            return {"Стандартний водій (Standard Tier)",
                    "Відповідає базовим вимогам", 0.0, false};
        case DriverTier::Risk:
            return {"У зоні ризику (Risk Tier)", "Потребує переатестації",
                    -20.0, true};
    }
    return {};
}

DriverTier determineDriverTier(double rating)
{
    if (rating >= 4.90) {
        return DriverTier::Platinum;
    }
    if (rating >= 4.75) {
        return DriverTier::Gold;
    }
    if (rating >= 4.50) {
        return DriverTier::Standard;
    }
    return DriverTier::Risk;
}

double calculateUpdatedRating(double currentRating, int previousCount,
                              int newScore)
{
    if (previousCount <= 0) {
        return static_cast<double>(newScore);
    }

    double totalAccumulated =
        currentRating * previousCount + static_cast<double>(newScore);
    double calculated = totalAccumulated / (previousCount + 1);
    return std::round(calculated * 1000.0) / 1000.0;
}

RatingResult processRatingData(const RatingInput &input)
{
    RatingResult res;
    res.oldRating = input.currentRating;
    res.newRating = calculateUpdatedRating(
        input.currentRating, input.totalPreviousTrips, input.tripStars);
    res.ratingDelta = res.newRating - res.oldRating;
    res.updatedTripCount = input.totalPreviousTrips + 1;
    res.tier = determineDriverTier(res.newRating);
    res.tierMeta = getTierMeta(res.tier);
    return res;
}

bool inputRatingData(RatingInput &input)
{
    std::cout << "==================================================\n";
    std::cout << "  АС «ВИКЛИК ТАКСІ» — МОДУЛЬ ОЦІНЮВАННЯ ПОЇЗДКИ   \n";
    std::cout << "==================================================\n\n";

    std::cout << "Введіть ID водія (наприклад, DRV-1042): ";
    if (!(std::cin >> input.driverId)) {
        return false;
    }

    std::cin.ignore(1000, '\n');
    std::cout << "Введіть ім'я водія: ";
    std::getline(std::cin, input.driverName);
    if (input.driverName.empty()) {
        input.driverName = "Олександр Коваленко";
    }

    std::cout << "Введіть поточний рейтинг (1.00 - 5.00): ";
    if (!(std::cin >> input.currentRating) || input.currentRating < 1.0 ||
        input.currentRating > 5.0) {
        return false;
    }

    std::cout << "Кількість попередніх поїздок: ";
    if (!(std::cin >> input.totalPreviousTrips) ||
        input.totalPreviousTrips < 0) {
        return false;
    }

    std::cout << "\nОцінка поїздки (1-5 зірок): ";
    if (!(std::cin >> input.tripStars) || input.tripStars < 1 ||
        input.tripStars > 5) {
        return false;
    }

    std::cout << "\nОберіть компліменти (можна декілька через пробіл, "
                 "закінчіть '0'):\n";
    std::cout << "  1 — Охайне авто | 2 — Ввічливість | 3 — Комфорт | 4 — "
                 "Швидкість | 0 — Завершити\n";

    int choice;
    while (std::cin >> choice && choice != 0) {
        if (choice >= 1 && choice <= 4) {
            input.compliments.push_back(static_cast<Compliment>(choice - 1));
        }
    }
    if (input.compliments.empty()) {
        input.compliments.push_back(Compliment::NoComments);
    }

    std::cin.ignore(1000, '\n');
    std::cout << "Текстовий відгук: ";
    std::getline(std::cin, input.comment);

    return true;
}

void printRatingReceipt(const RatingInput &input, const RatingResult &result)
{
    std::cout << "\n========================================================\n";
    std::cout << "         ПІДСУМКОВА КАРТКА ОЦІНЮВАННЯ ПОЇЗДКИ           \n";
    std::cout << "========================================================\n";
    std::cout << " Водій:               " << input.driverName << " ("
              << input.driverId << ")\n";
    std::cout << " Оцінка:              " << input.tripStars << " / 5 зірок\n";

    std::cout << " Відзнаки:            ";
    for (size_t i = 0; i < input.compliments.size(); ++i) {
        std::cout << complimentToString(input.compliments[i]);
        if (i + 1 < input.compliments.size()) {
            std::cout << ", ";
        }
    }
    std::cout << "\n";

    if (!input.comment.empty()) {
        std::cout << " Відгук:              \"" << input.comment << "\"\n";
    }

    std::cout << "--------------------------------------------------------\n";
    std::cout << std::fixed << std::setprecision(3);
    std::cout << " Оновлений рейтинг:   " << result.newRating << " ("
              << (result.ratingDelta >= 0 ? "+" : "") << result.ratingDelta
              << ")\n";
    std::cout << " Рівень:              " << result.tierMeta.name << "\n";
    std::cout << " Бейдж:               [" << result.tierMeta.badge << "]\n";

    if (result.tierMeta.isWarningIssued) {
        std::cout
            << "\n [УВАГА!] Рейтинг нижче 4.50 — надіслано попередження!\n";
    }
    std::cout << "========================================================\n";
}

int main()
{
    RatingInput input;
    if (!inputRatingData(input)) {
        std::cerr << "\n[ПОМИЛКА] Некоректні вхідні дані.\n";
        return 1;
    }

    RatingResult result = processRatingData(input);
    printRatingReceipt(input, result);
    return 0;
}
