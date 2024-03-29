use std::str::FromStr;

pub enum Status {
    Tracked = 0,
    Untracked = 1,
}
impl Status {
    pub fn opposite(&self) -> Self {
        match self {
            Status::Tracked => Status::Untracked,
            Status::Untracked => Status::Tracked,
        }
    }
}
impl From<bool> for Status {
    fn from(tracked: bool) -> Self {
        if tracked {
            Status::Tracked
        } else {
            Status::Untracked
        }
    }
}
impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Status::Tracked),
            "1" => Ok(Status::Untracked),
            _ => Err(()),
        }
    }
}
