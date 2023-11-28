use crate::app::{App, Msg};
use crate::input::Input;

use yew::prelude::*;

/// Renders component view.
pub fn view(app: &App, ctx: &Context<App>) -> Html {
    // context link
    let link = ctx.link().clone();

    // html view
    html! {
        <main id="app">
            <div id="boxes">
                <div class="box">
                    <h2>{"Bandsaw"}</h2>

                    // sheet csv input
                    <Input
                        name="Input Google Sheet"
                        on_data={link.callback(Msg::SheetInput)}
                    />

                    // canvas csv input
                    <Input
                        name="Input Canvas IDs"
                        on_data={link.callback(Msg::CanvasInput)}
                    />

                    // csv output
                    <button
                        disabled={app.records.is_none() || app.student_ids.is_none()}
                        onclick={link.callback(|_| Msg::Output)}
                    >
                        {"Output Rubric CSV"}
                    </button>
                </div>

                if let Some(names) = &app.names {
                    <div class="box">
                        <h2>{"Students"}</h2>

                        <ul>
                            if let Some(student_ids) = &app.student_ids {
                                // show student names and ids
                                {names
                                    .iter()
                                    .map(|name| html! {
                                        <li>
                                            {format!("{} ({})", name, student_ids[&name.clone()].clone())}
                                        </li>
                                    })
                                    .collect::<Vec<_>>()
                                }
                            } else {
                                // show student names
                                {names
                                    .iter()
                                    .map(|name| html! { <li>{name}</li> })
                                    .collect::<Vec<_>>()
                                }
                            }
                        </ul>
                    </div>
                }
            </div>
        </main>
    }
}
