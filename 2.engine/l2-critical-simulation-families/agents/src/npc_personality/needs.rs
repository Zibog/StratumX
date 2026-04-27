use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeedType {
    Hunger,
    Rest,
    Safety,
    Money,
    Social,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Need {
    pub need_type: NeedType,
    pub value: f32,
    pub critical_threshold: f32,
    pub decay_rate: f32,
}

impl Need {
    pub fn new(need_type: NeedType, initial_value: f32) -> Self {
        let (critical_threshold, decay_rate) = match need_type {
            NeedType::Hunger => (70.0, 5.0),
            NeedType::Rest => (75.0, 4.0),
            NeedType::Safety => (80.0, 2.0),
            NeedType::Money => (60.0, 1.0),
            NeedType::Social => (50.0, 0.5),
        };

        Self {
            need_type,
            value: initial_value.clamp(0.0, 100.0),
            critical_threshold,
            decay_rate,
        }
    }

    pub fn is_critical(&self) -> bool {
        self.value >= self.critical_threshold
    }

    pub fn update(&mut self, delta_hours: f32) {
        self.value = (self.value + self.decay_rate * delta_hours).min(100.0);
    }

    pub fn satisfy(&mut self, amount: f32) {
        self.value = (self.value - amount).max(0.0);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeedsState {
    pub needs: Vec<Need>,
}

impl NeedsState {
    pub fn new() -> Self {
        Self {
            needs: vec![
                Need::new(NeedType::Hunger, 20.0),
                Need::new(NeedType::Rest, 30.0),
                Need::new(NeedType::Safety, 10.0),
                Need::new(NeedType::Money, 40.0),
                Need::new(NeedType::Social, 25.0),
            ],
        }
    }

    pub fn get_need(&self, need_type: NeedType) -> Option<&Need> {
        self.needs.iter().find(|n| n.need_type == need_type)
    }

    pub fn get_need_mut(&mut self, need_type: NeedType) -> Option<&mut Need> {
        self.needs.iter_mut().find(|n| n.need_type == need_type)
    }

    pub fn update(&mut self, delta_hours: f32) {
        for need in &mut self.needs {
            need.update(delta_hours);
        }
    }

    pub fn has_critical_need(&self) -> bool {
        self.needs.iter().any(|n| n.is_critical())
    }

    pub fn most_critical_need(&self) -> Option<NeedType> {
        self.needs
            .iter()
            .filter(|n| n.is_critical())
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            .map(|n| n.need_type)
    }
}

impl Default for NeedsState {
    fn default() -> Self {
        Self::new()
    }
}
