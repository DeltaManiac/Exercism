use std::fmt;

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 24 * MINUTES_PER_HOUR;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let total_minutes = hours * MINUTES_PER_HOUR + minutes;
        let total_minutes_to_rollover = total_minutes % MINUTES_PER_DAY;
        let minutes = (total_minutes_to_rollover + MINUTES_PER_DAY) % MINUTES_PER_DAY;
        Clock { minutes }
    }

    pub fn add_minutes(self, minutes: i32) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_PER_HOUR,
            self.minutes % MINUTES_PER_HOUR
        )
    }
}
