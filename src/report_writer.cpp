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

bool needsRides(ReportType type) {
    return type == ReportType::All || type == ReportType::Rides;
}

bool needsRevenue(ReportType type) {
    return type == ReportType::All || type == ReportType::Revenue;
}

bool needsDrivers(ReportType type) {
    return type == ReportType::All || type == ReportType::Drivers;
}

template <typename T>
bool readNumber(const std::string& prompt, T& value, T min) {
    std::string line;

    while (true) {
        std::cout << prompt;

        if (!std::getline(std::cin, line)) {
            return false;
        }

        std::istringstream input(line);
        T number{};
        char extra;

        if ((input >> number) &&
            !(input >> extra) &&
            std::isfinite(static_cast<double>(number)) &&
            number >= min) {
            value = number;
            return true;
        }

        std::cout << "Помилка: введіть число не менше "
                  << min << ".\n";
    }
}

bool isLeapYear(int year) {
    return year % 400 == 0 || (year % 4 == 0 && year % 100 != 0);
}

bool parseDate(const std::string& text, Date& date) {
    if (text.size() != 10 || text[2] != '-' || text[5] != '-') {
        return false;
    }

    for (std::size_t i = 0; i < text.size(); ++i) {
        if (i == 2 || i == 5) {
            continue;
        }

        if (text[i] < '0' || text[i] > '9') {
            return false;
        }
    }

    Date result;
    result.day = std::stoi(text.substr(0, 2));
    result.month = std::stoi(text.substr(3, 2));
    result.year = std::stoi(text.substr(6, 4));

    if (result.year < 1 || result.month < 1 || result.month > 12) {
        return false;
    }

    const int daysInMonth[] = {
        31, 28, 31, 30, 31, 30,
        31, 31, 30, 31, 30, 31
    };

    int maxDay = daysInMonth[result.month - 1];

    if (result.month == 2 && isLeapYear(result.year)) {
        maxDay = 29;
    }

    if (result.day < 1 || result.day > maxDay) {
        return false;
    }

    date = result;
    return true;
}

int dateKey(const Date& date) {
    return date.year * 10000 + date.month * 100 + date.day;
}

std::string formatDate(const Date& date) {
    std::ostringstream output;

    output << std::setfill('0')
           << std::setw(2) << date.day << '-'
           << std::setw(2) << date.month << '-'
           << std::setw(4) << date.year;

    return output.str();
}

bool readDate(const std::string& prompt, Date& date) {
    std::string line;

    while (true) {
        std::cout << prompt;

        if (!std::getline(std::cin, line)) {
            return false;
        }

        if (parseDate(line, date)) {
            return true;
        }

        std::cout << "Помилка: введіть існуючу дату у форматі ДД-ММ-РРРР.\n";
    }
}

bool readYesNo(const std::string& prompt, bool& answer) {
    std::string input;

    while (true) {
        std::cout << prompt;

        if (!std::getline(std::cin, input)) {
            return false;
        }

        if (input == "y" || input == "Y") {
            answer = true;
            return true;
        }

        if (input == "n" || input == "N") {
            answer = false;
            return true;
        }

        std::cout << "Введіть y або n.\n";
    }
}

// Введення параметрів звіту
bool getReportParams(ReportParams& params) {
    std::cout << "\nОберіть тип звіту:\n"
              << "1. Усі показники\n"
              << "2. Замовлення\n"
              << "3. Виручка\n"
              << "4. Завантаженість водіїв\n";

    int choice = 0;

    while (true) {
    if (!readNumber("Ваш вибір (1-4): ", choice, 1)) {
        return false;
    }

    if (choice <= 4) {
        break;
    }

    std::cout << "Оберіть пункт від 1 до 4.\n";
    }

    params.type = static_cast<ReportType>(choice);

    if (!readDate("Дата початку (ДД-ММ-РРРР): ", params.startDate)) {
        return false;
    }

    while (true) {
        if (!readDate("Дата завершення (ДД-ММ-РРРР): ", params.endDate)) {
            return false;
        }

        if (dateKey(params.endDate) >= dateKey(params.startDate)) {
            return true;
        }

        std::cout << "Дата завершення не може бути раніше дати початку.\n";
    }
}

