use chrono::{Utc, DateTime};

struct Schedule {
    id: u64,
    plane: u64,
    from_city: u64,
    to_city: u64,
    time: DateTime<Utc>,
    created_at: DateTime<Utc>
}