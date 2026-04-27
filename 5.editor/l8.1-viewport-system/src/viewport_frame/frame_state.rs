// Viewport frame state types

use editor_dto_law::{
    ViewportBootFrame, ViewportDegradedFrame, ViewportFailureProjection, ViewportLiveFrame,
    ViewportPlayAttachedFrame,
};

pub enum ViewportFrame {
    Boot(ViewportBootFrame),
    Live(ViewportLiveFrame),
    Degraded(ViewportDegradedFrame),
    Failure(ViewportFailureProjection),
    PlayAttached(ViewportPlayAttachedFrame),
}
