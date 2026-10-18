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

bool getReportParams(ReportParams& params);

bool readReportData(const ReportParams& params, ReportData& data);

Report generateReport(const ReportData& data);

void printReport(const ReportParams& params, const Report& report, const ReportData& data);
