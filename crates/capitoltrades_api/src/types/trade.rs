use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::{
    issuer::Sector,
    politician::{Politician, PoliticianID},
    Chamber, IssuerID,
};

extern crate serde_json;

#[derive(Copy, Clone)]
pub enum TradeSize {
    Less1K = 1,
    From1Kto15K = 2,
    From15Kto50K = 3,
    From50Kto100K = 4,
    From100Kto250K = 5,
    From250Kto500K = 6,
    From500Kto1M = 7,
    From1Mto5M = 8,
    From5Mto25M = 9,
    From25Mto50M = 10,
}

#[derive(Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "_txId")]
    pub tx_id: i64,

    #[serde(rename = "_politicianId")]
    pub politician_id: PoliticianID,

    #[serde(rename = "_assetId")]
    asset_id: i64,

    #[serde(rename = "_issuerId")]
    pub issuer_id: IssuerID,

    #[serde(rename = "pubDate")]
    pub pub_date: DateTime<Utc>,

    #[serde(rename = "filingDate")]
    pub filing_date: NaiveDate,

    #[serde(rename = "txDate")]
    pub tx_date: NaiveDate,

    #[serde(rename = "txType")]
    pub tx_type: TxType,

    #[serde(rename = "txTypeExtended")]
    tx_type_extended: Option<serde_json::Value>,

    #[serde(rename = "hasCapitalGains")]
    has_capital_gains: bool,

    #[serde(rename = "owner")]
    owner: Owner,

    #[serde(rename = "chamber")]
    chamber: Chamber,

    #[serde(rename = "price")]
    pub price: Option<f64>,

    #[serde(rename = "size")]
    pub size: Option<i64>,

    #[serde(rename = "sizeRangeHigh")]
    size_range_high: Option<i64>,

    #[serde(rename = "sizeRangeLow")]
    size_range_low: Option<i64>,

    #[serde(rename = "value")]
    pub value: i64,

    #[serde(rename = "filingId")]
    filing_id: i64,

    #[serde(rename = "filingURL")]
    pub filing_url: String,

    #[serde(rename = "reportingGap")]
    pub reporting_gap: i64,

    #[serde(rename = "comment")]
    comment: Option<String>,

    #[serde(rename = "committees")]
    committees: Vec<String>,

    #[serde(rename = "asset")]
    pub asset: Asset,

    #[serde(rename = "issuer")]
    pub issuer: Issuer,

    #[serde(rename = "politician")]
    pub politician: Politician,

    #[serde(rename = "labels")]
    labels: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "assetType")]
    pub asset_type: String,

    #[serde(rename = "assetTicker")]
    pub asset_ticker: Option<String>,

    #[serde(rename = "instrument")]
    pub instrument: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Issuer {
    #[serde(rename = "_stateId")]
    state_id: Option<String>,

    #[serde(rename = "c2iq")]
    c2_iq: Option<String>,

    #[serde(rename = "country")]
    country: Option<String>,

    #[serde(rename = "issuerName")]
    pub issuer_name: String,

    #[serde(rename = "issuerTicker")]
    pub issuer_ticker: Option<String>,

    #[serde(rename = "sector")]
    sector: Option<Sector>,
}

#[derive(Serialize, Deserialize)]
pub enum Owner {
    #[serde(rename = "child")]
    Child,

    #[serde(rename = "joint")]
    Joint,

    #[serde(rename = "not-disclosed")]
    NotDisclosed,

    #[serde(rename = "self")]
    OwnerSelf,

    #[serde(rename = "spouse")]
    Spouse,
}

#[derive(Serialize, Deserialize)]
pub enum TxType {
    #[serde(rename = "buy")]
    Buy,

    #[serde(rename = "sell")]
    Sell,

    #[serde(rename = "exchange")]
    Exchange,

    #[serde(rename = "receive")]
    Receive,
}
