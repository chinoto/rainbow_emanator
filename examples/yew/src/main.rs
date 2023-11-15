use gloo::timers::callback::Interval;
use rainbow_emanator::rainbow_emanator;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let rows = use_state(|| 5);
    let columns = use_state(|| 8);
    let speed = use_state(|| 10);
    let ms = use_state(|| 0.);

    {
        let ms = ms.clone();
        let speed = speed.clone();
        use_effect(|| {
            let interval = Interval::new(33, move || {
                ms.set(*ms + (*speed as f32) * 3.3);
            });
            || {
                interval.cancel();
            }
        });
    }

    let [row_change, column_change, speed_change] = [&rows, &columns, &speed].map(|state| {
        let state = state.clone();
        move |e: InputEvent| {
            let elem = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let new_value = elem.value().parse::<u32>().unwrap_or(*state).max(1);
            state.set(new_value);
        }
    });

    let mut rainbow_iter = rainbow_emanator(*columns, *rows, *ms);

    html! {
        <>
            <form id="settings" action="javascript: void 0;">
                {[
                    ("Rows", row_change, *rows),
                    ("Columns", column_change, *columns),
                    ("Speed", speed_change, *speed)
                ].into_iter().map(|(label,handler,value)| html! {
                    <label>
                        <span>{label}{":\u{a0}"}</span>
                        <span><input type="number" oninput={handler} value={value.to_string()} /></span>
                    </label>
                }).collect::<Html>()}
            </form>

            <table><tbody>
                {(0..*rows).map(|_| html! {
                    <tr>
                        {(0..*columns).map(|_| {
                            // TODO: change rainbow_emanator to return floats so the user can choose the scaling.
                            let [r, g, b] = rainbow_iter.next().unwrap().map(|x| x.saturating_mul(128));
                            let color = format!("#{r:02x}{g:02x}{b:02x}");
                            html! {<td bgcolor={color}></td>}
                        }).collect::<Html>()}
                    </tr>
                }).collect::<Html>()}
            </tbody></table>
        </>
    }
}
