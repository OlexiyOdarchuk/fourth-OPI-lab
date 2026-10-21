use std::fmt;

use crate::input::read_line;

// Категорії компліментів від пасажира
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Compliment {
    CleanCar,      //< Чисте та охайне авто
    PoliteDriver,  //< Ввічливий та уважний водій
    SmoothDriving, //< Плавне та безпечне водіння
    FastArrival,   //< Швидке прибуття та оптимальний маршрут
    Standard,      //< Без особливих приміток / стандартна поїздка
}

impl fmt::Display for Compliment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::CleanCar => "Чисте та доглянуте авто",
            Self::PoliteDriver => "Ввічливість і клієнтоорієнтованість",
            Self::SmoothDriving => "Безпечне і комфортне керування",
            Self::FastArrival => "Пунктуальність та оптимальний маршрут",
            Self::Standard => "Базовий рівень обслуговування",
        };
        write!(f, "{text}")
    }
}

impl TryFrom<u32> for Compliment {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::CleanCar),
            2 => Ok(Self::PoliteDriver),
            3 => Ok(Self::SmoothDriving),
            4 => Ok(Self::FastArrival),
            5 => Ok(Self::Standard),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DriverTier {
    Newcomer, // Новий водій (замало оцінок для визначення рівня)
    Platinum, // Платиновий водій (найвищий рейтинг)
    Gold,     // Золотий водій
    Standard, // Стандартний водій
    Risk,     // У зоні ризику (потребує переатестації)
}

// Метадані про сервісний рівень водія
struct TierMeta {
    name: &'static str,          // Текстова назва рівня (наприклад, "Platinum Tier")
    badge_title: &'static str,   // Опис статусного бейджа
    dispatch_bonus_percent: f64, // Відсоток пріоритету в розподілі замовлень
    is_warning_issued: bool,     // Прапорець критичного зниження рейтингу нижче порогу
}

// Мінімальна кількість оцінок, після якої водію присвоюється рівень
const MIN_TRIPS_FOR_TIER: u32 = 10;

const TRIP_COUNT_OVERFLOW: &str = "Кількість попередніх поїздок занадто висока.";

fn next_trip_count(previous: u32) -> Result<u32, &'static str> {
    previous.checked_add(1).ok_or(TRIP_COUNT_OVERFLOW)
}

fn read_rating_line(prompt: &str) -> Result<String, &'static str> {
    read_line(prompt).ok_or("Ввід перервано або сталася помилка читання. Оцінку не збережено.")
}

impl DriverTier {
    fn from_rating(rating: f64, trip_count: u32) -> Self {
        if trip_count < MIN_TRIPS_FOR_TIER {
            return Self::Newcomer;
        }
        match rating {
            r if r >= 4.90 => Self::Platinum,
            r if r >= 4.75 => Self::Gold,
            r if r >= 4.50 => Self::Standard,
            _ => Self::Risk,
        }
    }

    fn meta(self) -> TierMeta {
        match self {
            Self::Newcomer => TierMeta {
                name: "Новий водій (Newcomer)",
                badge_title: "Рівень визначається після 10 оцінок",
                dispatch_bonus_percent: 0.0,
                is_warning_issued: false,
            },
            Self::Platinum => TierMeta {
                name: "Платиновий водій (Platinum Tier)",
                badge_title: "Вищий пілотаж сервісу",
                dispatch_bonus_percent: 15.0,
                is_warning_issued: false,
            },
            Self::Gold => TierMeta {
                name: "Золотий водій (Gold Tier)",
                badge_title: "Надійний перевізник",
                dispatch_bonus_percent: 8.0,
                is_warning_issued: false,
            },
            Self::Standard => TierMeta {
                name: "Стандартний водій (Standard Tier)",
                badge_title: "Відповідає базовим вимогам",
                dispatch_bonus_percent: 0.0,
                is_warning_issued: false,
            },
            Self::Risk => TierMeta {
                name: "У зоні ризику (Risk Tier)",
                badge_title: "Потребує переатестації",
                dispatch_bonus_percent: -20.0,
                is_warning_issued: true,
            },
        }
    }
}

impl fmt::Display for DriverTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.meta().name)
    }
}

