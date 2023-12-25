
use serde::{Serialize, Deserialize};

pub enum Provider {
    None,
    Coingecko,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Asset {
    XMR,
    BTC,
    ETH,
    USD,
}

impl Asset {
    pub fn get_provider(&self) -> Provider {
        use Asset::*;
        return match self {
            XMR | ETH | BTC => Provider::Coingecko,
            USD => Provider::None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Holding {
    pub name: String,
    pub asset: Asset,
    pub shares: f64,
}

impl Holding {
    pub fn new(
        name: &str,
        asset: Asset,
        shares: f64,
    ) -> Self { return Self {
        name: name.to_string(),
        asset,
        shares
    }}
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Holdings (pub Vec<Holding>);

impl Holdings {
    pub fn from_file <
        P: AsRef<std::path::Path>
    > (fpath: P) -> Result<Self, Box<dyn std::error::Error>> {
        return crate::util::read_from_ron_file(fpath);
    }

    pub fn to_file <
        P: AsRef<std::path::Path>
    > (&self, fpath: P) -> () {
        return crate::util::_write_to_ron_file(fpath, self);
    }

    pub fn enumerate_assets(&self) -> Vec<Asset> {
        return self.0.iter()
            .map(|holding| return holding.asset.clone())
            .collect::<Vec<Asset>>()
            .partition_dedup()
            .0
            .iter()
            .map(|asset| asset.clone())
            .collect::<Vec<Asset>>();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Record {
    pub holdings: Holdings,
    pub prices: std::collections::HashMap<Asset, f64>,
    pub time: chrono::DateTime<chrono::Utc>
}

impl Record {
    pub fn new(
        holdings: Holdings,
        prices: std::collections::HashMap<Asset, f64>,
    ) -> Self {
        return Self {
            holdings,
            prices,
            time: chrono::Utc::now(),
        };
    }

    pub fn net_value(&self) -> f64 {
        let mut output: f64 = 0.;
        for holding in self.holdings.0.iter() {
            output += holding.shares*self.prices[&holding.asset];
        }

        return output;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Records (pub Vec<Record>);

impl Records {
    pub fn from_file <
        P: AsRef<std::path::Path>
    > (fpath: P) -> Result<Self, Box<dyn std::error::Error>> {
        return match std::fs::read_to_string(fpath) {
            Ok(string) => {
                match ron::de::from_str(string.as_str()) {
                    Ok(value) => Ok(value),
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(e) => Err(Box::new(e)),
        };
    }

    pub fn to_file <
        P: AsRef<std::path::Path>
    > (&self, fpath: P) -> Result<(), String> {
        return match ron::ser::to_string(self) {
            Ok(string) => {
                match std::fs::write(fpath, string) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        };
    }

    pub fn push(&mut self, item: Record) {
        self.0.push(item);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn window(
        &self,
        time_range: crate::util::TimeWindow,
    ) -> Self {
        return self.0.iter()
            .filter(|record| {
                record.time >= time_range.start &&
                record.time < time_range.end
            }).map(|record_ref| {
                record_ref.clone()
            }).collect::<Vec<Record>>().into()
    }

    pub fn minimum(&self) -> f64 {
        return self.0.iter()
            .map(|record| {
                return record.net_value();
            }).reduce(|acc, num| {
                if num < acc { return num }
                else { return acc }
            })
            .unwrap();
    }

    pub fn maximum(&self) -> f64 {
        return self.0.iter()
        .map(|record| record.net_value())
            .reduce(|acc, num| {
                if num > acc { return num }
                else { return acc }
            })
            .unwrap();
    }

    pub fn start(
        &self
    ) -> chrono::DateTime<chrono::Utc> {
        return self.0.iter()
            .map(|record| {
                return record.time
            }).reduce(|acc, time| {
                if time < acc { return time }
                else { return acc }
            })
            .unwrap();
    }

    pub fn end(
        &self,
    ) -> chrono::DateTime<chrono::Utc> {
        return self.0.iter()
            .map(|record| record.time )
            .reduce(|acc, time| {
                if time > acc { return time }
                else { return acc }
            })
            .unwrap();
    }

    pub fn filter_accounts(&self, accounts: Vec<String>) -> Self {
        let self_filtered = self.0.iter()
            .map(|record| {
                let holdings_filtered = record.holdings.0.iter()
                    .filter(|holding| {
                        return accounts.contains(&holding.name.to_string())
                    })
                    .map(|holding_ref| holding_ref.clone())
                    .collect::<Vec<Holding>>();

                let holdings = Holdings(holdings_filtered);

                return Record {
                    holdings,
                    prices: record.prices.clone(),
                    time: record.time,
                };
            })
            .collect::<Vec<Record>>();

        return Self ( self_filtered );
    }

    pub fn get_accounts(&self) -> Vec<String> {
        return self.0[self.0.len()-1].holdings.0.iter()
            .map(|holding| holding.name.clone())
            .collect::<Vec<String>>();
    }
}

impl std::ops::Index<usize> for Records {
    type Output = Record;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl From<Vec<Record>> for Records {
    fn from(value: Vec<Record>) -> Self {
        return Self(value);
    }
}




