use crate::app::{StudentName, StudentId, StudentValue};

use wasm_bindgen::JsValue;
use web_sys::{Blob, BlobPropertyBag, Url};

use std::collections::HashMap;
use csv::StringRecord;

use anyhow::Result;

// format input
pub fn format_data(
    records:     &[StringRecord],
    names:       &[StudentName],
    student_ids: &HashMap<StudentName, StudentId>
) -> Vec<Vec<StudentValue>> {
    if records.is_empty() {
        panic!("error when accessing first record");
    }

    (2..records[0].len()).map(|col| {
        let mut values: Vec<StudentValue> = records
            .iter()
            .flat_map(|row| {
                let val = row.get(col)
                    .unwrap_or_else(|| panic!("error when getting student data at {col}"))
                    .to_string();

                if val.parse::<u8>().is_ok() {
                    // add mastery points
                    vec![val.clone(), val]
                } else {
                    vec![val]
                }
            })
            .collect();

        // extra values
        values.insert(1, student_ids[&names[col - 2]].clone());
        values.insert(2, String::new());

        values
    }).collect()
}

// column headers
pub fn data_headers(records: &[StringRecord]) -> Vec<String> {
    // number of headers
    let num_headers = records.get(1).unwrap().iter().fold(0, |acc, val| {
        if val.parse::<f32>().is_err() {
            acc + 1
        } else {
            acc
        }
    });

    let mut headers: Vec<String> =
        records.iter().enumerate().flat_map(|(idx, row)| {
            if idx > 0 {
                let header = row.get(0).expect("error occurred when getting header");

                // compress headers
                let formatted = (1..num_headers)
                    .fold(format!("A. {header}"), |acc, idx| {
                        let header = row.get(idx).expect("error occurred when getting header");
                        format!("{acc} > 1. {header}")
                    });

                vec![
                    // result and mastery points
                    format!("{formatted} result"),
                    format!("{formatted} mastery points"),
                ]
            } else {
                vec![
                    row.get(1)
                        .expect("error occurred when getting student name header")
                        .to_string()
                ]
            }
        }).collect();

    // extra headers
    headers.insert(1, String::from("Student ID"));
    headers.insert(2, String::from("Student SIS ID"));

    headers
}

// create blob url
pub fn blob_url(data: String) -> Result<String, JsValue> {
    // js array
    let data_ary = js_sys::Array::new();
    data_ary.push(&data.into());

    // create blob
    let blob = Blob::new_with_u8_array_sequence_and_options(
        &JsValue::from(data_ary),
        BlobPropertyBag::new().type_("text/csv;charset=utf-8;"),
    )?;

    Url::create_object_url_with_blob(&blob)
}
