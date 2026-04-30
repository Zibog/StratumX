use crate::{
    ContentDescriptor, ContentFailure, ContentFailureReason, ContentLocator, ContentPipelineResult,
};

pub(crate) fn validate_descriptor(descriptor: &ContentDescriptor) -> ContentPipelineResult<()> {
    if descriptor.content_id == 0 {
        return Err(ContentFailure::new(
            ContentFailureReason::InvalidDescriptor,
            "content descriptor requires non-zero content id",
        ));
    }
    if descriptor.label.trim().is_empty() {
        return Err(ContentFailure::new(
            ContentFailureReason::InvalidDescriptor,
            "content descriptor requires non-empty label",
        ));
    }
    Ok(())
}

pub(crate) fn validate_locator(locator: &ContentLocator) -> ContentPipelineResult<String> {
    let uri = locator.uri.trim();
    if uri.is_empty() {
        return Err(invalid_locator("content locator requires non-empty uri"));
    }
    let Some((scheme, path)) = uri.split_once("://") else {
        return Err(invalid_locator("content locator requires explicit scheme"));
    };
    if scheme.is_empty() {
        return Err(invalid_locator("content locator requires explicit scheme"));
    }
    if path.trim().is_empty() {
        return Err(invalid_locator("content locator requires non-empty path"));
    }
    Ok(uri.to_string())
}

const fn invalid_locator(message: &'static str) -> ContentFailure {
    ContentFailure::new(ContentFailureReason::InvalidLocator, message)
}
