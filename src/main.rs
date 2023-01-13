mod influx;

use clap::Parser;
use flexi_logger::Logger;
use influx::Measurement;
use time::OffsetDateTime;
use wiser::{Hub, State};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The secret for the hub to be accessed.
    #[arg(long, env)]
    secret: String,

    /// The IP or hostname of the hub.
    #[arg(long, env)]
    hub: String,
}

#[tokio::main]
async fn main() {
    if let Err(e) = Logger::try_with_env_or_str("wiser=info,warn").and_then(|logger| logger.start())
    {
        eprintln!("Warning, failed to start logging: {}", e);
    }

    let cli = Cli::parse();

    let hub = Hub::new(&cli.hub, &cli.secret);
    let domain = hub.domain().await.unwrap();
    let now = OffsetDateTime::now_utc();
    // println!("{:#?}", domain);

    for room in domain.rooms.iter() {
        let mut measurement = Measurement::new("wiser_room", now);

        if let Some(room_stat) = domain.room_stats.iter().find(|s| s.id == room.room_stat_id) {
            measurement.add_field("humidity", room_stat.measured_humidity);
        }

        measurement.add_field("temperature", room.calculated_temperature);
        measurement.add_field("scheduled_temperature", room.scheduled_set_point);
        measurement.add_field("set_point", room.current_set_point);
        measurement.add_field(
            "heating",
            if room.control_output_state == State::On {
                1.0
            } else {
                0.0
            },
        );

        measurement.add_tag("room", &room.name);
        measurement.add_tag("id", &room.id.to_string());
        println!("{}", measurement);
    }

    for hw in domain.hot_water.iter() {
        let mut measurement = Measurement::new("wiser_hot_water", now);
        measurement.add_field(
            "heating",
            if hw.water_heating_state == State::On {
                1.0
            } else {
                0.0
            },
        );

        measurement.add_tag("id", &hw.id.to_string());
        println!("{}", measurement);
    }
}
