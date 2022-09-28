use gloo::{events::EventListener, utils::window};
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::{html::ChildrenRenderer, prelude::*};

/// A Canvas component is encapsulated.
///
/// # Parameters and types
/// ```ignore
/// <Canvas<...1, ...2>
///    style="
///        ...3
///    "
///    rander={Box::new(...4)}
/// />
/// ```
/// **...1:** The canvas context u need.
///
/// **...2:** struct you impl`yew_canvas::WithRander`.
///
/// **...3:** Just use style, canvas can suit automaticly.
///
/// **...4:** U have to wrap ur `yew_canvas::WithRander` struct in `Box<>`.
///
/// # Example
///
/// ```ignore
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html!(
///         <Canvas<CanvasRenderingContext2d, Rander>
///             //Just use style, canvas can suit automaticly.
///             style="
///                 width: 100%;
///                 height: 100%;
///             "
///             rander={Box::new(Rander())}
///         />
///             {"The browser is not supported."}
///         </Canvas<CanvasRenderingContext2d, Rander>>
///     )
/// }
/// ```
#[function_component(Canvas)]
pub fn canvas<CanvasContext, T>(props: &Props<T>) -> Html
where
    T: PartialEq + WithRander + Clone + 'static,
    CanvasContext: JsCast,
{
    let node_ref = NodeRef::default();
    let is_first_rander = use_state(|| true);
    let style = props.style.clone().unwrap_or(String::new());
    let display_size = use_state(|| (300, 150));

    let size_listen_enent_state = use_state(|| EventListener::new(&window(), "resize", |_| ()));

    {
        let node_ref = node_ref.clone();
        let display_size = display_size.clone();
        let rander = props.rander.clone();

        use_effect(move || {
            let canvas = node_ref.cast::<HtmlCanvasElement>().unwrap();

            if *is_first_rander {
                is_first_rander.set(false);
                let canvas = canvas.clone();

                display_size.set((canvas.client_width(), canvas.client_height()));

                size_listen_enent_state.set(EventListener::new(&window(), "resize", move |_| {
                    display_size.set((canvas.client_width(), canvas.client_height()));
                }));
            }

            rander.rand(&canvas);

            || ()
        });
    }

    let children = props
        .children
        .clone()
        .unwrap_or(ChildrenRenderer::default());

    html! {
    <canvas
        style={style}
        width={display_size.clone().deref().0.to_string()}
        height={display_size.clone().deref().1.to_string()}
        ref={node_ref}
    >
    { for children.iter() }
    </ canvas>
    }
}

/// Implement this trait for rendering.
///
/// use `&self` to pass data.
///
/// # example
/// ```ignore
/// #[derive(Clone, PartialEq)]
///struct Rander();
///
///impl WithRander for Rander {
///    fn rand(self, canvas: &HtmlCanvasElement) {
///    // CanvasRenderingContext2d can be
///    // any kind of canvas context.
///    // Make sure that, it's the same
///    // context as Canvas component.
///        let interface: CanvasRenderingContext2d = canvas
///            .get_context("2d")
///            .unwrap()
///            .unwrap()
///            .dyn_into()
///            .unwrap();
///    ...
/// ```
pub trait WithRander: Clone + PartialEq {
    fn rand(self, canvas: &HtmlCanvasElement);
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: PartialEq> {
    pub rander: Box<T>,
    pub children: Option<Children>,
    pub style: Option<String>,
}
