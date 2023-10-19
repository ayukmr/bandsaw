use yew::prelude::*;

use gloo::file::File;
use gloo::file::callbacks::FileReader;

use web_sys::HtmlInputElement;
use csv::{ReaderBuilder, StringRecord};

// input state
pub struct Input {
    pub input:  NodeRef,
    pub reader: Option<FileReader>,
}

// update messages
pub enum Msg {
    Click,
    FileInput(Event),
    Convert(String)
}

// input props
#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub on_data: Callback<Vec<StringRecord>>,
}

// app component
impl Component for Input {
    type Message    = Msg;
    type Properties = Props;

    // create component
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input:   NodeRef::default(),
            reader:  None,
        }
    }

    // update messages
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::Click => {
                // input file
                self.input.cast::<HtmlInputElement>().unwrap().click();
            }

            Msg::FileInput(event) => {
                // input target
                let target: HtmlInputElement = event.target_unchecked_into();

                // input file
                let file = File::from(
                    target.files()
                        .unwrap().item(0)
                        .expect("error occurred when accessing input file")
                );

                // read callback
                let callback = gloo::file::callbacks::read_as_text(&file, move |res| {
                    let data = res
                        .expect("error occurred when reading input file");

                    link.send_message(Msg::Convert(data));
                });

                // add reader to state
                self.reader = Some(callback);
            }

            // convert csv
            Msg::Convert(data) => {
                // drop file reader
                self.reader = None;

                // csv reader
                let mut reader =
                    ReaderBuilder::new()
                        .has_headers(false)
                        .from_reader(data.as_bytes());

                // records as vector
                let records =
                    reader
                        .records()
                        .map(|r| r.unwrap())
                        .collect::<Vec<StringRecord>>()
                        .to_vec();

                // emit callback
                ctx.props().on_data.emit(records);
            }
        }

        true
    }

    // component view
    fn view(&self, ctx: &Context<Self>) -> Html {
        // context link
        let link = ctx.link().clone();

        // html view
        html! {
            <>
                // input button
                <button onclick={link.callback(|_| Msg::Click)}>
                    {&ctx.props().name}
                </button>

                // hidden file input
                <input
                    type="file"
                    accept="text/csv"
                    style="display: none;"

                    ref={&self.input}
                    onchange={link.callback(Msg::FileInput)}
                />
            </>
        }
    }
}
