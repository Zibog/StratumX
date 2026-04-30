#[test]
fn net_latency_bucket_case_0() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(10);
    s.record_sample(60);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_1() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(11);
    s.record_sample(61);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_2() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(12);
    s.record_sample(62);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_3() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(13);
    s.record_sample(63);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_4() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(14);
    s.record_sample(64);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_5() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(15);
    s.record_sample(65);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_6() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(16);
    s.record_sample(66);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_7() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(17);
    s.record_sample(67);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_8() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(18);
    s.record_sample(68);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_9() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(19);
    s.record_sample(69);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_10() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(20);
    s.record_sample(70);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_11() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(21);
    s.record_sample(71);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_12() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(22);
    s.record_sample(72);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_13() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(23);
    s.record_sample(73);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_14() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(24);
    s.record_sample(74);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_15() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(25);
    s.record_sample(75);
    assert_eq!(s.metrics().bucket, LatencyBucket::Low);
}
#[test]
fn net_latency_bucket_case_16() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(26);
    s.record_sample(76);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_17() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(27);
    s.record_sample(77);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_18() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(28);
    s.record_sample(78);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_19() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(29);
    s.record_sample(79);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_20() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(30);
    s.record_sample(80);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_21() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(31);
    s.record_sample(81);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_22() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(32);
    s.record_sample(82);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_23() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(33);
    s.record_sample(83);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
#[test]
fn net_latency_bucket_case_24() {
    let mut s = NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
    s.record_sample(34);
    s.record_sample(84);
    assert_eq!(s.metrics().bucket, LatencyBucket::Medium);
}
