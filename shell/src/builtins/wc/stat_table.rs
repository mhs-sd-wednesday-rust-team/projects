use std::fmt::Display;

#[derive(Default)]
pub struct StatTable {
    table: Vec<(Vec<String>, String)>,
    max_width: usize,
}

impl StatTable {
    pub fn add_row(&mut self, path: String, counters: Vec<usize>) {
        let mut table_row = vec![];
        for count in counters {
            let count_str = count.to_string();
            self.max_width = self.max_width.max(count_str.len());
            table_row.push(count_str);
        }
        self.table.push((table_row, path.clone()));
    }
}

impl Display for StatTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (path_stats, path)) in self.table.iter().enumerate() {
            for stat in path_stats {
                write!(f, "{:>#width$} ", stat, width = self.max_width)?;
            }
            write!(f, "{}", path)?;
            if i != self.table.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        static CHECK: &str = r#"   1   11  111 path_1
2222    2   22 p_2"#;

        let mut table = StatTable::default();

        table.add_row("path_1".to_string(), vec![1, 11, 111]);
        table.add_row("p_2".to_string(), vec![2222, 2, 22]);

        assert_eq!(format!("{}", table), CHECK)
    }
}
