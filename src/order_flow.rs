use crate::driver_matching::{self, CarClass};
use crate::fare_calculation::{self, Tariff, TripOrder};
use crate::input::read_line;
use crate::trip_rate;

// Завершене замовлення поточного сеансу роботи
pub(crate) struct CompletedRide {
    pub(crate) driver_id: String,
    pub(crate) fare: i64, // Сплачена сума в копійках
}

// Тариф відповідає класу авто, який обрав пасажир
fn tariff_for(class: CarClass) -> Tariff {
    match class {
        CarClass::Economy => Tariff::Economy,
        CarClass::Standard => Tariff::Standard,
        CarClass::Comfort => Tariff::Comfort,
        CarClass::Business => Tariff::Business,
    }
}

pub fn run(rides: &mut Vec<CompletedRide>) {
    println!("==================================================");
    println!("    АС «ВИКЛИК ТАКСІ» - ОФОРМЛЕННЯ ЗАМОВЛЕННЯ     ");
    println!("==================================================\n");

    println!("--- Крок 1 з 3: підбір водія ---");
    let Some(order) = driver_matching::read_order() else {
        eprintln!("\n[ПОМИЛКА] Некоректні вхідні дані замовлення.");
        return;
    };

    println!("\n--- Пункт призначення ---");
    let Some(x) = driver_matching::read_coordinate("  X: ") else {
        return;
    };
    let Some(y) = driver_matching::read_coordinate("  Y: ") else {
        return;
    };
    let distance_km = order.pos.distance(&driver_matching::Position { x, y });
    if distance_km <= 0.0 {
        println!("[ПОМИЛКА] Пункт призначення збігається з місцем посадки.");
        return;
    }

    let drivers = driver_matching::candidate_drivers();
    let result = driver_matching::match_driver(&drivers, &order, driver_matching::SEARCH_RADIUS_KM);
    driver_matching::print_match_result(&order, result);
    let Some((driver, _, _)) = result else {
        println!("Замовлення скасовано: вільного водія не знайдено.");
        return;
    };

    println!("--- Крок 2 з 3: розрахунок вартості ---");
    let duration_minutes = driver_matching::estimated_time(distance_km);
    println!("Маршрут: {distance_km:.2} км, орієнтовний час у дорозі {duration_minutes:.0} хв");
    let Some(passenger_phone) = read_line("Введіть номер телефону (+380...): ")
    else {
        return;
    };
    let Some(request) = fare_calculation::read_trip_request(
        tariff_for(order.desired_class),
        distance_km,
        duration_minutes,
    ) else {
        return;
    };
    let trip = TripOrder {
        order_id: order.order_id,
        passenger_phone,
        request,
    };
    let Some(fare) = fare_calculation::settle(&trip) else {
        return;
    };
    rides.push(CompletedRide {
        driver_id: driver.id.clone(),
        fare,
    });

    println!("--- Крок 3 з 3: оцінювання поїздки ---");
    trip_rate::rate_driver(&driver.id, &driver.name, driver.rating, driver.total_trips);
}
