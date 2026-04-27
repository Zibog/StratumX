use crate::dashboard::*;
use std::collections::HashMap;

pub struct ProductionDashboardService {
    dashboards: HashMap<uuid::Uuid, Dashboard>,
}

impl ProductionDashboardService {
    pub fn new() -> Self {
        Self {
            dashboards: HashMap::new(),
        }
    }

    pub fn create_dashboard(&mut self, name: String) -> uuid::Uuid {
        let dashboard = Dashboard::new(name);
        let id = dashboard.id;
        self.dashboards.insert(id, dashboard);
        id
    }
}

impl Default for ProductionDashboardService {
    fn default() -> Self {
        Self::new()
    }
}