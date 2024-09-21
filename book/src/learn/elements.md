# Elements

This is an overview of the elements supported in Freya. 

> For more info check the [API Reference](https://docs.rs/freya/latest/freya/elements/index.html#structs).

### `rect`

[`rect`](https://docs.rs/freya/latest/freya/elements/struct.rect.html) is a generic element that acts as a container for other elements.

You can specify things like **width**, **padding** or even in what **direction** the inner elements are stacked.

Example:

```rust
fn app() -> Element {
    rsx!(
        rect {
            direction: "vertical",
            label { "Hi!" }
        }
    )
}
```

### `label`

[`label`](https://docs.rs/freya/latest/freya/elements/struct.label.html) simply let's you display some text.

Example:

```rust
fn app() -> Element {
    rsx!(
        label {
            "Hello World"
        }
    )
}
```

### `paragraph`

[`paragraph`](https://docs.rs/freya/latest/freya/elements/struct.paragraph.html) element let's you build texts with different styles.

This used used with the `text` element.

Example:

```rust
fn app() -> Element {
    rsx!(
        paragraph {
            text {
                font_size: "15",
                "Hello, "
            }
            text {
                font_size: "30",
                "World!"
            }
        }
    )
}
```

### `image`

[`image`](https://docs.rs/freya/latest/freya/elements/struct.image.html) element let's you show an image.

Example:

```rust
static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn app() -> Element {
    let image_data = static_bytes(RUST_LOGO);
    rsx!(
        image {
            image_data: image_data,
            width: "100%",
            height: "100%",
        }
    )
}
```

### `svg`

[`svg`](https://docs.rs/freya/latest/freya/elements/struct.svg.html) element let's you display an SVG.

Example:

```rust
static FERRIS: &[u8] = static_bytes!("./ferris.svg");

fn app() -> Element {
    let ferris = static_bytes(FERRIS);
    rsx!(
        svg {
            svg_data: ferris,
            width: "100%",
            height: "100%",
        }
    )
}
```