/// Stage in the tooling conveyor pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConveyorStage {
    /// Raw asset ingested from source.
    Imported,
    /// Asset processed through the cooking pipeline.
    Cooked,
    /// Asset validated and certified for production.
    Certified,
}
