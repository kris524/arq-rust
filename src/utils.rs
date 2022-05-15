
use std::time::SystemTime;

fn as_int(f: f32) -> u32 {
        let int = f as u32;
        return int
}

fn timestamp_ms() -> u64 {
    let sys_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    return sys_time.as_secs() * 1000
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_as_int() {
        assert_eq!(as_int(1.0), 1);
    }

}