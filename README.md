# fluid-macro

A macro that allows you to write long method call chains as a series of steps instead, with support for sub-scopes.

## Basic Usage

```rust
let x = fluid!("123", {
    parse::<i32>();
    unwrap_or_default();
    [- 100];
    [* 2];
    clamp(5, 100);
    to_string();
});
```

This is equivalent to writing:

```rust
let x = (("123".parse::<i32>().unwrap_or_default() - 100) * 2).clamp(5, 100).to_string();
```

## (Motivating) Example

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

## Known limitations

It's not very friendly to the IDE whilst writing. You will have to already know the names of methods you want to use. After compilation, however, symbol lookup and the like works fine.