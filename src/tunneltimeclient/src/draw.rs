pub struct Row {
    pub header: String,
    pub columns: Vec<String>,
}

pub fn draw_table(rows: Vec<Row>) -> String {
    let mut longest_of_each: Vec<usize> = rows.iter().map(|_| 0).collect();
    let mut index: usize = 0;
    for row in rows.iter() {
        let mut length = row.header.len();
        for column in row.columns.iter() {
            let new_length = column.len();
            if new_length > length {
                length = new_length;
            }
        }
        longest_of_each[index] = length;
        index += 1;
    }
    let mut longest_of_each_str = "".to_string();
    for length in longest_of_each.iter() {
        longest_of_each_str = format!("{}{}", longest_of_each_str, length);
    }
    format!("hi: {}", longest_of_each_str)
}
