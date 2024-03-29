use std::str::FromStr;

pub enum Action {
    PoliticiansList = 0,
    PoliticiansSearch = 1,
    IssuersList = 2,
    IssuersSearch = 3,
    TradesList = 4,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Action::PoliticiansList),
            "1" => Ok(Action::PoliticiansSearch),
            "2" => Ok(Action::IssuersList),
            "3" => Ok(Action::IssuersSearch),
            "4" => Ok(Action::TradesList),
            _ => Err(()),
        }
    }
}
