mod driver_matching;
mod fare_calculation;
mod input;
mod order_flow;
mod report_writer;
mod trip_rate;

use input::read_line;

fn main() {
    let mut rides = Vec::new();

    loop {
        println!("\n=================================");
        println!("      СИСТЕМА TAXI MANAGEMENT    ");
        println!("=================================");
        println!("1. Оцінювання поїздки");
        println!("2. Генерація звітів");
        println!("3. Обрахунок вартості поїздки");
        println!("4. Підбір водія");
        println!("5. Оформлення замовлення (повний цикл)");
        println!("0. Вихід");
        println!("---------------------------------");

        let Some(line) = read_line("Оберіть дію: ") else {
            break;
        };

        match line.parse::<u8>() {
            Ok(1) => {
                println!("\n--- [ МОДУЛЬ: ОЦІНКА ПОЇЗДКИ ] ---");
                trip_rate::run();
            }
            Ok(2) => {
                println!("\n--- [ МОДУЛЬ: ЗВІТИ ] ---");
                report_writer::run(&rides);
            }
            Ok(3) => {
                println!("\n--- [ МОДУЛЬ: ОБРАХУНОК ВАРТОСТІ ПОЇЗДКИ ] ---");
                fare_calculation::run();
            }
            Ok(4) => {
                println!("\n--- [ МОДУЛЬ: ПІДБІР ВОДІЯ ] ---");
                driver_matching::run();
            }
            Ok(5) => {
                println!("\n--- [ МОДУЛЬ: ОФОРМЛЕННЯ ЗАМОВЛЕННЯ ] ---");
                order_flow::run(&mut rides);
            }
            Ok(0) => break,
            Ok(_) => eprintln!("\n[ПОМИЛКА] Невідома опція. Спробуйте ще раз."),
            Err(_) => eprintln!("\n[ПОМИЛКА] Введіть числову опцію."),
        }
    }

    println!("\nРоботу завершено.");
}
