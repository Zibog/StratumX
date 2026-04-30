use super::disabled_reasons;
use super::domains::PromotedCommand;
use super::lifecycle;
use super::serialization;

impl PromotedCommand {
    pub fn route_id(&self) -> &'static str {
        lifecycle::route_id(self)
            .or_else(|| serialization::route_id(self))
            .or_else(|| disabled_reasons::route_id(self))
            .expect("promoted command route ids must stay stable")
    }
}
