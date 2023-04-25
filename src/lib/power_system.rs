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
    bus_number: i32,
    real_power: f64,
    reactive_power: f64,
    max_reactive_power: f64,
    min_reactive_power: f64,
    voltage_magnitude: f64,
    rated_power: f64,
    status: i32,
    max_real_power: f64,
    min_real_power: f64,
    startup_cost: f64,
    shutdown_cost: f64,
    polynomial_coefficients: Vec<f64>,
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