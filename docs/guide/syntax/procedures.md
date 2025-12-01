# Procedures (My Blocks)

Procedures (also known as "My Blocks" in Scratch) allow you to create custom blocks to reuse code.

## Defining a Procedure

Use the `proc` keyword to define a new procedure. You can define input parameters with types.

```rust
proc jump(height: number) {
    change_y_by(height);
    wait(0.1);
    change_y_by(0 - height);
}
```

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
proc jump(height: number) {
    change_y_by(height);
    wait(0.1);
    change_y_by(0 - height);
}
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
define jump (height)
change y by (height)
wait (0.1) seconds
change y by ((0) - (height))
</pre>
</div>
</div>

## Calling a Procedure

You can call a procedure just like any other function.

```rust
jump(10);
```

<pre class="blocks">
jump (10)
</pre>

## Parameter Types

Supported types for parameters:
- `number`: Number input
- `string`: String input
- `boolean`: Boolean input

```rust
proc log_message(msg: string, is_error: boolean) {
    if is_error {
        say(msg);
    }
}
```

<pre class="blocks">
define log_message (msg) &lt;is_error&gt;
if &lt;is_error&gt; then
    say (msg)
end
</pre>

<style>
.comparison {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    margin-bottom: 20px;
    align-items: start;
}
.comparison h4 {
    margin-top: 0;
}
</style>
