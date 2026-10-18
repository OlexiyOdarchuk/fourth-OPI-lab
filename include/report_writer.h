#pragma once

#include <iomanip>
#include <iostream>
#include <limits>
#include <string>

// Типи звітів
enum class ReportType { All = 1, Rides, Revenue, Drivers };

// Параметри звіту
struct ReportParams {
    ReportType type = ReportType::All;
    std::string startDate;
    std::string endDate;
};

// Структура даних звіту
struct Report {
    int rideCount = 0;
    double revenue = 0.0;
    int totalDrivers = 0;
    double driverWorkload = 0.0;
};

// Очищення буфера вводу при помилках
void clearInput() {
    std::cin.clear();
    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
}

// Отримання параметрів від користувача
bool getReportParams(ReportParams& params) {
    std::cout << "Оберіть тип звіту:\n";
    std::cout << "1. Усі показники\n";
    std::cout << "2. Замовлення\n";
    std::cout << "3. Виручка\n";
    std::cout << "4. Завантаженість водіїв\n";
    std::cout << "Ваш вибір (1-4): ";

    int choice = 0;
    if (!(std::cin >> choice) || choice < 1 || choice > 4) {
        clearInput();
        return false;
    }
    params.type = static_cast<ReportType>(choice);

    std::cout << "Введіть дату початку (ДД-ММ-РРРР): ";
    std::cin >> params.startDate;
    std::cout << "Введіть дату завершення (ДД-ММ-РРРР): ";
    std::cin >> params.endDate;

    return true;
}

// Введення даних для звіту
bool generateReport(const ReportParams& params, Report& report) {
    std::cout << "\nВведіть кількість виконаних замовлень: ";
    if (!(std::cin >> report.rideCount) || report.rideCount < 0) return false;

    std::cout << "Введіть сумарну виручку (грн): ";
    if (!(std::cin >> report.revenue) || report.revenue < 0.0) return false;

    std::cout << "Введіть кількість водіїв на лінії: ";
    if (!(std::cin >> report.totalDrivers) || report.totalDrivers < 0) return false;

    if (report.totalDrivers > 0) {
        std::cout << "Введіть завантаженість водіїв (%): ";
        if (!(std::cin >> report.driverWorkload) || report.driverWorkload < 0.0 || report.driverWorkload > 100.0) {
            return false;
        }
    } else {
        report.driverWorkload = 0.0;
    }

    return true;
}

// Вивід звіту
void printReport(const ReportParams& params, const Report& report) {
    // А1. За обраний період немає даних
    if (report.rideCount == 0 && report.revenue == 0.0 && report.totalDrivers == 0) {
        std::cout << "\nЗа обраний період дані відсутні.\n";
        return;
    }

    std::cout << "\nЗвіт за період " << params.startDate << " - " << params.endDate << ":\n";
    std::cout << std::fixed << std::setprecision(2);

    if (params.type == ReportType::All || params.type == ReportType::Rides) {
        std::cout << "Замовлень: " << report.rideCount << "\n";
    }
    if (params.type == ReportType::All || params.type == ReportType::Revenue) {
        std::cout << "Виручка: " << report.revenue << " грн\n";
    }
    if (params.type == ReportType::All || params.type == ReportType::Drivers) {
        std::cout << "Водіїв на лінії: " << report.totalDrivers << "\n";
        std::cout << "Завантаженість водіїв: " << std::setprecision(1) << report.driverWorkload << "%\n";
    }
}