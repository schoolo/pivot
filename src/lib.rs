mod csv;

#[cfg(test)]
mod test_csv {
    use crate::csv::*;
    #[test]
    fn test_empty() {
        let cols = vec![];
        let rows = vec![];
        let csv = Csv::new(cols.clone(), rows.clone()).unwrap();
        assert_eq!(csv.rows().len(), 0)
    }
    #[test]
    fn test_simple() {
        let cols = vec!["first_name".to_string(), "last_name".to_string()];
        let rows = vec![vec!["nick".to_string(), "pascullo".to_string()]];
        let csv1 = Csv::new(cols.clone(), rows.clone());
        let csv2 = Csv::new(cols.clone(), rows.clone());
        assert_eq!(csv1, csv2);
    }
    #[test]
    fn test_simple_rows() {
        let cols = vec!["first_name".to_string(), "last_name".to_string()];
        let rows = vec![vec!["nick".to_string(), "pascullo".to_string()]];
        let csv = Csv::new(cols.clone(), rows.clone()).unwrap();
        assert_eq!(csv.rows().len(), 1);

        let first_row = &csv.rows()[0];
        assert_eq!(first_row.get("first_name").unwrap(), "nick");
        assert_eq!(first_row.get("last_name").unwrap(), "pascullo");
    }

    // Error tests
    #[test]
    #[should_panic]
    fn test_wrong_row_length() {
        let cols = vec!["first_name".to_string(), "last_name".to_string()];
        let rows = vec![vec!["nick".to_string()]];
        let _csv = Csv::new(cols.clone(), rows.clone()).unwrap();
    }
    #[test]
    fn test_nonexistent_file() {
        let csv = Csv::from_file("foo/bar/nonexistent.csv").map_err(|e| e.kind());
        assert_eq!(csv, Err(std::io::ErrorKind::NotFound));
    }
}
