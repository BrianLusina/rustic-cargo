pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let rows: usize = minefield.len();
    let columns: usize = minefield[0].len();

    // handle edge cases of empty columns
    if columns == 0 {
        return vec![String::new()];
    }

    // convert input to a 2D grid of bytes fir efficient access
    let grid: Vec<&[u8]> = minefield.iter().map(|row| row.as_bytes()).collect();

    let mut result: Vec<String> = Vec::with_capacity(rows);

    for y in 0..rows {
        let mut new_row = String::with_capacity(columns);

        for x in 0..columns {
            if grid[y][x] == b'*' {
                new_row.push('*');
                continue;
            }

            let mut count = 0;

            // check all 8 possible adjacent cells
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        // skip current cel
                        continue;
                    }

                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;

                    if ny >= 0 && ny < rows as i32 && nx >= 0 && nx < columns as i32 {
                        if grid[ny as usize][nx as usize] == b'*' {
                            count += 1;
                        }
                    }
                }
            }

            if count == 0 {
                new_row.push(' ');
            } else {
                new_row.push_str(&count.to_string());
            }
        }
        result.push(new_row);
    }

    result
}


pub fn annotate_v2(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let rows: usize = minefield.len();
    let columns: usize = minefield[0].len();

    // handle edge cases of empty columns
    if columns == 0 {
        return vec![String::new()];
    }

    // convert input to a 2D grid of bytes fir efficient access
    let mut result: Vec<String> = Vec::with_capacity(rows);

    for (row_idx, row) in minefield.iter().enumerate() {
        let row_bytes = row.as_bytes();
        let mut new_row = Vec::with_capacity(columns);

        for (col_idx, &cell) in row_bytes.iter().enumerate() {
            if cell== b'*' {
                // keep mines as they are
                new_row.push(b'*');
            } else {
                // count adjacent mines
                let mine_count = count_adjacent_mines(minefield, row_idx, col_idx, rows, columns);
                if mine_count == 0 {
                    new_row.push(b' ');
                } else {
                    new_row.push(b'0' + mine_count);
                }
            }
        }

        result.push(String::from_utf8(new_row).unwrap());
    }

    result
}

fn count_adjacent_mines(minefield: &[&str], row: usize, col: usize, rows:usize, cols:usize) -> u8 {
    let mut count = 0;

    // check all 8 directions(including diagonals)
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                let nr = new_row as usize;
                let nc = new_col as usize;

                // check if it's a mine
                if minefield[nr].as_bytes()[nc] == b'*' {
                    count += 1;
                }
            }
        }
    }
    count
}