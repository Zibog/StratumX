#[test]
fn core_tick_value_12() {
    let t = Tick(12);
    assert_eq!(t.0, 12);
}
#[test]
fn core_tick_value_13() {
    let t = Tick(13);
    assert_eq!(t.0, 13);
}
#[test]
fn core_tick_value_14() {
    let t = Tick(14);
    assert_eq!(t.0, 14);
}
#[test]
fn core_tick_value_15() {
    let t = Tick(15);
    assert_eq!(t.0, 15);
}
#[test]
fn core_tick_value_16() {
    let t = Tick(16);
    assert_eq!(t.0, 16);
}
#[test]
fn core_tick_value_17() {
    let t = Tick(17);
    assert_eq!(t.0, 17);
}
#[test]
fn core_tick_value_18() {
    let t = Tick(18);
    assert_eq!(t.0, 18);
}
#[test]
fn core_tick_value_19() {
    let t = Tick(19);
    assert_eq!(t.0, 19);
}
#[test]
fn core_aabb_valid_0() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 0.0),
        max: Vec3::new(1.0, 1.0, 1.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_1() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(1.0, 2.0, 3.0),
        max: Vec3::new(1.0, 2.0, 3.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_2() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(-1.0, -1.0, -1.0),
        max: Vec3::new(0.0, 0.0, 0.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_3() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 5.0, 0.0),
        max: Vec3::new(5.0, 6.0, 7.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_4() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(2.0, 2.0, 2.0),
        max: Vec3::new(3.0, 3.0, 3.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_5() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(-10.0, 0.0, 1.0),
        max: Vec3::new(0.0, 1.0, 2.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_6() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(4.0, 4.0, 4.0),
        max: Vec3::new(8.0, 8.0, 8.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_7() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 0.0),
        max: Vec3::new(0.0, 0.0, 0.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_8() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(3.0, 4.0, 5.0),
        max: Vec3::new(6.0, 7.0, 8.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_9() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(7.0, 8.0, 9.0),
        max: Vec3::new(10.0, 11.0, 12.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_10() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 0.0),
        max: Vec3::new(1.0, 1.0, 1.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_11() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(1.0, 2.0, 3.0),
        max: Vec3::new(1.0, 2.0, 3.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_12() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(-1.0, -1.0, -1.0),
        max: Vec3::new(0.0, 0.0, 0.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_13() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 5.0, 0.0),
        max: Vec3::new(5.0, 6.0, 7.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_14() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(2.0, 2.0, 2.0),
        max: Vec3::new(3.0, 3.0, 3.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_15() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(-10.0, 0.0, 1.0),
        max: Vec3::new(0.0, 1.0, 2.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_16() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(4.0, 4.0, 4.0),
        max: Vec3::new(8.0, 8.0, 8.0),
    };
    assert!(a.validate());
}
