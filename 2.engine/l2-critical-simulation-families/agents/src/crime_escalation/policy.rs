use super::types::{CrimeReason, CrimeRecord, CrimeType, CriminalStatus};
use crate::{NeedType, NeedsState, PersonalityTraits};

pub struct CrimeEscalationCalculator;

impl CrimeEscalationCalculator {
    pub fn calculate_crime_probability(
        needs: &NeedsState,
        personality: &PersonalityTraits,
        criminal_status: &CriminalStatus,
    ) -> f32 {
        let mut probability = 0.0;

        if let Some(critical_need) = needs.most_critical_need() {
            let need_value = needs.get_need(critical_need).unwrap().value;
            let desperation = (need_value - 50.0) / 50.0;
            probability += desperation * 0.4;
        }

        probability += personality.aggression * 0.2;
        probability += personality.greed * 0.2;
        probability -= personality.discipline * 0.3;

        if criminal_status.is_criminal {
            probability += 0.2;
        }

        probability.clamp(0.0, 1.0)
    }

    pub fn determine_crime_type(
        needs: &NeedsState,
        personality: &PersonalityTraits,
        criminal_status: &CriminalStatus,
    ) -> (CrimeType, CrimeReason) {
        if let Some(critical_need) = needs.most_critical_need() {
            let need = needs.get_need(critical_need).unwrap();

            if need.value > 70.0 {
                let crime_type = if personality.aggression > 0.6 {
                    CrimeType::Robbery
                } else {
                    CrimeType::Theft
                };

                return (
                    crime_type,
                    CrimeReason::Desperation {
                        need: critical_need,
                        value: need.value,
                    },
                );
            }
        }

        for need in &needs.needs {
            if need.value > 65.0 {
                let crime_type = if personality.aggression > 0.5 {
                    CrimeType::Robbery
                } else {
                    CrimeType::Theft
                };

                return (
                    crime_type,
                    CrimeReason::Desperation {
                        need: need.need_type,
                        value: need.value,
                    },
                );
            }
        }

        if personality.greed > 0.6 {
            return (
                CrimeType::Theft,
                CrimeReason::Greed {
                    opportunity_value: personality.greed * 100.0,
                },
            );
        }

        if criminal_status.is_criminal && personality.aggression > 0.5 {
            return (
                CrimeType::Assault,
                CrimeReason::Desperation {
                    need: NeedType::Money,
                    value: 90.0,
                },
            );
        }

        (
            CrimeType::Theft,
            CrimeReason::Desperation {
                need: NeedType::Hunger,
                value: 50.0,
            },
        )
    }

    pub fn escalate_under_scarcity(
        needs: &mut NeedsState,
        personality: &PersonalityTraits,
        criminal_status: &mut CriminalStatus,
        scarcity_factor: f32,
        current_time: f32,
    ) -> Option<CrimeRecord> {
        if let Some(hunger) = needs.get_need_mut(NeedType::Hunger) {
            hunger.value = (hunger.value + scarcity_factor * 20.0).min(100.0);
        }
        if let Some(money) = needs.get_need_mut(NeedType::Money) {
            money.value = (money.value + scarcity_factor * 15.0).min(100.0);
        }

        let crime_prob = Self::calculate_crime_probability(needs, personality, criminal_status);
        let adjusted_prob = (crime_prob + scarcity_factor * 0.5).min(1.0);

        if adjusted_prob > 0.5 {
            let (crime_type, reason) =
                Self::determine_crime_type(needs, personality, criminal_status);

            let record = CrimeRecord {
                crime_type,
                reason,
                timestamp: current_time,
                target_id: None,
                witness_count: 0,
            };

            criminal_status.commit_crime(record.clone());
            return Some(record);
        }

        None
    }
}
