#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BusType {
    Slack,
    PV,
    PQ,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bus {
    pub id: usize,
    pub name: String,
    pub voltage: f64,
    pub angle: f64,
    pub p_load: f64,
    pub q_load: f64,
    pub bus_type: BusType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Generator {
    pub bus_number: i32,
    pub real_power: f64,
    pub reactive_power: f64,
    pub max_reactive_power: f64,
    pub min_reactive_power: f64,
    pub voltage_magnitude: f64,
    pub rated_power: f64,
    pub status: i32,
    pub max_real_power: f64,
    pub min_real_power: f64,
    pub startup_cost: f64,
    pub shutdown_cost: f64,
    #[serde(skip)] // skip this field when serializing, not implemented yet
    pub polynomial_coefficients: Vec<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Branch {
    pub from_bus_id: usize,
    pub to_bus_id: usize,
    pub resistance: f64,
    pub reactance: f64,
    pub susceptance: f64,
    pub rating: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerSystem {
    pub buses: Vec<Bus>,
    pub branches: Vec<Branch>,
}