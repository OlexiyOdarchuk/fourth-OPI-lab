#include <iostream>
#include <limits>

#include "fare_calculation.h"
#include "report_writer.h"
#include "trip_rate.h"

void runFareCalculation()
{
    TripRequest req;
    if (!inputTripData(req)) {
        std::cerr << "\n[ЗБІЙ] Помилка вводу параметрів розрахунку вартості.\n";
        return;
    }

    FareCalculationResult result = processFareCalculation(req);
    printTripReceipt(req, result);
}

void runRatingModule()
{
    RatingInput input;
    if (!inputRatingData(input)) {
        std::cerr << "\n[ПОМИЛКА] Некоректні вхідні дані.\n";
        return;
    }

    RatingResult result = processRatingData(input);
    printRatingReceipt(input, result);
}

void runReportModule()
{
    while (true) {
        ReportParams params;
        ReportData data;

        if (!getReportParams(params) || !readReportData(params, data)) {
            std::cout << "\nВведення завершено. Звіт не сформовано.\n";
            break;
        }

        const Report report = generateReport(data);
        printReport(params, report, data);

        bool repeat = false;
        if (!readYesNo("\nСформувати інший звіт? (y/n): ", repeat) || !repeat) {
            break;
        }
    }
}

void clearInputStream()
{
    std::cin.clear();
    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
}

int main()
{
    while (true) {
        std::cout << "\n=================================\n";
        std::cout << "      СИСTEMA TAXI MANAGEMENT    \n";
        std::cout << "=================================\n";
        std::cout << "1. Розрахунок рейтингу та чек\n";
        std::cout << "2. Генерація звітів\n";
        std::cout << "3. Обрахунок вартості поїздки\n";
        std::cout << "0. Вихід\n";
        std::cout << "---------------------------------\n";
        std::cout << "Оберіть дію: ";

        int choice = -1;
        if (!(std::cin >> choice)) {
            clearInputStream();
            std::cout << "\n[ПОМИЛКА] Введіть числову опцію.\n";
            continue;
        }

        switch (choice) {
            case 1:
                std::cout << "\n--- [ МОДУЛЬ: ОЦІНКА ПОЇЗДКИ ] ---\n";
                runRatingModule();
                break;
            case 2:
                std::cout << "\n--- [ МОДУЛЬ: ЗВІТИ ] ---\n";
                runReportModule();
                break;
            case 3:
                std::cout
                    << "\n--- [ МОДУЛЬ: РОЗРАХУНОК ВАРТОСТІ ПОЇЗДКИ ] ---\n";
                runFareCalculation();
                break;
            case 0:
                std::cout << "\nРоботу завершено.\n";
                return 0;
            default:
                std::cout << "\n[ПОМИЛКА] Невідома опція. Спробуйте ще раз.\n";
                break;
        }
    }
}
