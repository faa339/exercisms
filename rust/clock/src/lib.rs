use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock{
    minutes: i32,
}
//tried working with hours and mins before, this ended up being way simpler 
//we take advantage of the fact that there are 1440 mins per day, 
//then just add mins and use rem_euclid to mod it and handle negatives 
//we only care about formatting when displaying so we do that at display time
const DAY_MINS:i32 = 24*60;
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = (hours * 60 + minutes).rem_euclid(DAY_MINS);
        return Clock{minutes:m};
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock{minutes: (self.minutes + minutes).rem_euclid(DAY_MINS)}
    }

    pub fn to_string(&self) -> String{
        return format!("{:02}:{:02}", self.minutes/60, self.minutes%60);
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}

