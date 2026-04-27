mod common;
use common::*;

#[test]
fn core_generation_non_zero_0() {
    assert_ne!(Generation(0).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_1() {
    assert_ne!(Generation(1).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_2() {
    assert_ne!(Generation(2).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_3() {
    assert_ne!(Generation(3).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_4() {
    assert_ne!(Generation(4).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_5() {
    assert_ne!(Generation(5).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_6() {
    assert_ne!(Generation(6).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_7() {
    assert_ne!(Generation(7).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_8() {
    assert_ne!(Generation(8).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_9() {
    assert_ne!(Generation(9).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_10() {
    assert_ne!(Generation(10).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_11() {
    assert_ne!(Generation(11).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_12() {
    assert_ne!(Generation(12).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_13() {
    assert_ne!(Generation(13).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_14() {
    assert_ne!(Generation(14).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_15() {
    assert_ne!(Generation(15).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_16() {
    assert_ne!(Generation(16).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_17() {
    assert_ne!(Generation(17).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_18() {
    assert_ne!(Generation(18).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_19() {
    assert_ne!(Generation(19).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_20() {
    assert_ne!(Generation(20).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_21() {
    assert_ne!(Generation(21).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_22() {
    assert_ne!(Generation(22).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_23() {
    assert_ne!(Generation(23).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_24() {
    assert_ne!(Generation(24).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_25() {
    assert_ne!(Generation(25).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_26() {
    assert_ne!(Generation(26).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_27() {
    assert_ne!(Generation(27).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_28() {
    assert_ne!(Generation(28).next(), Generation(0));
}
#[test]
fn core_generation_non_zero_29() {
    assert_ne!(Generation(29).next(), Generation(0));
}
#[test]
fn core_tick_value_0() {
    let t = Tick(0);
    assert_eq!(t.0, 0);
}
#[test]
fn core_tick_value_1() {
    let t = Tick(1);
    assert_eq!(t.0, 1);
}
#[test]
fn core_tick_value_2() {
    let t = Tick(2);
    assert_eq!(t.0, 2);
}
#[test]
fn core_tick_value_3() {
    let t = Tick(3);
    assert_eq!(t.0, 3);
}
#[test]
fn core_tick_value_4() {
    let t = Tick(4);
    assert_eq!(t.0, 4);
}
#[test]
fn core_tick_value_5() {
    let t = Tick(5);
    assert_eq!(t.0, 5);
}
#[test]
fn core_tick_value_6() {
    let t = Tick(6);
    assert_eq!(t.0, 6);
}
#[test]
fn core_tick_value_7() {
    let t = Tick(7);
    assert_eq!(t.0, 7);
}
#[test]
fn core_tick_value_8() {
    let t = Tick(8);
    assert_eq!(t.0, 8);
}
#[test]
fn core_tick_value_9() {
    let t = Tick(9);
    assert_eq!(t.0, 9);
}
#[test]
fn core_tick_value_10() {
    let t = Tick(10);
    assert_eq!(t.0, 10);
}
#[test]
fn core_tick_value_11() {
    let t = Tick(11);
    assert_eq!(t.0, 11);
}
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
