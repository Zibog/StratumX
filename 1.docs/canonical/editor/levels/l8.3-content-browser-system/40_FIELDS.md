# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| content_browser_view_id | ContentBrowserViewId | active browser view identity | unique per browser host |
| asset_listing_ref | AssetListingRef | current asset listing backing the browser | must resolve through index/artifact surfaces |
| filter_state | AssetFilterState | active asset filters and search | editor-local and explicit |
| selection_ref_set | AssetSelectionRefSet | currently selected assets | bounded and publishable |
| action_availability_set | AssetActionAvailabilitySet | legal right-click actions for selected assets | must remain command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `content_browser_system` without stealing truth from neighboring levels or lower packages.