// Введення вихідних даних відокремлене від обчислень
bool readReportData(const ReportParams& params, ReportData& data) {
    data = ReportData{};

    std::cout << "\nВводьте дані лише за обраний період.\n";

    if (!readYesNo(
            "Є дані для обраного звіту? (y/n): ",
            data.hasData)) {
        return false;
    }

    if (!data.hasData) {
        return true;
    }

    if (needsRides(params.type)) {
        if (!readNumber(
                "Кількість виконаних замовлень: ",
                data.rideCount, 0)) {
            return false;
        }
    }

    if (needsRevenue(params.type)) {
        if (!readNumber(
                "Сумарна виручка (грн, десятковий роздільник - крапка): ",
                data.revenue, 0.0)) {
            return false;
        }
    }

    if (needsDrivers(params.type)) {
        if (!readNumber(
                "Кількість водіїв, які працювали за період: ",
                data.totalDrivers, 0)) {
            return false;
        }

        if (data.totalDrivers > 0) {
            if (!readNumber(
                    "Сумарний час усіх водіїв на лінії (год): ",
                    data.totalHours, 0.0)) {
                return false;
            }

            while (true) {
                if (!readNumber(
                        "Із них сумарний час виконання замовлень (год): ",
                        data.busyHours, 0.0)) {
                    return false;
                }

                if (data.busyHours <= data.totalHours) {
                    break;
                }

                std::cout << "Час виконання замовлень не може "
                          << "перевищувати час на лінії.\n";
            }
        }
    }

    return true;
}

// Обробка даних без введення та виведення
Report generateReport(const ReportData& data) {
    Report report;

    report.hasData = data.hasData;

    if (!data.hasData) {
        return report;
    }

    report.rideCount = data.rideCount;
    report.revenue = data.revenue;
    report.totalDrivers = data.totalDrivers;

    if (data.totalHours > 0.0) {
        report.driverWorkload =
            data.busyHours / data.totalHours * 100.0;
    }

    return report;
}

// Виведення результату
void printReport(const ReportParams& params, const Report& report,
                 const ReportData& data) {
    if (!report.hasData) {
        std::cout << "\nЗа обраний період дані відсутні.\n"
                  << "Можете сформувати звіт з іншими параметрами.\n";
        return;
    }

    std::cout << "\nЗвіт за період "
              << formatDate(params.startDate) << " - "
              << formatDate(params.endDate) << ":\n";

    if (needsRides(params.type)) {
        std::cout << "Виконано замовлень: " << report.rideCount << '\n';
    }

    if (needsRevenue(params.type)) {
        std::cout << "Виручка: "
                  << std::fixed << std::setprecision(2)
                  << report.revenue << " грн\n";
    }

    if (needsDrivers(params.type)) {
        std::cout << "Водіїв за період: " << report.totalDrivers << '\n';

        if (data.totalHours > 0.0) {
            std::cout << "Завантаженість водіїв: "
                      << std::fixed << std::setprecision(1)
                      << report.driverWorkload << "%\n";
        } else {
            std::cout << "Завантаженість не визначено: "
                      << "час на лінії дорівнює нулю.\n";
        }
    }
}

int main() {
    std::cout << "Авторизація успішна.\n";

    while (true) {
        ReportParams params;
        ReportData data;

        if (!getReportParams(params) ||
            !readReportData(params, data)) {
            std::cout << "\nВведення завершено. Звіт не сформовано.\n";
            break;
        }

        const Report report = generateReport(data);
        printReport(params, report, data);

        bool repeat = false;

        if (!readYesNo("\nСформувати інший звіт? (y/n): ", repeat) ||
            !repeat) {
            break;
        }
    }
}