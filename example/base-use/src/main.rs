use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew_canvas::{WithRander, Canvas};
use yew::prelude::*;

//Befor impl WithRander, derive Clone and PartialEq first!
#[derive(Clone, PartialEq)]
struct Rander();

impl WithRander for Rander {
    fn rand(&self, canvas: &HtmlCanvasElement) {
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

#[function_component(App)]
pub fn app() -> Html {
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
}

fn main() {
    yew::start_app::<App>();
}
