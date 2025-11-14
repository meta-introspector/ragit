use super::column::{Column, ColumnAlignment};
//use std::io::{self, Write};

pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Table { columns, rows: Vec::new() }
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn add_row(&mut self, row_data: Vec<String>) {
        self.rows.push(row_data);
    }

    pub fn print(&self) {
        self.print_header();
        for row in &self.rows {
            self.print_row(row.clone());
        }
    }

    fn print_header(&self) {
        let mut header_line = String::new();
        let mut separator_line = String::new();

        for column in &self.columns {
            let padded_header = match column.alignment {
                ColumnAlignment::Left => format!("{:<width$}", column.header, width = column.width),
                ColumnAlignment::Right => format!("{:>width$}", column.header, width = column.width),
                ColumnAlignment::Center => format!("{:^width$}", column.header, width = column.width),
            };
            header_line.push_str(&padded_header);
            header_line.push(' '); // Add a space between columns

            for _ in 0..column.width {
                separator_line.push('-');
            }
            separator_line.push(' '); // Add a space between columns
        }
        println!("{}", header_line.trim_end());
        println!("{}", separator_line.trim_end());
    }

    fn print_row(&self, row_data: Vec<String>) {
        let mut row_line = String::new();
        for (i, data) in row_data.iter().enumerate() {
            let column = &self.columns[i];
            let padded_data = match column.alignment {
                ColumnAlignment::Left => format!("{:<width$}", data, width = column.width),
                ColumnAlignment::Right => format!("{:>width$}", data, width = column.width),
                ColumnAlignment::Center => format!("{:^width$}", data, width = column.width),
            };
            row_line.push_str(&padded_data);
            row_line.push(' '); // Add a space between columns
        }
        println!("{}", row_line.trim_end());
    }
}
