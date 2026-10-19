mod fare_calculation;
mod input;
mod report_writer;
mod trip_rate;

use input::read_line;

fn main() {
    loop {
        println!("\n=================================");
        println!("      СИСТЕМА TAXI MANAGEMENT    ");
        println!("=================================");
        println!("1. Оцінювання поїздки");
        println!("2. Генерація звітів");
        println!("3. Обрахунок вартості поїздки");
        println!("0. Вихід");
        println!("---------------------------------");

        let Some(line) = read_line("Оберіть дію: ") else {
            break;
        };

        match line.parse::<u32>() {
            Ok(1) => {
                println!("\n--- [ МОДУЛЬ: ОЦІНКА ПОЇЗДКИ ] ---");
                trip_rate::run();
            }
            Ok(2) => {
                println!("\n--- [ МОДУЛЬ: ЗВІТИ ] ---");
                report_writer::run();
            }
            Ok(3) => {
                println!("\n--- [ МОДУЛЬ: ОБРАХУНОК ВАРТОСТІ ПОЇЗДКИ ] ---");
                fare_calculation::run();
            }
            Ok(0) => break,
            Ok(_) => println!("\n[ПОМИЛКА] Невідома опція. Спробуйте ще раз."),
            Err(_) => println!("\n[ПОМИЛКА] Введіть числову опцію."),
        }
    }

    println!("\nРоботу завершено.");
}
