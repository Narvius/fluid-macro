# fluid-macros

A macro that allows you to write long method call chains as a series of steps instead, with support for sub-scopes.

# Basic Usage

```rust
let x = fluid!("123", {
    parse::<i32>();
    unwrap_or_default();
    clamp(5, 100);
    to_string();
});
```

This is equivalent to writing:

```rust
let x = "123".parse::<i32>().unwrap_or_default().clamp(5, 100).to_string();
```

# (Motivating) Example

I was working on a little Visual Novel-like module for a game, with a DSL. I ended up creating a bunch of builders that modeled that DSL:

```rust
SceneBuilder::new().with_character(Cid::Player, |b| {
    b.message("Oh, what's this, a loose brick?")
        .choice("Should I push it?", |b| {
            b.branch("Yes", |b| { /* omitted */ })
                .branch("No", |b| { /* omitted */ })
        })
})
```

While the overall structure is fine, there's a lot of line noise--periods, `|b|`s, etc. Not to mention that the otherwise-lovely `rustfmt` really struggles with the nested builders. Thus I wrote this macro to be able to write this instead:

```rust
fluid!(SceneBuilder::new(), {
    with_character(Cid::Player) {
        message("Oh, what's this, a loose brick?");
        choice("Should I push it?") {
            branch("Yes") {
                /* omitted */
            }
            branch("No") {
                /* omitted */
            }
        }
    }
})
```

# Enabling nested blocks

Since the macro simply moves around the identifiers it is given, any method that matches after all transformations are done will automatically work. There is really only one assumption made: Blocks (like the `choice` call in the example above) are simply a **a one-argument closure as a final argument**. An example is given in `multiplied` below:

```rust
#[derive(Default)]
struct Builder {
    total: i32,
}

impl Builder {
    fn add(mut self, amount: i32) -> Self {
        self.total += amount;
        self
    }

    fn multiplied(mut self, multiplier: i32, f: impl FnOnce(SubBuilder) -> SubBuilder) -> Self {
        f(SubBuilder {
            parent: &mut self,
            multiplier,
        });
        self
    }
}

struct SubBuilder<'a> {
    parent: &'a mut Builder,
    multiplier: i32,
}

impl SubBuilder<'_> {
    fn add(mut self, amount: i32) -> Self {
        self.parent.total += amount * self.multiplier;
        self
    }
}
```

```rust
let b = fluid!(Builder::default(), {
    add(5);
    multiplied(3) {
        add(4);
    }
});

assert_eq!(b.total, 17);
```

It can, of course, be any closure type--not just `impl FnOnce`.

For completeness, the above macro invocation expands to the following:

```rust
let b = Builder::default()
    .add(5)
    .multiplied(3, |b| {
        b.add(4)
    });
```

# Known limitations

It's not very friendly to the IDE whilst writing. You will have to already know the names of methods you want to use. After compilation, however, symbol lookup and the like works fine.