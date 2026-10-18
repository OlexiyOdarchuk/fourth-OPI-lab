/**
 * Модуль оцінювання поїздки та розрахунку рейтингу водія.
 * Використовується для збору фідбеку пасажирів, кумулятивного
 * перерахунку зваженого рейтингу та аналізу кваліфікаційного статусу водія.
 */

#pragma once

#include <cstdint>
#include <string>
#include <vector>

// Категорії компліментів від пасажира
enum class Compliment {
    Cleanliness,        //< Чисте та охайне авто
    Politeness,         //< Ввічливий та уважний водій
    ComfortableDriving, //< Плавне та безпечне водіння
    Speed,              //< Швидке прибуття та оптимальний маршрут
    NoComments          //< Без особливих приміток / стандартна поїздка
};

// Кваліфікаційний рівень (сервісний тир) водія
enum class DriverTier {
    Platinum, // Платиновий водій (найвищий рейтинг)
    Gold,     // Золотий водій
    Standard, // Стандартний водій
    Risk      // У зоні ризику (потребує переатестації)
};

// Метадані про сервісний рівень водія
struct TierMeta {
    std::string name;  // Текстова назва рівня (наприклад, "Platinum Tier")
    std::string badge; // Опис статусного бейджа
    double dispatchBonusPercent; // Відсоток пріоритету в розподілі замовлень
    bool isWarningIssued;        // Прапорець критичного зниження рейтингу нижче
                                 // порогу
};

// Вхідні дані для збору фідбеку про поїздку
struct RatingInput {
    std::string driverId;       // Унікальний ID водія (наприклад, DRV-1042)
    std::string driverName;     // Повне ім'я водія
    double currentRating = 5.0; // Поточний рейтинг водія (від 1.0 до 5.0)
    int totalPreviousTrips = 0; // Кількість вже оцінених поїздок до цього
    int tripStars = 5;          // Оцінка нової поїздки (від 1 до 5 зірок)
    std::vector<Compliment>
        compliments;     // Список обраних компліментів (множинний вибір)
    std::string comment; // Текстовий відгук або побажання від клієнта
};

// Результат перерахунку рейтингу та аналізу фідбеку
struct RatingResult {
    double oldRating = 0.0;   // Попередній рейтинг водія
    double newRating = 0.0;   // Оновлений кумулятивний рейтинг
    double ratingDelta = 0.0; // Зміна рейтингу (+/-)
    int updatedTripCount = 0; // Загальна кількість оцінених поїздок (+1)
    DriverTier tier =
        DriverTier::Standard; // Призначений кваліфікаційний рівень
    TierMeta tierMeta;        // Детальні метадані рівня (назва, бейдж, бонуси)
};

/**
 * Зчитує вхідні дані про поїздку та оцінку з консолі.
 * input Структура для заповнення зчитаними даними.
 * Повертає true, якщо дані введено коректно; false при помилці введення.
 */
bool inputRatingData(RatingInput &input);

/**
 *  Виконує обробку оцінки: перераховує рейтинг та визначає статус водія.
 *  input - Вхідні дані оцінювання.
 *  Повертає сформовану структуру RatingResult із підсумковими розрахунками.
 */
RatingResult processRatingData(const RatingInput &input);

/**
 * Форматує та виводить підсумкову картку оцінювання (чек) у консоль.
 * input - Вхідні дані оцінювання.
 * result - Розраховані результати.
 */
void printRatingReceipt(const RatingInput &input, const RatingResult &result);
