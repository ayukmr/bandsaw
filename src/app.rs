use crate::{update, view};

use yew::prelude::*;

use std::collections::HashMap;
use csv::StringRecord;

/// ID for student.
pub type StudentId = String;

/// Name for student.
pub type StudentName = String;

/// Value for student.
pub type StudentValue = String;

/// Main component for webapp.
pub struct App {
    /// Student records from input.
    pub records: Option<Vec<StringRecord>>,

    /// Student names from input.
    pub names: Option<Vec<StudentName>>,

    /// Student names paired with IDs.
    pub student_ids: Option<HashMap<StudentName, StudentId>>,
}

/// Update messages for component.
pub enum Msg {
    /// Prompts to download output CSV.
    Output,

    /// Responds to Google Sheet data input.
    SheetInput(Vec<StringRecord>),

    /// Responds to Canvas data input.
    CanvasInput(Vec<StringRecord>),
}

impl Component for App {
    /// Update messages for component.
    type Message = Msg;

    /// Properties for component.
    type Properties = ();

    /// Creates component.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            records: None,

            names:       None,
            student_ids: None,
        }
    }

    /// Responds to messages from component by deferring to [`update::update`].
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        update::update(self, ctx, msg)
            .expect("error occured when updating state")
    }

    /// Renders component view by deferring to [`view::view`].
    fn view(&self, ctx: &Context<Self>) -> Html {
        view::view(self, ctx)
    }
}
