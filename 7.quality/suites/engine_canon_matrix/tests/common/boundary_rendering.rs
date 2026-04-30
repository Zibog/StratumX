use super::*;

pub fn render_receipt(
    service: &ImagingService,
    transfer: &mut TransferControlService,
    request: ImagingRequest,
    material_id: MaterialId,
) -> ImagingResult<ImagingReceipt> {
    let world = WorldState::new();
    let ecs = EcsSubstrate::new();
    let materials = materials();
    let residency = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    service.render(
        request,
        ImagingInputs {
            world: &world,
            ecs: &ecs,
            materials: &materials,
            residency: &residency,
            transfer,
            material_id,
        },
    )
}

pub trait LegacyImagingRenderCompat {
    fn render(
        &self,
        world: &WorldState,
        ecs: &EcsSubstrate,
        materials: &MaterialRegistry,
        residency: &ResidencyControlService,
        transfer: &mut TransferControlService,
        request: ImagingRequest,
    ) -> ImagingResult<ImagingReceipt>;
}

impl LegacyImagingRenderCompat for ImagingService {
    fn render(
        &self,
        world: &WorldState,
        ecs: &EcsSubstrate,
        materials: &MaterialRegistry,
        residency: &ResidencyControlService,
        transfer: &mut TransferControlService,
        request: ImagingRequest,
    ) -> ImagingResult<ImagingReceipt> {
        ImagingService::render(
            self,
            request,
            ImagingInputs {
                world,
                ecs,
                materials,
                residency,
                transfer,
                material_id: MaterialId(1),
            },
        )
    }
}

pub fn synthesize_receipt(
    service: &AcousticsService,
    transfer: &mut TransferControlService,
    request: AcousticsRequest,
    material_id: MaterialId,
) -> AcousticResult<AcousticReceipt> {
    let world = WorldState::new();
    let ecs = EcsSubstrate::new();
    let materials = materials();
    let residency = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    service.synthesize(
        request,
        AcousticInputs {
            world: &world,
            ecs: &ecs,
            materials: &materials,
            residency: &residency,
            transfer,
            material_id,
        },
    )
}

pub trait LegacyAcousticsRenderCompat {
    fn synthesize(
        &self,
        world: &WorldState,
        ecs: &EcsSubstrate,
        materials: &MaterialRegistry,
        residency: &ResidencyControlService,
        transfer: &mut TransferControlService,
        request: AcousticsRequest,
    ) -> AcousticResult<AcousticReceipt>;
}

impl LegacyAcousticsRenderCompat for AcousticsService {
    fn synthesize(
        &self,
        world: &WorldState,
        ecs: &EcsSubstrate,
        materials: &MaterialRegistry,
        residency: &ResidencyControlService,
        transfer: &mut TransferControlService,
        request: AcousticsRequest,
    ) -> AcousticResult<AcousticReceipt> {
        AcousticsService::synthesize(
            self,
            request,
            AcousticInputs {
                world,
                ecs,
                materials,
                residency,
                transfer,
                material_id: MaterialId(1),
            },
        )
    }
}
