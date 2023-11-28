use crate::utils;
use crate::app::{App, Msg, StudentName};

use std::collections::HashMap;

use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;

use csv::Writer;

use anyhow::{Result, Context};

/// Responds to messages from component.
pub fn update(app: &mut App, _ctx: &yew::Context<App>, msg: <App as BaseComponent>::Message) -> Result<bool> {
    match msg {
        // sheet records
        Msg::SheetInput(records) => {
            // strip first row
            app.records = Some(records[1..].to_vec());

            // student names
            app.names = Some(
                app.records.as_ref().unwrap()[0]
                    .iter()
                    .map(String::from)
                    .collect::<Vec<StudentName>>()[2..]
                    .to_vec()
            );
        }

        // canvas records
        Msg::CanvasInput(records) => {
            let mut student_ids = HashMap::new();

            for row in records[2..].iter() {
                // split name
                let name =
                    row.get(0)
                        .context("error occurred when accessing student name")?
                        .split(", ")
                        .collect::<Vec<&str>>();

                // associate id with name
                student_ids.insert(
                    format!(
                        "{} {}",
                        name.get(1)
                            .context("error occurred when accessing last name")?,
                        name.get(0)
                            .context("error occurred when accessing first name")?,
                    ),

                    row.get(1)
                        .context("error occurred when accessing student id")?
                        .to_string(),
                );
            }

            app.student_ids = Some(student_ids);
        }

        // output csv
        Msg::Output => {
            let records = app.records.as_ref()
                .context("error occurred when accessing records")?;

            let names = app.names.as_ref()
                .context("error occurred when accessing student names")?;

            let student_ids = app.student_ids.as_ref()
                .context("error occurred when accessing student ids")?;

            // output data
            let data = [
                vec![utils::data_headers(records)],
                utils::format_data(records, names, student_ids)
            ].concat();

            // csv writer
            let mut writer = Writer::from_writer(vec![]);

            // write records
            for row in data {
                writer.write_record(row)
                    .context("error occurred when writing record")?;
            }

            // csv string
            let data = String::from_utf8(
                writer.into_inner()
                    .context("error occurred when converting to buffer")?
            ).context("error occurred when converting buffer to utf8")?;

            let document =
                web_sys::window()
                    .context("error occurred when accessing window")?
                    .document()
                    .context("error occurred when accessing documetn")?;

            let anchor: HtmlAnchorElement =
                document
                    .create_element("a")
                    .expect("error occurred when creating anchor element")
                    .dyn_into()
                    .unwrap();

            // set attributes
            anchor.set_download("export.csv");
            anchor.set_href(
                &utils::blob_url(data)
                    .expect("error occurred when creating blob url")
            );

            // prompt download
            anchor.click();
        }
    }

    Ok(true)
}
