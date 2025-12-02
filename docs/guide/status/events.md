# Events Blocks Status

| Block | Opcode | Status | Notes |
| :--- | :--- | :---: | :--- |
| When Green Flag Clicked | `event_whenflagclicked` | ✅ | Via `#[on_flag_clicked]` attribute |
| When Key (key) Pressed | `event_whenkeypressed` | ✅ | Via `#[on_key_pressed("key")]` attribute |
| When this Sprite Clicked | `event_whenthisspriteclicked` | ❌ | |
| When Backdrop Switches to (backdrop) | `event_whenbackdropswitchesto` | ❌ | |
| When (loudness/timer/video) > (val) | `event_whengreaterthan` | ❌ | |
| When I Receive (message) | `event_whenbroadcastreceived` | ❌ | |
| Broadcast (message) | `event_broadcast` | ✅ | |
| Broadcast (message) and Wait | `event_broadcastandwait` | ✅ | |