// Вхідні дані для збору фідбеку про поїздку
#[derive(Debug)]
struct RatingInput {
    driver_id: String,            // Унікальний ID водія (наприклад, DRV-1042)
    driver_name: String,          // Повне ім'я водія
    current_rating: f64,          // Поточний рейтинг водія (від 1.0 до 5.0)
    total_previous_trips: u32,    // Кількість вже оцінених поїздок до цього
    trip_stars: u32,              // Оцінка нової поїздки (від 1 до 5 зірок)
    compliments: Vec<Compliment>, // Список обраних компліментів (множинний вибір)
    comment: String,              // Текстовий відгук або побажання від клієнта
}

impl RatingInput {
    fn read_from_stdin() -> Result<Self, &'static str> {
        let driver_id = read_rating_line("Введіть ідентифікатор водія (наприклад, DRV-1042): ")?;
        if driver_id.is_empty() {
            return Err("Ідентифікатор водія не може бути порожнім.");
        };
        let mut driver_name = read_rating_line("Введіть повне ім'я водія: ")?;
        if driver_name.is_empty() {
            driver_name = "Володимир Ковалюк".to_string();
        };

        let rating_str = read_rating_line("Введіть поточний рейтинг водія (від 1.00 до 5.00): ")?;
        let current_rating: f64 = rating_str
            .parse()
            .map_err(|_| "Рейтинг має бути числом від 1.00 до 5.00; для дробової частини використовуйте крапку, наприклад 4.8.")?;
        if !(1.00..=5.00).contains(&current_rating) {
            return Err("Рейтинг має бути скінченним числом у межах [1.00; 5.00].");
        };

        let trips_str =
            read_rating_line("Введіть кількість оцінених поїздок водія до цього часу: ")?;
        let total_previous_trips = trips_str.parse::<u32>().map_err(
            |_| "Кількість поїздок має бути цілим невід'ємним числом від 0 до 4294967294.",
        )?;

        Self::read_trip_feedback(driver_id, driver_name, current_rating, total_previous_trips)
    }

    // Читає оцінку, компліменти та відгук для водія з відомою історією
    fn read_trip_feedback(
        driver_id: String,
        driver_name: String,
        current_rating: f64,
        total_previous_trips: u32,
    ) -> Result<Self, &'static str> {
        next_trip_count(total_previous_trips)?;
        println!("\n--- Оцінювання щойно завершеної поїздки ---");
        let stars_str = read_rating_line("Поставте оцінку поїздці (від 1 до 5 зірок): ")?;
        let trip_stars: u32 = stars_str
            .parse()
            .map_err(|_| "Оцінка поїздки має бути цілим числом від 1 до 5.")?;
        if !(1..=5).contains(&trip_stars) {
            return Err("Оцінка поїздки має бути цілим числом від 1 до 5.");
        };

        println!(
            "\nОберіть категорії відгуку / компліменти (через пробіл або кому, наприклад '1 3'):"
        );
        println!("  1 — Чисте та охайне авто");
        println!("  2 — Ввічливий та уважний водій");
        println!("  3 — Плавне та безпечне водіння");
        println!("  4 — Швидке прибуття та оптимальний маршрут");
        println!("  5 — Стандартна поїздка (без особливих приміток)");

        let comp_line = read_rating_line("Ваш вибір (1-5, можна залишити порожнім): ")?;

        let mut compliments: Vec<Compliment> = comp_line
            .split(|ch: char| ch.is_whitespace() || ch == ',')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.parse::<u32>()
                    .ok()
                    .and_then(|val| Compliment::try_from(val).ok())
                    .ok_or("Категорії відгуку мають бути цілими числами від 1 до 5 через пробіл або кому; можна залишити поле порожнім.")
            })
            .collect::<Result<_, _>>()?;

        // Кожен комплімент враховується один раз, у порядку пунктів меню
        compliments.sort();
        compliments.dedup();

        if compliments.is_empty() {
            compliments.push(Compliment::Standard);
        }

        let comment = read_rating_line(
            "\nВведіть текстовий відгук або побажання (залиште порожнім, якщо немає): ",
        )?;

        Ok(Self {
            driver_id,
            driver_name,
            current_rating,
            total_previous_trips,
            trip_stars,
            compliments,
            comment,
        })
    }
}

