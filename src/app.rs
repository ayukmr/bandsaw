use crate::{update, view};

use yew::prelude::*;

use std::collections::HashMap;
use csv::StringRecord;

pub type StudentId     = String;
pub type StudentName   = String;
pub type StudentValue  = String;

// app state
pub struct App {
    pub records: Option<Vec<StringRecord>>,

    pub names: Option<Vec<StudentName>>,
    pub student_ids: Option<HashMap<StudentName, StudentId>>,
}

// update messages
pub enum Msg {
    Output,
    SheetInput(Vec<StringRecord>),
    CanvasInput(Vec<StringRecord>),
}

// app component
impl Component for App {
    type Message = Msg;
    type Properties = ();

    // create component
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            records: None,

            names:       None,
            student_ids: None,
        }
    }

    // update messages
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        update::update(self, ctx, msg)
            .expect("error occured when updating state")
    }

    // component view
    fn view(&self, ctx: &Context<Self>) -> Html {
        view::view(self, ctx)
    }
}
