#pragma once

#include <cmath>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>

enum class ReportType {
    All = 1,
    Rides,
    Revenue,
    Drivers
};

struct Date {
    int day = 1;
    int month = 1;
    int year = 2000;
};

struct ReportParams {
    ReportType type = ReportType::All;
    Date startDate;
    Date endDate;
};

// Вхідні дані за обраний період.
struct ReportData {
    bool hasData = false;
    int rideCount = 0;
    double revenue = 0.0;
    int totalDrivers = 0;
    double totalHours = 0.0;
    double busyHours = 0.0;
};

struct Report {
    bool hasData = false;
    int rideCount = 0;
    double revenue = 0.0;
    int totalDrivers = 0;
    double driverWorkload = 0.0;
};

bool needsRides(ReportType type);
bool needsRevenue(ReportType type);
bool needsDrivers(ReportType type);

template <typename T>
bool readNumber(const std::string& prompt, T& value, T min);

bool isLeapYear(int year);

bool parseDate(const std::string& text, Date& date);

int dateKey(const Date& date);
std::string formatDate(const Date& date);

bool readDate(const std::string& prompt, Date& date);

bool readYesNo(const std::string& prompt, bool& answer);

// Введення параметрів звіту
bool getReportParams(ReportParams& params);

// Введення вихідних даних відокремлене від обчислень
bool readReportData(const ReportParams& params, ReportData& data);

// Обробка даних без введення та виведення
Report generateReport(const ReportData& data);

// Виведення результату
void printReport(const ReportParams& params, const Report& report, const ReportData& data);
