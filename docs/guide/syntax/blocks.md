# Standard Blocks

Scrust includes bindings for most standard Scratch blocks.

## Motion

```rust
move_steps(10);
turn_right(15);
turn_left(15);

go_to(0, 0);
go_to_mouse_pointer(); // "go to random position" or specific target

glide(1.0, 0, 0);

point_in_direction(90);
point_towards("Mouse-pointer");

change_x_by(10);
set_x_to(0);
change_y_by(10);
set_y_to(0);

if_on_edge_bounce();
set_rotation_style("left-right");
```

## Looks

```rust
say("Hello!");
say_for("Hello!", 2.0);
think("Hmm...");
think_for("Hmm...", 2.0);

switch_costume_to("costume1");
next_costume();
switch_backdrop_to("backdrop1");
next_backdrop();

change_size_by(10);
set_size_to(100);

show();
hide();

go_to_front_layer();
go_forward_layers(1);
```

## Sound

```rust
start_sound("Meow");
play_sound_until_done("Meow");
stop_all_sounds();

change_volume_by(-10);
set_volume_to(100);
```

## Sensing

```rust
touching("Mouse-pointer");
touching_color("#ff0000");

distance_to("Mouse-pointer");

ask_and_wait("What's your name?");
// answer() retrieves the answer

key_pressed("space");
mouse_down();
mouse_x();
mouse_y();

timer();
reset_timer();

current_year(); // and month, date, day_of_week, hour, minute, second
days_since_2000();
username();
```
