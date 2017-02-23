pub mod draw_field {
    pub fn draw(field: &::game::Field) -> String {
        let rows: Vec<String> = field.iter().map(|&row| draw_row(&row)).collect();
        rows.join("\n")
    }

    fn draw_row(row: &[::game::Player; 3]) -> String {
        let cells: Vec<String> = row.iter().map(|&cell| draw_cell(cell)).collect();
        cells.join(" ")
    }

    fn draw_cell(cell: ::game::Player) -> String {
        match cell {
            ::game::Player::Nobody => "-",
            ::game::Player::X => "X",
            ::game::Player::O => "O",
        }.to_string()
    }
}
