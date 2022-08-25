# ✨Yew-Canvas.rs✨
[中文](https://github.com/1216892614/Yew-Canvas.rs/blob/main/README-zh.md)

`Yew-Canvas.rs` is a simple Canvas component for Yew.

U can easily create a canvas with the context u need.

# 📑How to use it?📑
Just 3 Simple steps, if the context u need is `HtmlCanvasElement`, do like this:

1. Get `Yew-Canvas.rs` and context type!
    ```toml
    #Cargo.toml
    [dependencies]
    yew="0.19"
    yew-canvas="0.1"

    [dependencies.web-sys]
    version = "0.3.59"
    features = ["HtmlCanvasElement"]
    ```

1. Create a Rander struct!
    ```rust
    #[derive(Clone, PartialEq)]
    struct Rander();

    impl WithRander for Rander {
        fn rand(&self, canvas: &HtmlCanvasElement) {
            
            //...
        }
    }
    ```

1. Return the component as VNode!
    ```rust
    html!(
        <Canvas<CanvasRenderingContext2d, Rander>
            //Just use style, canvas can suit automaticly.
            style="
                width: 100%;
                height: 100%;
            "
            rander={Box::new(Rander())}
        />
    )
    ```

# 👌Run Exmple👌

0. *This requires you to set up the `Yew.rs` development environment in advance, the following is a `Trunk` packaging example:
1. `cd ./example/base-use`
2. `trunk server`

# ⚖️License⚖️

`Yew-Canvas.rs` is dual licensed under the MIT license and the Apache License (Version 2.0).