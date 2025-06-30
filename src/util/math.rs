pub fn ratio(a: u64, b: u64) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(a as f64 / b as f64)
    }
}

pub fn mb_to_bytes(mb: u64) -> u64 {
    mb * 1_000_000
}

pub fn gb_to_bytes(gb: u64) -> u64 {
    gb * 1_000_000_000
}
