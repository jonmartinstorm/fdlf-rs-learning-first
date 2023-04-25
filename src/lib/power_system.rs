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
struct Generator {
    id: usize,
    bus_id: usize,
    p_gen: f64,
    q_gen: f64,
    p_min: f64,
    p_max: f64,
    q_min: f64,
    q_max: f64,
    ramp_up: f64,
    ramp_down: f64,
    start_cost: f64,
    cost_coefficients: Vec<f64>,
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