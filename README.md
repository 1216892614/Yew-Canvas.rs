# ✨Yew-Canvas.rs✨
[中文](https://github.com/1216892614/Yew-Canvas.rs/blob/main/README-zh.md)

**Now support Yew 2.21 !**

`Yew-Canvas.rs` is a simple Canvas component for Yew.

U can easily create a canvas with the context u need.

# 📑How to use it?📑
Just 3 Simple steps, if the context u need is `CanvasRenderingContext2d`, do like this:

1. Get `Yew-Canvas.rs`, `HtmlCanvasElement` and context type!
    ```toml
    #Cargo.toml
    [dependencies]
    yew="0.21"
    yew-canvas="..."

    [dependencies.web-sys]
    version = "0.3"
    features = ["HtmlCanvasElement", "CanvasRenderingContext2d"]
    ```

1. Create a Rander struct!
    ```rust
    #[derive(Clone, PartialEq)]
    struct Rander();

    impl WithRander for Rander {
        fn rand(self, canvas: &HtmlCanvasElement) {
            
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
        >
            {"The browser is not supported."}
        </Canvas<CanvasRenderingContext2d, Rander>>
    )
    ```

# 👌Run Exmple👌

0. *This requires you to set up the `Yew.rs` development environment in advance, the following is a `Trunk` packaging example:
1. `cd ./examples/base-use`
2. `trunk serve`

# ⚖️License⚖️

`Yew-Canvas.rs` is dual licensed under the MIT license and the Apache License (Version 2.0).
