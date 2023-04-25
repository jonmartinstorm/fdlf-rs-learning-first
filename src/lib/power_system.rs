#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BusType {
    Slack,
    PV,
    PQ,
}

#[derive(Debug, Clone)]
pub struct Bus {
    pub id: usize,
    pub name: String,
    pub voltage: f64,
    pub angle: f64,
    pub p_gen: f64,
    pub q_gen: f64,
    pub p_load: f64,
    pub q_load: f64,
    pub bus_type: BusType,
}

pub struct Branch {
    // ...
}

pub struct PowerSystem {
    // ...
}