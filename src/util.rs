use std::time::SystemTime;

#[inline(always)]
pub fn time<F: FnMut() -> T, T>(mut f: F) -> (u64, T) {
    let before = SystemTime::now();
    
    let result = f();
    
    let after = SystemTime::now();
    
    (after.duration_since(before).unwrap().as_micros() as u64, result)
}