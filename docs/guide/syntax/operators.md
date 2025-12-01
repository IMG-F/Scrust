# Operators

Scrust provides a wide range of mathematical and logical operators.

## Math Operators

Standard arithmetic operators work as expected.

| Scrust | Scratch |
| :--- | :--- |
| `1 + 2` | `(1) + (2)` |
| `1 - 2` | `(1) - (2)` |
| `1 * 2` | `(1) * (2)` |
| `1 / 2` | `(1) / (2)` |
| `random(1, 10)` | `pick random (1) to (10)` |

```rust
let sum = score + 10;
let chance = random(1, 100);
```

## Comparison Operators

Used in `if` statements and loops.

| Scrust | Scratch |
| :--- | :--- |
| `a > b` | `(a) > (b)` |
| `a < b` | `(a) < (b)` |
| `a == b` | `(a) = (b)` |

```rust
if score == 100 {
    // ...
}
```

## Logic Operators

Combine boolean conditions.

| Scrust | Scratch |
| :--- | :--- |
| `a && b` | `&lt;&lt;a&gt; and &lt;b&gt;&gt;` |
| `a || b` | `&lt;&lt;a&gt; or &lt;b&gt;&gt;` |
| `!a` | `&lt;not &lt;a&gt;&gt;` |

```rust
if key_pressed("space") && touching("Ground") {
    jump();
}
```

<pre class="blocks">
if &lt;&lt;key [space v] pressed?&gt; and &lt;touching [Ground v]?&gt;&gt; then
    jump
end
</pre>

## String Operators

| Scrust | Scratch |
| :--- | :--- |
| `join("Hello ", "World")` | `join [Hello ] [World]` |
| `letter_of("Apple", 1)` | `letter (1) of [Apple]` |
| `length_of("Apple")` | `length of [Apple]` |
| `contains("Apple", "a")` | `[Apple] contains [a]?` |

## Math Functions

Common math functions are available.

```rust
mod(10, 3); // Modulo
round(3.5); // Round
abs(-10);   // Absolute value
floor(3.9); // Floor
ceil(3.1);  // Ceiling
sqrt(9);    // Square root
sin(90);    // Sine
cos(0);     // Cosine
```
