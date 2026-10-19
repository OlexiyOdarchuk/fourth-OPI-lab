pub mod report_writer;

use report_writer::{
    generate_report, get_report_data, get_report_params, print_report, read_yes_no,
};

fn main() {
    println!("Авторизація успішна");

    while let Some(params) = get_report_params() {
        let Some(data) = get_report_data(&params) else {
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
