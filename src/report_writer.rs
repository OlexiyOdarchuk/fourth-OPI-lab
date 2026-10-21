use std::{collections::HashSet, fmt, str::FromStr};

use chrono::NaiveDate;

use crate::input::read_line;
use crate::order_flow::CompletedRide;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReportType {
    All = 1,
    Rides,
    Revenue,
    Drivers,
}

impl ReportType {
    fn needs_rides(&self) -> bool {
        matches!(self, ReportType::All | ReportType::Rides)
    }
    fn needs_revenue(&self) -> bool {
        matches!(self, ReportType::All | ReportType::Revenue)
    }
    fn needs_drivers(&self) -> bool {
        matches!(self, ReportType::All | ReportType::Drivers)
    }
}

impl TryFrom<i64> for ReportType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ReportType::All),
            2 => Ok(ReportType::Rides),
            3 => Ok(ReportType::Revenue),
            4 => Ok(ReportType::Drivers),
            _ => Err(()),
        }
    }
}

struct ReportParams {
    report_type: ReportType,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

struct ReportData {
    ride_count: usize,
    revenue: f64,
    total_drivers: i64,
    total_hours: f64,
    busy_hours: f64,
}

struct Report {
    ride_count: usize,
    revenue: f64,
    total_drivers: i64,
    drivers_workload: Option<f64>,
}

fn read_number<T>(prompt: &str, min: T) -> Option<T>
where
    T: FromStr + PartialOrd + fmt::Display + Copy,
{
    loop {
        let line = read_line(prompt)?;
        if let Ok(val) = line.parse::<T>()
            && val >= min
            && line.parse::<f64>().is_ok_and(f64::is_finite)
        {
            return Some(val);
        }
        println!("Помилка: введіть число не менше {min}.");
    }
}

fn read_date(prompt: &str) -> Option<NaiveDate> {
    loop {
        let input = read_line(prompt)?;
        if let Ok(date) = NaiveDate::parse_from_str(&input, "%d-%m-%Y") {
            return Some(date);
        }
        println!("Помилка: введіть існуючу дату у форматі ДД-ММ-РРРР");
    }
}

fn read_yes_no(prompt: &str) -> Option<bool> {
    loop {
        let input = read_line(prompt)?;
        match input.to_lowercase().as_str() {
            "y" => return Some(true),
            "n" => return Some(false),
            _ => println!("Введіть y або n"),
        }
    }
}

fn get_report_params() -> Option<ReportParams> {
    println!(
        "\nОберіть тип звіту:\n1. Усі показники\n2. Замовлення\n3. Виручка\n4. Завантаженість водіїв"
    );

    let choice: i64 = loop {
        let num = read_number("Ваш вибір (1-4): ", 1)?;
        if (1..=4).contains(&num) {
            break num;
        }
        println!("Оберіть пункт від 1 до 4");
    };

    let report_type = ReportType::try_from(choice).ok()?;
    let start_date = read_date("Дата початку(ДД-ММ-РРРР): ")?;

    let end_date = loop {
        let end = read_date("Дата завершення(ДД-ММ-РРРР): ")?;
        if end >= start_date {
            break end;
        }
        println!("Дата завершення не може бути раніше дати початку");
    };

    Some(ReportParams {
        report_type,
        start_date,
        end_date,
    })
}

fn get_report_data(params: &ReportParams, rides: &[CompletedRide]) -> Option<Option<ReportData>> {
    println!("Вводьте дані за обраний період");

    // Замовлення, оформлені в поточному сеансі, можна взяти без ручного вводу
    let from_session = !rides.is_empty()
        && read_yes_no(&format!(
            "Використати замовлення поточного сеансу ({} шт.)? (y/n): ",
            rides.len()
        ))?;

    if !from_session {
        let has_data = read_yes_no("Є дані для обраного звіту? (y/n): ")?;
        if !has_data {
            return Some(None);
        }
    }

    let mut ride_count = 0;
    let mut revenue = 0.0;
    let mut total_drivers = 0;
    let mut total_hours = 0.0;
    let mut busy_hours = 0.0;

    if params.report_type.needs_rides() {
        ride_count = if from_session {
            rides.len()
        } else {
            read_number("Кількість виконаних замовлень: ", 0)?
        };
    }

    if params.report_type.needs_revenue() {
        revenue = if from_session {
            rides.iter().map(|ride| ride.fare).sum::<i64>() as f64 / 100.0
        } else {
            read_number("Сумарна виручка (грн): ", 0.0)?
        };
    }

    if params.report_type.needs_drivers() {
        total_drivers = if from_session {
            let drivers: HashSet<&str> = rides.iter().map(|ride| ride.driver_id.as_str()).collect();
            drivers.len() as i64
        } else {
            read_number("Кількість водіїв, які працювали за період: ", 0)?
        };

        if total_drivers > 0 {
            total_hours = read_number("Сумарний час усіх водіїв на лінії (год): ", 0.0)?;

            busy_hours = loop {
                let hours = read_number("Із них сумарний час виконання замовлень (год): ", 0.0)?;
                if hours <= total_hours {
                    break hours;
                }
                println!("Час замовлень не може перевищувати час на лінії");
            };
        }
    }

    Some(Some(ReportData {
        ride_count,
        revenue,
        total_drivers,
        total_hours,
        busy_hours,
    }))
}

fn generate_report(data: &ReportData) -> Report {
    let drivers_workload = if data.total_hours > 0.0 {
        Some((data.busy_hours / data.total_hours) * 100.0)
    } else {
        None
    };

    Report {
        ride_count: data.ride_count,
        revenue: data.revenue,
        total_drivers: data.total_drivers,
        drivers_workload,
    }
}

fn print_report(params: &ReportParams, report: Option<(&Report, &ReportData)>) {
    let (report, data) = match report {
        Some(val) => val,
        None => {
            println!(
                "\nЗа обраний період дані відсутні.\nМожете сформувати звіт з іншими параметрами."
            );
            return;
        }
    };

    println!(
        "\nЗвіт за період {} - {}",
        params.start_date.format("%d-%m-%Y"),
        params.end_date.format("%d-%m-%Y")
    );

    if params.report_type.needs_rides() {
        println!("Виконано замовлень: {}", report.ride_count);
    }

    if params.report_type.needs_revenue() {
        println!("Виручка {:.2} грн", report.revenue);
    }

    if params.report_type.needs_drivers() {
        println!("Водіїв за період: {}", report.total_drivers);

        if data.total_hours > 0.0 {
            if let Some(workload) = report.drivers_workload {
                println!("Завантаженість водіїв: {:.1} %", workload);
            }
        } else {
            println!("Завантаженість не визначено: час на лінії дорівнює 0");
        }
    }
}

pub fn run(rides: &[CompletedRide]) {
    while let Some(params) = get_report_params() {
        let Some(data) = get_report_data(&params, rides) else {
            break;
        };

        if let Some(report_data) = &data {
            let report = generate_report(report_data);
            print_report(&params, Some((&report, report_data)));
        } else {
            print_report(&params, None);
        }

        if read_yes_no("\nСформувати інший звіт? (y/n): ") != Some(true) {
            break;
        }
    }

    println!("\nВведення завершено.");
}
