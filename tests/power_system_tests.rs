use fdlf_rs_tutorial::power_system::{Bus, Branch, BusType, PowerSystem, Generator};

#[test]
fn test_bus() {
    let bus1 =  Bus {
        id: 1,
        name: "Bus 1".to_string(),
        voltage: 1.0,
        angle: 0.0,
        p_load: 0.0,
        q_load: 0.0,
        bus_type: BusType::Slack,
    };

    let bus2 =  Bus {
        id: 2,
        name: "Bus 2".to_string(),
        voltage: 1.0,
        angle: 0.0,
        p_load: 1.0,
        q_load: 0.0,
        bus_type: BusType::PQ,
    };

    assert_eq!(bus1.id, 1);
    assert_eq!(bus1.name, "Bus 1");
    assert_eq!(bus1.voltage, 1.0);
    assert_eq!(bus1.angle, 0.0);
    assert_eq!(bus1.p_load, 0.0);
    assert_eq!(bus1.q_load, 0.0);
    assert_eq!(bus1.bus_type, BusType::Slack);

    assert_eq!(bus2.id, 2);
    assert_eq!(bus2.name, "Bus 2");
    assert_eq!(bus2.voltage, 1.0);
    assert_eq!(bus2.angle, 0.0);
    assert_eq!(bus2.p_load, 1.0);
    assert_eq!(bus2.q_load, 0.0);
    assert_eq!(bus2.bus_type, BusType::PQ);

}

#[test]
fn test_branch() {
    let branch1 = Branch {
        from_bus_id: 1,
        to_bus_id: 2,
        resistance: 0.0,
        reactance: 0.0,
        susceptance: 0.0,
        rating: 0.0,
    };

    let branch2 = Branch {
        from_bus_id: 2,
        to_bus_id: 1,
        resistance: 1.0,
        reactance: 1.0,
        susceptance: 1.0,
        rating: 0.0,
    };

    assert_eq!(branch1.from_bus_id, 1);
    assert_eq!(branch1.to_bus_id, 2);
    assert_eq!(branch1.resistance, 0.0);
    assert_eq!(branch1.reactance, 0.0);
    assert_eq!(branch1.susceptance, 0.0);
    assert_eq!(branch1.rating, 0.0);

    assert_eq!(branch2.from_bus_id, 2);
    assert_eq!(branch2.to_bus_id, 1);
    assert_eq!(branch2.resistance, 1.0);
    assert_eq!(branch2.reactance, 1.0);
    assert_eq!(branch2.susceptance, 1.0);
    assert_eq!(branch2.rating, 0.0);

}

#[test]
fn test_power_system() {
    let powersystem = PowerSystem {
        buses: vec![
            Bus {
                id: 1,
                name: "Bus 1".to_string(),
                voltage: 1.0,
                angle: 0.0,
                p_load: 0.0,
                q_load: 0.0,
                bus_type: BusType::Slack,
            },
            Bus {
                id: 2,
                name: "Bus 2".to_string(),
                voltage: 1.0,
                angle: 0.0,
                p_load: 1.0,
                q_load: 0.0,
                bus_type: BusType::PQ,
            },
        ],
        branches: vec![
            Branch {
                from_bus_id: 1,
                to_bus_id: 2,
                resistance: 0.0,
                reactance: 0.0,
                susceptance: 0.0,
                rating: 0.0,
            },
            Branch {
                from_bus_id: 2,
                to_bus_id: 1,
                resistance: 1.0,
                reactance: 1.0,
                susceptance: 1.0,
                rating: 0.0,
            },
        ],
    };

    // asserts for powersystem
    assert_eq!(powersystem.buses.len(), 2);
    assert_eq!(powersystem.branches.len(), 2);
    
}


#[test]
fn test_generator_struct() {
    let gen = Generator {
        bus_number: 1,
        real_power: 10.0,
        reactive_power: 5.0,
        max_reactive_power: 10.0,
        min_reactive_power: -10.0,
        voltage_magnitude: 1.0,
        rated_power: 20.0,
        status: 1,
        max_real_power: 20.0,
        min_real_power: 0.0,
        startup_cost: 100.0,
        shutdown_cost: 50.0,
        polynomial_coefficients: vec![1.0, -2.0, 3.0],
    };

    assert_eq!(gen.bus_number, 1);
    assert_eq!(gen.real_power, 10.0);
    assert_eq!(gen.reactive_power, 5.0);
    assert_eq!(gen.max_reactive_power, 10.0);
    assert_eq!(gen.min_reactive_power, -10.0);
    assert_eq!(gen.voltage_magnitude, 1.0);
    assert_eq!(gen.rated_power, 20.0);
    assert_eq!(gen.status, 1);
    assert_eq!(gen.max_real_power, 20.0);
    assert_eq!(gen.min_real_power, 0.0);
    assert_eq!(gen.startup_cost, 100.0);
    assert_eq!(gen.shutdown_cost, 50.0);
    assert_eq!(gen.polynomial_coefficients, vec![1.0, -2.0, 3.0]);
}