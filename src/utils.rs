
// def as_int(f: float) -> int:
//      return int(round(f))
// from time import time

fn as_int(f: f32) -> u32 {
        return f.parse::<u32>()
}

fn timestamp_ms() -> u32 {
    return as_int()
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(as_int(1.0), 1);
    }

}