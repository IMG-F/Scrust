# Events

Events are the entry points for your scripts. In Scrust, these are represented as functions with specific attributes (decorators).

## Flag Clicked

Runs when the Green Flag is clicked.

```rust
#[on_flag_clicked]
fn start() {
    // Game initialization
}
```

<pre class="blocks">
when flag clicked
</pre>

## Key Pressed

Runs when a specific key is pressed.

```rust
#[on_key_pressed("space")]
fn jump() {
    // Jump logic
}

#[on_key_pressed("right arrow")]
fn move_right() {
    // Move right logic
}
```

<pre class="blocks">
when [space v] key pressed

when [right arrow v] key pressed
</pre>

## Sprite Clicked

Runs when the sprite is clicked.

```rust
#[on_sprite_clicked]
fn on_click() {
    say("You clicked me!");
}
```

<pre class="blocks">
when this sprite clicked
</pre>

## Broadcast Received

Runs when a broadcast message is received.

```rust
#[on_broadcast_received("start_game")]
fn on_start() {
    // ...
}
```

<pre class="blocks">
when I receive [start_game v]
</pre>

## Backdrop Switches

Runs when the backdrop switches to a specific one.

```rust
#[on_backdrop_switches("Level 1")]
fn on_level_1() {
    // ...
}
```

<pre class="blocks">
when backdrop switches to [Level 1 v]
</pre>

## Greater Than (Loudness/Timer)

Runs when a value (loudness or timer) is greater than a threshold.

```rust
#[on_greater_than("LOUDNESS", 10)]
fn on_loud() {
    // ...
}
```

<pre class="blocks">
when [loudness v] > (10)
</pre>

## Cloning Events

Runs when the sprite starts as a clone.

```rust
#[on_clone_start]
fn clone_logic() {
    // Behavior for clones
}
```

<pre class="blocks">
when I start as a clone
</pre>

## Summary

| Attribute | Scratch Block | Notes |
| :--- | :--- | :--- |
| `#[on_flag_clicked]` | <pre class="blocks">when flag clicked</pre> | |
| `#[on_key_pressed("KEY")]` | <pre class="blocks">when [KEY v] key pressed</pre> | Supports "space", "up arrow", "a", etc. |
| `#[on_sprite_clicked]` | <pre class="blocks">when this sprite clicked</pre> | |
| `#[on_broadcast_received("MSG")]` | <pre class="blocks">when I receive [MSG v]</pre> | |
| `#[on_backdrop_switches("BG")]` | <pre class="blocks">when backdrop switches to [BG v]</pre> | |
| `#[on_greater_than("VAR", VAL)]` | <pre class="blocks">when [VAR v] > (VAL)</pre> | VAR can be "LOUDNESS" or "TIMER" |
| `#[on_clone_start]` | <pre class="blocks">when I start as a clone</pre> | |
