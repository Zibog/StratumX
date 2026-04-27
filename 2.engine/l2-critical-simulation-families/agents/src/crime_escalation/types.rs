use crate::NeedType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrimeType {
    None,
    Theft,
    Robbery,
    Assault,
    Murder,
}

impl CrimeType {
    pub fn severity(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Theft => 1,
            Self::Robbery => 2,
            Self::Assault => 3,
            Self::Murder => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrimeReason {
    Desperation { need: NeedType, value: f32 },
    Greed { opportunity_value: f32 },
    Revenge { target_id: u32 },
    Coercion { faction_pressure: f32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrimeRecord {
    pub crime_type: CrimeType,
    pub reason: CrimeReason,
    pub timestamp: f32,
    pub target_id: Option<u32>,
    pub witness_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriminalStatus {
    pub is_criminal: bool,
    pub crime_history: Vec<CrimeRecord>,
    pub reputation: f32,
    pub wanted_level: u8,
}

impl CriminalStatus {
    pub fn new() -> Self {
        Self {
            is_criminal: false,
            crime_history: Vec::new(),
            reputation: 50.0,
            wanted_level: 0,
        }
    }

    pub fn commit_crime(&mut self, crime: CrimeRecord) {
        let severity = crime.crime_type.severity() as f32;
        self.reputation -= severity * 15.0;
        self.reputation = self.reputation.max(-100.0);

        if self.reputation < 10.0 {
            self.is_criminal = true;
        }

        self.wanted_level = ((self.reputation.abs() / 20.0) as u8).min(5);
        self.crime_history.push(crime);
    }

    pub fn rehabilitate(&mut self, amount: f32) {
        self.reputation = (self.reputation + amount).min(100.0);

        if self.reputation >= 0.0 {
            self.is_criminal = false;
            self.wanted_level = 0;
        }
    }
}

impl Default for CriminalStatus {
    fn default() -> Self {
        Self::new()
    }
}
