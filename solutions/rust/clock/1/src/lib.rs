use std::fmt::{self, Debug, Display, Formatter};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours % 24, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        (self.hours % 24 == other.hours % 24) && (self.minutes == other.minutes)
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Clock {{ hours: {}, minutes: {} }}",
            self.hours, self.minutes
        )
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        let mut minutes = minutes;

        while minutes < 0 {
            hours -= 1;
            minutes += 60;
        }

        while hours < 0 {
            hours += 24;
        }

        Clock {
            hours: hours + minutes / 60,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut minutes = match minutes {
            0 => 0,
            _ => minutes,
        };

        println!("New minutes: {}", minutes);

        if minutes > 0 {
            let hours_to_add = (self.minutes + minutes) / 60;

            if hours_to_add > 0 {
                minutes = self.minutes + minutes;
                while minutes >= 60 {
                    minutes -= 60;
                }
            }

            println!("{}", minutes);

            return Clock {
                hours: self.hours + hours_to_add,
                minutes: minutes,
            };
        } else if minutes < 0 {
            minutes = self.minutes + minutes;
            let mut hours = self.hours;

            while minutes < 0 {
                hours -= 1;
                minutes += 60;
            }

            while hours < 0 {
                hours += 24;
            }

            return Clock {
                hours: hours,
                minutes: minutes,
            };
        }

        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }
}
