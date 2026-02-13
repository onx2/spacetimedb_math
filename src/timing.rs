use crate::Scalar;
use spacetimedb::Timestamp;

const MICROS_PER_SEC: Scalar = 1_000_000.0;

/// Returns the delta time in seconds, or `None` if `last` is after `now`.
pub fn try_delta_time(now: Timestamp, last: Timestamp) -> Option<Scalar> {
    now.time_duration_since(last)
        .map(|dur| dur.to_micros() as Scalar / MICROS_PER_SEC)
}

/// Returns the delta time in seconds, or `fallback` if unavailable.
pub fn delta_time_or(now: Timestamp, last: Timestamp, fallback: Scalar) -> Scalar {
    try_delta_time(now, last).unwrap_or(fallback)
}

/// Returns the delta time in seconds, or computes a fallback lazily.
pub fn delta_time_or_else<F>(now: Timestamp, last: Timestamp, fallback: F) -> Scalar
where
    F: FnOnce() -> Scalar,
{
    try_delta_time(now, last).unwrap_or_else(fallback)
}
