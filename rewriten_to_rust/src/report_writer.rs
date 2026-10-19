use std::{
    fmt,
    io::{self, Write},
    str::FromStr,
};

use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportType {
    All = 1,
    Rides,
    Revenue,
    Drivers,
}

impl ReportType {
    fn needs_rides(&self) -> bool {
        matches!(self, ReportType::All | ReportType::Drivers)
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

pub struct ReportParams {
    report_type: ReportType,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

pub struct ReportData {
    ride_count: usize,
    revenue: f64,
    total_drivers: i64,
    total_hours: f64,
    busy_hours: f64,
}

pub struct Report {
    ride_count: usize,
    revenue: f64,
    total_drivers: i64,
    drivers_workload: Option<f64>,
}

fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    if io::stdin().read_line(&mut input).ok()? == 0 {
        return None;
    }
    Some(input.trim().to_string())
}

fn read_number<T>(prompt: &str, min: T) -> Option<T>
where
    T: FromStr + PartialOrd + fmt::Display + Copy,
{
    loop {
        let line = read_line(prompt)?;
        if let Ok(val) = line.parse::<T>()
            && val >= min
        {
            return Some(val);
        }
        println!("Помилка: введіть число не менше {min}");
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

pub fn read_yes_no(prompt: &str) -> Option<bool> {
    loop {
        let input = read_line(prompt)?;
        match input.to_lowercase().as_str() {
            "y" => return Some(true),
            "n" => return Some(false),
            _ => println!("Введіть y або n"),
        }
    }
}

pub fn get_report_params() -> Option<ReportParams> {
    println!(
        "\nОберіть тип звіту:\n1. Усі показники\n2. Замовлення\n3. Виручка\n4. Завантаженість водіїв"
    );

    let choice: i64 = loop {
        let num = read_number("Ваш вибір (1-4)", 1)?;
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
        println!("Дата завершення не можу бути раніше дати початку");
    };

    Some(ReportParams {
        report_type,
        start_date,
        end_date,
    })
}

pub fn get_report_data(params: &ReportParams) -> Option<Option<ReportData>> {
    println!("Вводьте дані за обраний період");

    let has_data = read_yes_no("Є дані для обраного звіту? (y/n): ")?;
    if !has_data {
        return Some(None);
    }

    let mut ride_count = 0;
    let mut revenue = 0.0;
    let mut total_drivers = 0;
    let mut total_hours = 0.0;
    let mut busy_hours = 0.0;

    if params.report_type.needs_rides() {
        ride_count = read_number("Кількість виконаних замовлень: ", 0)?;
    }

    if params.report_type.needs_revenue() {
        revenue = read_number("Сумарна виручка (грн): ", 0.0)?;
    }

    if params.report_type.needs_drivers() {
        total_drivers = read_number("Кількість водіїв, які працювали за період: ", 0)?;

        if total_drivers > 0 {
            total_hours = read_number("Сумарний час усії водіїв на лінії (год): ", 0.0)?;

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

pub fn generate_report(data: &ReportData) -> Report {
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

pub fn print_report(params: &ReportParams, report: Option<(&Report, &ReportData)>) {
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
                println!("Завантаженість водіїв: {:.1}", workload);
            }
        } else {
            println!("Завантаженість не визначено: час на лінії дорівнює 0");
        }
    }
}
