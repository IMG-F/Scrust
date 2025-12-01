# Variables & Lists

State management in Scrust is handled through typed variables and lists.

## Variables

Variables in Scrust are declared using the `var` keyword. You must specify the visibility of the variable.

### Visibility

There are two types of visibility:

1.  **Public (`public`)**: Corresponds to "For all sprites" (Global).
2.  **Private (`private`)**: Corresponds to "For this sprite only" (Local).

::: warning Stage Restrictions
Variables declared in the **Stage** (`stage.sr`) MUST be `public`. The Stage cannot have private variables.
:::

### Syntax

```rust
// Syntax: [visibility] var NAME = VALUE;

// Global variable (All sprites can see this)
public var SCORE = 0;

// Local variable (Only this sprite sees this)
private var HP = 100;
```

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
public var SCORE = 0;
private var HP = 100;
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
(SCORE)
(HP)
</pre>
</div>
</div>

### Assigning & Changing

Use standard assignment operators to modify variables.

```rust
// Set variable
SCORE = 0;

// Change variable
SCORE += 1;
```

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
SCORE = 0;
SCORE += 1;
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
set [SCORE v] to (0)
change [SCORE v] by (1)
</pre>
</div>
</div>

## Lists

Lists are arrays of values. In Scrust, lists are dynamic and can hold numbers or strings.

### Declaration

Like variables, lists have visibility. The type is specified using the `list` keyword.

```rust
// Syntax: [visibility] list NAME = [INITIAL_VALUES];

public list HIGHSCORES = [10, 20, 30];
private list INVENTORY = [];
```

<div class="comparison">
<div>
<h4>Scrust</h4>

```rust
public list HIGHSCORES = [10, 20, 30];
```
</div>
<div>
<h4>Scratch</h4>
<pre class="blocks">
(HIGHSCORES :: list)
</pre>
</div>
</div>

### List Operations

Scrust provides standard functions to manipulate lists.

| Operation | Scrust Syntax | Scratch Block |
| :--- | :--- | :--- |
| Add Item | `add_to_list("LIST", value);` | `add (value) to [LIST v]` |
| Delete Item | `delete_of_list("LIST", index);` | `delete (index) of [LIST v]` |
| Delete All | `delete_all_of_list("LIST");` | `delete all of [LIST v]` |
| Insert | `insert_at_list("LIST", index, value);` | `insert (value) at (index) of [LIST v]` |
| Replace | `replace_item_of_list("LIST", index, value);` | `replace item (index) of [LIST v] with (value)` |
| Get Item | `item_of_list("LIST", index)` | `item (index) of [LIST v]` |
| Length | `length_of_list("LIST")` | `length of [LIST v]` |
| Contains | `list_contains("LIST", value)` | `[LIST v] contains (value)?` |

#### Example

```rust
add_to_list("HIGHSCORES", SCORE);
if length_of_list("HIGHSCORES") > 5 {
    delete_of_list("HIGHSCORES", 1);
}
```

<pre class="blocks">
add (SCORE) to [HIGHSCORES v]
if <(length of [HIGHSCORES v]) > (5)> then
    delete (1) of [HIGHSCORES v]
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
