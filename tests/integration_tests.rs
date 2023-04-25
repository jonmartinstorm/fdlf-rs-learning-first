use fdlf_rs_tutorial::power_system::{Bus, Branch, BusType, PowerSystem};

#[test]
fn test_bus() {
    let bus1 =  Bus {
        id: 1,
        name: "Bus 1".to_string(),
        voltage: 1.0,
        angle: 0.0,
        p_gen: 1.0,
        q_gen: 0.0,
        p_load: 0.0,
        q_load: 0.0,
        bus_type: BysType::Slack,
    };

    let bus2 =  Bus {
        id: 2,
        name: "Bus 2".to_string(),
        voltage: 1.0,
        angle: 0.0,
        p_gen: -1.0, // negative generation is invalid
        q_gen: 0.0,
        p_load: 1.0,
        q_load: 0.0,
        bus_type: BysType::PQ,
    };

    assert_eq!(bus1.id, 1);
    assert_eq!(bus1.name, "Bus 1");
    assert_eq!(bus1.voltage, 1.0);
    assert_eq!(bus1.angle, 0.0);
    assert_eq!(bus1.p_gen, 1.0);
    assert_eq!(bus1.q_gen, 0.0);
    assert_eq!(bus1.p_load, 0.0);
    assert_eq!(bus1.q_load, 0.0);
    assert_eq!(bus1.bus_type, BysType::Slack);

    assert_eq!(bus2.id, 2);
    assert_eq!(bus2.name, "Bus 2");
    assert_eq!(bus2.voltage, 1.0);
    assert_eq!(bus2.angle, 0.0);
    assert_eq!(bus2.p_gen, -1.0);
    assert_eq!(bus2.q_gen, 0.0);
    assert_eq!(bus2.p_load, 1.0);
    assert_eq!(bus2.q_load, 0.0);
    assert_eq!(bus2.bus_type, BysType::PQ);

}