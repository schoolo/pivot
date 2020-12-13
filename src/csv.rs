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
