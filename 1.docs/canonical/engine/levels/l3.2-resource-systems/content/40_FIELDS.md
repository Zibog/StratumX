# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| content_descriptor | ContentDescriptor | required | content metadata semantic | owned by `engine_content` |
| content_pack | ContentPack | required | bundled content package | owned by `engine_content` |
| content_locator | ContentLocator | required | content addressing semantic | owned by `engine_content` |
| content_manifest | ContentManifest | required | content inventory binding | owned by `engine_content` |
