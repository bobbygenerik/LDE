use std::env;

#[derive(Debug)]
pub struct SeatState {
    pub name: String,
}

pub fn init_seats() -> Vec<SeatState> {
    let name = env::var("XDG_SEAT").unwrap_or_else(|_| "seat0".to_string());
    vec![SeatState { name }]
}
