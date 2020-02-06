use crate::station::{Field, Station};

#[allow(dead_code)]
// Setup function to generate Stations across tests.
fn setup_station() -> Station {
    let telemetry_items = vec![
        Field {
            name: String::from("temperature"),
            standard: true,
        },
        Field {
            name: String::from("humidity"),
            standard: true,
        },
        Field {
            name: String::from("dewPoint"),
            standard: true,
        },
        Field {
            name: String::from("windSpeed"),
            standard: true,
        },
        Field {
            name: String::from("windDirection"),
            standard: true,
        },
        Field {
            name: String::from("dailyPrecipitation"),
            standard: false,
        },
        Field {
            name: String::from("maxWindSpeed"),
            standard: false,
        },
        Field {
            name: String::from("highTemperature"),
            standard: false,
        },
        Field {
            name: String::from("lowTemperature"),
            standard: false,
        },
    ];
    Station::new(telemetry_items)
}

#[test]
fn station_can_report_standard_telemetry() {
    let station = setup_station();
    let standard = station.report_standard();

    assert_eq!(
        standard,
        vec![
            String::from("dewPoint"),
            String::from("humidity"),
            String::from("temperature"),
            String::from("windDirection"),
            String::from("windSpeed"),
        ]
    );
}

#[test]
fn station_can_report_all_telemetry() {
    let station = setup_station();
    let standard = station.report_all();

    assert_eq!(
        standard,
        vec![
            String::from("dailyPrecipitation"),
            String::from("dewPoint"),
            String::from("highTemperature"),
            String::from("humidity"),
            String::from("lowTemperature"),
            String::from("maxWindSpeed"),
            String::from("temperature"),
            String::from("windDirection"),
            String::from("windSpeed"),
        ]
    );
}

#[test]
fn station_can_get_field_item() {
    let station = setup_station();
    let wind_speed = station.get_field("windSpeed").unwrap();

    assert_eq!(wind_speed, 0);
}

#[test]
fn station_can_set_field_item() {
    let mut station = setup_station();

    station.set_field("windSpeed", 15).unwrap();

    let wind_speed = station.get_field("windSpeed").unwrap();

    assert_eq!(wind_speed, 15);
}

#[test]
fn station_can_get_standard_telemetry() {
    let mut station = setup_station();

    station.set_field("temperature", 60).unwrap();
    station.set_field("humidity", 30).unwrap();
    station.set_field("dewPoint", 28).unwrap();
    station.set_field("windSpeed", 15).unwrap();
    station.set_field("windDirection", 2).unwrap();

    let standard_telemetry = station.get_standard().unwrap();

    assert_eq!(standard_telemetry, vec![28, 30, 60, 2, 15]);
}

#[test]
fn station_can_get_all_telemetry() {
    let mut station = setup_station();

    station.set_field("dewPoint", 28).unwrap();
    station.set_field("humidity", 30).unwrap();
    station.set_field("temperature", 60).unwrap();
    station.set_field("windSpeed", 15).unwrap();
    station.set_field("windDirection", 2).unwrap();

    station.set_field("dailyPrecipitation", 0).unwrap();
    station.set_field("highTemperature", 64).unwrap();
    station.set_field("lowTemperature", 38).unwrap();
    station.set_field("maxWindSpeed", 32).unwrap();

    let standard_telemetry = station.get_all();

    assert_eq!(standard_telemetry, vec![0, 28, 64, 30, 38, 32, 60, 2, 15]);
}
