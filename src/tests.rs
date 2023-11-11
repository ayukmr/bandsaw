#[cfg(test)]
mod tests {
    use crate::utils::format_data;
    use crate::utils::data_headers;

    use std::collections::HashMap;
    use csv::StringRecord;

    #[test]
    // test output data
    fn test_format_data() {
        // input records
        let records = vec![
            StringRecord::from(vec!["yellow shading = reassessed", "Student name",  "First-Name Last-Name"]),
            StringRecord::from(vec!["Content and Knowledge",       "Standard Name", "1"]),
            StringRecord::from(vec!["Skills and Practices",        "Standard Name", "2"]),
            StringRecord::from(vec!["Habits of Learning",          "Standard Name", "3"]),
        ];

        // input names
        let names = vec![String::from("First-Name Last-Name")];

        // student ids
        let mut student_ids = HashMap::new();
        student_ids.insert(String::from("First-Name Last-Name"), String::from("ID"));

        // generate data
        let result = format_data(&records, &names, &student_ids);

        // test data
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ["First-Name Last-Name", "ID", "", "1", "1", "2", "2", "3", "3"]);
    }

    #[test]
    // test output headers
    fn test_data_headers() {
        // input records
        let records = vec![
            StringRecord::from(vec!["yellow shading = reassessed", "Student name",    "First-Name Last-Name"]),
            StringRecord::from(vec!["Content and Knowledge",       "Standard Name 1", "1"]),
            StringRecord::from(vec!["Skills and Practices",        "Standard Name 2", "2"]),
            StringRecord::from(vec!["Habits of Learning",          "Standard Name 3", "3"]),
        ];

        // generate headers
        let result = data_headers(&records);

        // test headers
        assert_eq!(result.len(), 9);
        assert_eq!(
            result,
            [
                "Student name", "Student ID", "Student SIS ID",
                "A. Content and Knowledge > 1. Standard Name 1 result", "A. Content and Knowledge > 1. Standard Name 1 mastery points",
                "A. Skills and Practices > 1. Standard Name 2 result", "A. Skills and Practices > 1. Standard Name 2 mastery points",
                "A. Habits of Learning > 1. Standard Name 3 result", "A. Habits of Learning > 1. Standard Name 3 mastery points"
            ]
        );
    }
}
