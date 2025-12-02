use std::time::{Duration, Instant};

pub fn measure<F, T>(mut f: F) -> (T, Duration)
where
    F: FnMut() -> T,
{
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}
