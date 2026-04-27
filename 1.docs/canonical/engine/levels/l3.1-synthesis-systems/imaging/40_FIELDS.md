# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| render_target | RenderTarget | required | framebuffer semantic | owned by `engine_imaging` |
| render_view | RenderView | required | view projection semantic | owned by `engine_imaging` |
| frame_resource | FrameResource | required | per-frame GPU resource | owned by `engine_imaging` |
| upload_stage | UploadStage | required | staging buffer for texture upload | owned by `engine_imaging` |
