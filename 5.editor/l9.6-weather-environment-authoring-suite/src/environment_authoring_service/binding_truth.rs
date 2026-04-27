use uuid::Uuid;

/// Authoritative environment bindings read by editor surfaces.
#[derive(Debug, Clone, Default)]
pub struct EnvironmentBindings {
    sky_profile_binding: Option<Uuid>,
    weather_regime_binding: Option<Uuid>,
    cloud_profile_binding: Option<Uuid>,
}

impl EnvironmentBindings {
    pub fn bind_sky_profile(&mut self, profile_id: Uuid) {
        self.sky_profile_binding = Some(profile_id);
    }

    pub fn bind_weather_regime(&mut self, regime_id: Uuid) {
        self.weather_regime_binding = Some(regime_id);
    }

    pub fn bind_cloud_profile(&mut self, profile_id: Uuid) {
        self.cloud_profile_binding = Some(profile_id);
    }

    pub fn sky_profile_binding(&self) -> Option<Uuid> {
        self.sky_profile_binding
    }

    pub fn weather_regime_binding(&self) -> Option<Uuid> {
        self.weather_regime_binding
    }

    pub fn cloud_profile_binding(&self) -> Option<Uuid> {
        self.cloud_profile_binding
    }

    pub fn clear(&mut self) {
        self.sky_profile_binding = None;
        self.weather_regime_binding = None;
        self.cloud_profile_binding = None;
    }
}