// Результат перерахунку рейтингу та аналізу фідбеку
#[derive(Debug)]
struct RatingResult {
    old_rating: f64,         // Попередній рейтинг водія
    new_rating: f64,         // Оновлений кумулятивний рейтинг
    rating_delta: f64,       // Зміна рейтингу (+/-)
    updated_trip_count: u32, // Загальна кількість оцінених поїздок (+1)
    tier: DriverTier,        // Призначений кваліфікаційний рівень
}

impl RatingResult {
    fn process(input: &RatingInput) -> Result<Self, &'static str> {
        let updated_trip_count = next_trip_count(input.total_previous_trips)?;
        let old_rating = input.current_rating;
        let new_rating = if input.total_previous_trips == 0 {
            input.trip_stars as f64
        } else {
            let total_score =
                old_rating * (input.total_previous_trips as f64) + (input.trip_stars as f64);
            let raw_avg = total_score / (updated_trip_count as f64);
            (raw_avg * 1000.0).round() / 1000.0
        };

        Ok(Self {
            old_rating,
            new_rating,
            rating_delta: new_rating - old_rating,
            updated_trip_count,
            tier: DriverTier::from_rating(new_rating, updated_trip_count),
        })
    }

    fn print_summary(input: Result<RatingInput, &'static str>) {
        let (input, result) =
            match input.and_then(|input| Self::process(&input).map(|result| (input, result))) {
                Ok(value) => value,
                Err(message) => {
                    eprintln!("\n[ПОМИЛКА] {message}");
                    return;
                }
            };
        let meta = result.tier.meta();

        println!("\n========================================================");
        println!("          ПІДСУМКОВА КАРТКА ОЦІНЮВАННЯ ПОЇЗДКИ           ");
        println!("========================================================");
        println!(" Ідентифікатор водія: {}", input.driver_id);
        println!(" Водій:                {}", input.driver_name);
        println!(
            " Попередня історія:   {} поїздок (рейтинг {:.2})",
            input.total_previous_trips, result.old_rating
        );

        let stars = "*".repeat(input.trip_stars as usize);
        let spaces = " ".repeat((5 - input.trip_stars) as usize);
        println!(
            " Оцінка поїздки:      {} / 5 зірок [{stars}{spaces}]",
            input.trip_stars
        );

        println!(" Відзнаки пасажира:");
        for comp in &input.compliments {
            println!("   • {comp}");
        }

        if !input.comment.is_empty() {
            println!(" Відгук клієнта:      \"{}\"", input.comment);
        }
        println!("--------------------------------------------------------");

        let delta_sign = if result.rating_delta >= 0.0 { "+" } else { "" };
        println!(
            " Оновлений рейтинг:   {:.3} (зміна: {delta_sign}{:.3})",
            result.new_rating, result.rating_delta
        );
        println!(" Всього оцінок:        {}", result.updated_trip_count);

        println!(" Кваліфікаційний рівень: {}", result.tier);
        println!(" Статусний бейдж:      [{}]", meta.badge_title);

        let bonus_sign = if meta.dispatch_bonus_percent >= 0.0 {
            "+"
        } else {
            ""
        };
        println!(
            " Пріоритет розподілу: {bonus_sign}{:.1} %",
            meta.dispatch_bonus_percent
        );

        if meta.is_warning_issued {
            println!("\n [УВАГА!] Рейтинг водія впав нижче критичного порогу 4.50!");
            println!(" Система надіслала попередження до диспетчерського центру.");
        } else {
            println!("\n [OK] Оцінку успішно зафіксовано в базі даних сервісу.");
        }
        println!("========================================================");
        println!(" Дякуємо за допомогу у покращенні якості нашого таксі!\n");
    }
}

pub fn run() {
    println!("==================================================");
    println!("  АС «ВИКЛИК ТАКСІ» - МОДУЛЬ ОЦІНЮВАННЯ ПОЇЗДКИ   ");
    println!("==================================================\n");

    RatingResult::print_summary(RatingInput::read_from_stdin());
}

// Оцінювання поїздки з водієм, якого призначила система
pub(crate) fn rate_driver(
    driver_id: &str,
    driver_name: &str,
    current_rating: f64,
    total_previous_trips: u32,
) {
    RatingResult::print_summary(RatingInput::read_trip_feedback(
        driver_id.to_string(),
        driver_name.to_string(),
        current_rating,
        total_previous_trips,
    ));
}
