# Audio Event, Bank, Material Sound, and Mix Reference

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **API reference**.

---

# V34 audio packet and graph reference closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Audio event descriptor

| Field | Meaning |
|---|---|
| `audio_event_id` | Stable event identity. |
| `event_family` | footstep, impact, scrape, break, weather, voice, UI, ambient, weapon. |
| `layers` | Layer ids and playback behavior. |
| `variation_set_id` | Weighted/stochastic variation. |
| `material_row_ref` | Optional material sound row. |
| `bus_route` | Bus/mix destination. |
| `occlusion_policy` | none, simple, portal, zone. |
| `bank_ref` | Cooked bank package. |
| `fallback_event_ref` | Legal fallback. |

## Audio failure codes

- `audio.bank.missing`
- `audio.sample.decode_failed`
- `audio.event.unbound`
- `audio.material_row_missing`
- `audio.bus_route_missing`
- `audio.device_unavailable`
- `audio.occlusion_portal_invalid`
- `audio.audition.no_output`
