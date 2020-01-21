use crate::station::{Field, Station};

#[test]
fn station_can_report_standard_telemetry() {
    let telemetry_items = vec![
        Field { name: String::from("temperature"), standard: true },
        Field { name: String::from("humidity"), standard: true },
        Field { name: String::from("dewPoint"), standard: true },
        Field { name: String::from("windSpeed"), standard: true },
        Field { name: String::from("windDirection"), standard: true },
        Field { name: String::from("dailyParticipation"), standard: false },
        Field { name: String::from("maxWindSpeed"), standard: false },
        Field { name: String::from("highTemperature"), standard: false },
        Field { name: String::from("lowTemperature"), standard: false },
    ];

    let station = Station::new(telemetry_items);

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
    let telemetry_items = vec![
        Field { name: String::from("temperature"), standard: true },
        Field { name: String::from("humidity"), standard: true },
        Field { name: String::from("dewPoint"), standard: true },
        Field { name: String::from("windSpeed"), standard: true },
        Field { name: String::from("windDirection"), standard: true },
        Field { name: String::from("dailyParticipation"), standard: false },
        Field { name: String::from("maxWindSpeed"), standard: false },
        Field { name: String::from("highTemperature"), standard: false },
        Field { name: String::from("lowTemperature"), standard: false },
    ];

    let station = Station::new(telemetry_items);

    let standard = station.report_all();

    assert_eq!(
        standard,
        vec![
            String::from("dailyParticipation"),
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