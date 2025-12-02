# Control Blocks Status

| Block | Opcode | Status | Notes |
| :--- | :--- | :---: | :--- |
| Wait (secs) | `control_wait` | ✅ | |
| Repeat (times) | `control_repeat` | ✅ | |
| Forever | `control_forever` | ✅ | |
| If (condition) Then | `control_if` | ✅ | |
| If (condition) Then ... Else | `control_if_else` | ✅ | |
| Wait Until (condition) | `control_wait_until` | ✅ | |
| Repeat Until (condition) | `control_repeat_until` | ✅ | |
| Stop (all/this/other) | `control_stop` | ✅ | |
| When I Start as a Clone | `control_start_as_clone` | ✅ | Via `#[on_clone_start]` attribute |
| Create Clone of (sprite) | `control_create_clone_of` | ✅ | |
| Delete This Clone | `control_delete_this_clone` | ✅ | |
