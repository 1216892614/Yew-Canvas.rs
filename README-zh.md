# ✨Yew-Canvas.rs✨
[English](./README.md)

`Yew-Canvas.rs` 是一个简单封装 Canvas 得到的 Yew 组件.

你可以简单的创建基于任何你需要的上下文的 canvas.

# 📑如何使用?📑
使用 `Yew-Canvas.rs` 只需要简单的三步, 下面以 `CanvasRenderingContext2d` 作为上下文举例:

1. 把 `Yew-Canvas.rs` 和你需要的上下文类型以及 `HtmlCanvasElement` 添加依赖!
    ```toml
    #Cargo.toml
    [dependencies]
    yew="0.19"
    yew-canvas="..."

    [dependencies.web-sys]
    version = "0.3.59"
    features = ["HtmlCanvasElement", "CanvasRenderingContext2d"]
    ```

1. 创建一个渲染器(Rander)结构体!
    ```rust
    #[derive(Clone, PartialEq)]
    struct Rander();

    impl WithRander for Rander {
        fn rand(self, canvas: &HtmlCanvasElement) {
            
            //...
        }
    }
    ```

1. 创建 `Canvas` 组件!
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
            {"The browser is not supported."}
        </Canvas<CanvasRenderingContext2d, Rander>>
    )
    ```

# 👌运行实例👌

0. *这一步需要你配置完成 `Yew.rs` 的开发环境, 下面以 `Trunk` 打包器举例:
1. `cd ./example/base-use`
2. `trunk server`

# ⚖️开源协议⚖️

`Yew-Canvas.rs` 是基于 MIT license 和 the Apache License (Version 2.0) 双协议开源, 使用其中你喜欢的协议来进行开发吧!