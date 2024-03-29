use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type PoliticianID = String;

#[derive(Serialize, Deserialize)]
pub struct Politician {
    #[serde(rename = "_stateId")]
    pub state_id: String,

    #[serde(rename = "chamber")]
    pub chamber: Chamber,

    #[serde(rename = "dob")]
    dob: String,

    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "gender")]
    gender: Gender,

    #[serde(rename = "lastName")]
    pub last_name: String,

    #[serde(rename = "nickname")]
    nickname: Option<String>,

    #[serde(rename = "party")]
    pub party: Party,
}

#[derive(Serialize, Deserialize)]
pub struct PoliticianDetail {
    #[serde(rename = "_politicianId")]
    pub politician_id: PoliticianID,

    #[serde(rename = "_stateId")]
    pub state_id: String,

    #[serde(rename = "party")]
    pub party: Party,

    #[serde(rename = "partyOther")]
    party_other: Option<serde_json::Value>,

    #[serde(rename = "district")]
    district: Option<String>,

    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,

    #[serde(rename = "nickname")]
    nickname: Option<String>,

    #[serde(rename = "middleName")]
    middle_name: Option<String>,

    #[serde(rename = "fullName")]
    full_name: String,

    #[serde(rename = "dob")]
    dob: String,

    #[serde(rename = "gender")]
    gender: Gender,

    #[serde(rename = "socialFacebook")]
    social_facebook: Option<String>,

    #[serde(rename = "socialTwitter")]
    social_twitter: Option<String>,

    #[serde(rename = "socialYoutube")]
    social_youtube: Option<String>,

    #[serde(rename = "website")]
    website: Option<String>,

    #[serde(rename = "chamber")]
    pub chamber: Chamber,

    #[serde(rename = "committees")]
    committees: Vec<String>,

    #[serde(rename = "stats")]
    pub stats: Stats,
}
impl Into<Politician> for PoliticianDetail {
    fn into(self) -> Politician {
        Politician {
            state_id: self.state_id,
            chamber: self.chamber,
            dob: self.dob,
            first_name: self.first_name,
            gender: self.gender,
            last_name: self.last_name,
            nickname: self.nickname,
            party: self.party,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "dateLastTraded")]
    pub date_last_traded: Option<NaiveDate>,

    #[serde(rename = "countTrades")]
    pub count_trades: i64,

    #[serde(rename = "countIssuers")]
    pub count_issuers: i64,

    #[serde(rename = "volume")]
    pub volume: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Chamber {
    #[serde(rename = "house")]
    House,

    #[serde(rename = "senate")]
    Senate,
}

#[derive(Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "female")]
    Female,

    #[serde(rename = "male")]
    Male,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Party {
    #[serde(rename = "democrat")]
    Democrat,

    #[serde(rename = "republican")]
    Republican,

    #[serde(rename = "other")]
    Other,
}
impl std::fmt::Display for Party {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Party::Democrat => "democrat",
                Party::Republican => "republican",
                Party::Other => "other",
            }
        )?;
        Ok(())
    }
}
