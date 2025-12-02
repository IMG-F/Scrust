# Motion Blocks Status

| Block | Opcode | Status | Notes |
| :--- | :--- | :---: | :--- |
| Move (steps) Steps | `motion_movesteps` | ✅ | |
| Turn Right (degrees) | `motion_turnright` | ✅ | |
| Turn Left (degrees) | `motion_turnleft` | ✅ | |
| Go To (random/mouse) | `motion_goto` | ❌ | Needs random/mouse menu support |
| Go To X: () Y: () | `motion_gotoxy` | ✅ | |
| Glide (secs) to (random/mouse) | `motion_glideto` | ❌ | |
| Glide (secs) to X: () Y: () | `motion_glidesecstoxy` | ✅ | |
| Point in Direction (dir) | `motion_pointindirection` | ✅ | |
| Point Towards (mouse/sprite) | `motion_pointtowards` | ❌ | |
| Change X By (dx) | `motion_changexby` | ✅ | |
| Set X To (x) | `motion_setx` | ✅ | |
| Change Y By (dy) | `motion_changeyby` | ✅ | |
| Set Y To (y) | `motion_sety` | ✅ | |
| If on Edge, Bounce | `motion_ifonedgebounce` | ✅ | |
| Set Rotation Style | `motion_setrotationstyle` | ✅ | |
| X Position | `motion_xposition` | ❌ | `motion_yposition` is implemented, this seems missing in map_call |
| Y Position | `motion_yposition` | ✅ | |
| Direction | `motion_direction` | ❌ | |
