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

## Other Events

*Note: More events will be supported in future versions.*

| Attribute | Scratch Block | Notes |
| :--- | :--- | :--- |
| `#[on_flag_clicked]` | `when flag clicked` | |
| `#[on_key_pressed("KEY")]` | `when [KEY] key pressed` | Supports "space", "up arrow", "a", etc. |
| `#[on_clone_start]` | `when I start as a clone` | See [Control Flow](./control-flow) for creating clones. |
