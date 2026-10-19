pub mod report_writer;

use report_writer::{
    generate_report, get_report_data, get_report_params, print_report, read_yes_no,
};

fn main() {
    println!("Авторизація успішна");

    loop {
        let params = match get_report_params() {
            Some(p) => p,
            None => break,
        };

        let data = match get_report_data(&params) {
            Some(d) => d,
            None => break,
        };

        if let Some(ref report_data) = data {
            let report = generate_report(report_data);
            print_report(&params, Some((&report, report_data)));
        } else {
            print_report(&params, None);
        }

        let repeat = match read_yes_no("\nСформувати інший звіт? (y/n): ") {
            Some(val) => val,
            None => break,
        };

        if !repeat {
            break;
        }
    }

    println!("\nВведення завершено.");
}
