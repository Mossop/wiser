mod client;
mod error;

use client::Client;
pub use error::Error;
use serde::{Deserialize, Deserializer};

fn deserialize_temperature<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let val = f64::deserialize(deserializer)?;
    Ok(val / 10.0)
}

fn deserialize_optional_temperature<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let val = Option::<f64>::deserialize(deserializer)?;
    Ok(val.map(|v| v / 10.0))
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum State {
    On,
    Off,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Mode {
    Auto,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum DemandType {
    Modulating,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum ControlType {
    FromSchedule,
    FromBoost,
    FromManualOverride,
}

pub struct Hub {
    client: Client,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoomStat {
    #[serde(rename = "id")]
    pub id: usize,
    #[serde(deserialize_with = "deserialize_temperature")]
    pub set_point: f64,
    #[serde(deserialize_with = "deserialize_temperature")]
    pub measured_temperature: f64,
    pub measured_humidity: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Room {
    #[serde(rename = "id")]
    pub id: usize,
    #[serde(default, deserialize_with = "deserialize_optional_temperature")]
    pub manual_set_point: Option<f64>,
    pub schedule_id: usize,
    pub comfort_mode_score: u32,
    pub heating_rate: u32,
    pub room_stat_id: usize,
    pub name: String,
    pub mode: Mode,
    pub demand_type: DemandType,
    pub window_detection_active: bool,
    #[serde(deserialize_with = "deserialize_temperature")]
    pub calculated_temperature: f64,
    #[serde(deserialize_with = "deserialize_temperature")]
    pub current_set_point: f64,
    pub percentage_demand: u8,
    pub control_output_state: State,
    pub setpoint_origin: ControlType,
    #[serde(deserialize_with = "deserialize_temperature")]
    pub displayed_set_point: f64,
    #[serde(deserialize_with = "deserialize_temperature")]
    pub scheduled_set_point: f64,
    pub away_mode_suppressed: bool,
    #[serde(deserialize_with = "deserialize_temperature")]
    pub rounded_alexa_temperature: f64,
    pub effective_mode: Mode,
    pub percentage_demand_for_itrv: u8,
    pub control_direction: String,
    pub heating_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HotWater {
    #[serde(rename = "id")]
    pub id: usize,
    pub override_water_heating_state: State,
    pub schedule_id: usize,
    pub mode: String,
    pub water_heating_state: State,
    pub scheduled_water_heating_state: State,
    pub hot_water_relay_state: State,
    pub hot_water_description: ControlType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeatingChannel {
    #[serde(rename = "id")]
    pub id: usize,
    pub name: String,
    pub room_ids: Vec<usize>,
    pub percentage_demand: u8,
    pub demand_on_off_output: State,
    pub heating_relay_state: State,
    pub is_smart_valve_preventing_demand: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Domain {
    #[serde(rename = "HeatingChannel")]
    pub heating_channels: Vec<HeatingChannel>,
    pub hot_water: Vec<HotWater>,
    #[serde(rename = "Room")]
    pub rooms: Vec<Room>,
    #[serde(rename = "RoomStat")]
    pub room_stats: Vec<RoomStat>,
}

impl Hub {
    pub fn new(host: &str, secret: &str) -> Self {
        Self {
            client: Client::new(host, secret),
        }
    }

    pub async fn domain(&self) -> Result<Domain, Error> {
        self.client.get("/domain/").await
    }
}
