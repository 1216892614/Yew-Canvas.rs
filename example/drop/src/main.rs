use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yew_canvas::{Canvas, WithRander};

//Befor impl WithRander, derive Clone and PartialEq first!
#[derive(Clone, PartialEq)]
struct Rander();

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        let interface: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        interface.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        interface.set_fill_style(&JsValue::from_str("#fe5c5a"));
        interface.set_font("100px sans-serif");
        interface.set_text_baseline("top");

        let text = "🐟SAKARA🐟";

        let text_metrics = interface.measure_text(text).unwrap();
        let (actual_bounding_box_descent, font_bounding_box_descent, width) = (
            text_metrics.actual_bounding_box_descent(),
            text_metrics.font_bounding_box_descent(),
            text_metrics.width(),
        );

        let text_pos = (100.0, 100.0);

        interface.fill_text(text, text_pos.0, text_pos.1).unwrap();
        interface.set_stroke_style(&JsValue::from_str("red"));
        interface.stroke_rect(text_pos.0, text_pos.1, width, actual_bounding_box_descent);

        interface.set_stroke_style(&JsValue::from_str("green"));
        interface.stroke_rect(text_pos.0, text_pos.1, width, font_bounding_box_descent)
    }
}

#[function_component(WillDrop)]
pub fn will_drop() -> Html {
    html!(
        <Canvas<CanvasRenderingContext2d, Rander>
            //Just use style, canvas can suit automaticly.
            style="
                width: 2000px;
                height: 200px;
            "
            rander={Box::new(Rander())}
        />
    )
}

#[function_component(App)]
pub fn app() -> Html {
    let drop_state = use_state(|| html!(<WillDrop />));

    let onclick = {
        let drop_state = drop_state.clone();
        Callback::from(move |_| drop_state.set(html!(<a>{"SAKARA is droped!"}</a>)))
    };

    html!(
        <>
            <button {onclick} >
                {"Drop the SAKARA!"}
            </button>

            {
                (*drop_state).clone()
            }
        </>
    )
}

fn main() {
    yew::start_app::<App>();
}
