#[test]
fn core_aabb_valid_17() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 0.0),
        max: Vec3::new(0.0, 0.0, 0.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_18() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(3.0, 4.0, 5.0),
        max: Vec3::new(6.0, 7.0, 8.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_valid_19() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(7.0, 8.0, 9.0),
        max: Vec3::new(10.0, 11.0, 12.0),
    };
    assert!(a.validate());
}
#[test]
fn core_aabb_invalid_0() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(1.0, 0.0, 0.0),
        max: Vec3::new(0.0, 1.0, 1.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_1() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 1.0, 0.0),
        max: Vec3::new(1.0, 0.0, 1.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_2() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 1.0),
        max: Vec3::new(1.0, 1.0, 0.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_3() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(5.0, 5.0, 5.0),
        max: Vec3::new(4.0, 6.0, 7.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_4() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 0.0),
        max: Vec3::new(-1.0, 0.0, 0.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_5() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(1.0, 0.0, 0.0),
        max: Vec3::new(0.0, 1.0, 1.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_6() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 1.0, 0.0),
        max: Vec3::new(1.0, 0.0, 1.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_7() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 1.0),
        max: Vec3::new(1.0, 1.0, 0.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_8() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(5.0, 5.0, 5.0),
        max: Vec3::new(4.0, 6.0, 7.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_9() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 0.0),
        max: Vec3::new(-1.0, 0.0, 0.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_10() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(1.0, 0.0, 0.0),
        max: Vec3::new(0.0, 1.0, 1.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_11() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 1.0, 0.0),
        max: Vec3::new(1.0, 0.0, 1.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_12() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 1.0),
        max: Vec3::new(1.0, 1.0, 0.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_13() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(5.0, 5.0, 5.0),
        max: Vec3::new(4.0, 6.0, 7.0),
    };
    assert!(!a.validate());
}
#[test]
fn core_aabb_invalid_14() {
    let a = engine_core::Aabb3f {
        min: Vec3::new(0.0, 0.0, 0.0),
        max: Vec3::new(-1.0, 0.0, 0.0),
    };
    assert!(!a.validate());
}
