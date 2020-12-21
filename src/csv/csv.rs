use std::collections::HashMap;
use std::fs;

type Rows = Vec<HashMap<String, String>>;

#[derive(Debug, PartialEq)]
pub struct Csv {
    cols: Vec<String>,
    rows: Rows,
}

impl Csv {
    pub fn new(cols: Vec<String>, rows: Vec<Vec<String>>) -> Option<Csv> {
        let mut res_rows = vec![] as Vec<HashMap<String, String>>;
        for row in rows {
            assert!(row.len() == cols.len());
            let mut rmap = HashMap::new();
            for (idx, field) in row.iter().enumerate() {
                rmap.insert(cols.get(idx).cloned()?.to_string(), field.to_string());
            }
            res_rows.push(rmap);
        }

        Some(Csv {
            cols: cols,
            rows: res_rows,
        })
    }
    pub fn parse(s: String) -> Option<Csv> {
        let mut lines = s.lines();
        let cols: Vec<String> = lines.next()?.split(",").map(|x| x.to_string()).collect();

        let mut rows = vec![] as Vec<Vec<String>>;
        lines
            .map(|row| row.clone().split(","))
            .for_each(|fields| rows.push(fields.map(|x| x.to_string()).collect()));

        Csv::new(cols.to_owned(), rows)
    }
    pub fn from_file(path: &str) -> Result<Option<Csv>, std::io::Error> {
        let contents = fs::read_to_string(path)?;

        Ok(Csv::parse(contents))
    }
    pub fn rows(&self) -> &Rows {
        &self.rows
    }
}

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
