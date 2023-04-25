use fdlf_rs_tutorial::power_system::{Bus, Branch, BusType, PowerSystem};

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