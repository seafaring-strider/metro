#[test]
fn create_station_with_vector_of_fields() {
    let telemetry_items = vec![
        Field { name: "temperature", standard: true },
        Field { name: "humidity", standard: true },
        Field { name: "dewPoint", standard: true },
        Field { name: "windSpeed", standard: true },
        Field { name: "windDirection", standard: true },
        Field { name: "dailyParticipation", standard: false },
        Field { name: "maxWindSpeed", standard: false },
        Field { name: "highTemperature", standard: false },
        Field { name: "lowTemperature", standard: false },
    ];

    let station = Station::new(telemetry_items).unwrap();

    assert_eq!(
        station,
        Station {
            telemetry: vec![
                Field { name: "temperature", standard: true },
                Field { name: "humidity", standard: true },
                Field { name: "dewPoint", standard: true },
                Field { name: "windSpeed", standard: true },
                Field { name: "windDirection", standard: true },
                Field { name: "dailyParticipation", standard: false },
                Field { name: "maxWindSpeed", standard: false },
                Field { name: "highTemperature", standard: false },
                Field { name: "lowTemperature", standard: false },
            ]
        }
    )
}
