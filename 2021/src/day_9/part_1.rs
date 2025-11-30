pub fn print_success_message(total_risk: u32) {
    println!("The sum of all risk levels is {}.", total_risk);
}

pub fn compute_total_risk_level(field: &Vec<Vec<u32>>) -> u32 {
    field
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, value)| compute_risk_level(field, *value, x, y))
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn compute_risk_level(heightmap: &Vec<Vec<u32>>, value: u32, x: usize, y: usize) -> u32 {
    let prev_x = x.checked_sub(1);
    let next_x = x + 1;
    let prev_y = y.checked_sub(1);
    let next_y = y + 1;

    let up = prev_y.and_then(|prev_y| heightmap.get(prev_y).and_then(|prev_row| prev_row.get(x)));
    let right = heightmap.get(y).and_then(|row| row.get(next_x));
    let down = heightmap.get(next_y).and_then(|row| row.get(x));
    let left = prev_x.and_then(|prev_x| heightmap.get(y).and_then(|row| row.get(prev_x)));

    if up
        .iter()
        .chain(right.iter())
        .chain(down.iter())
        .chain(left.iter())
        .all(|&neighbor| *neighbor > value)
    {
        value + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let field = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(compute_total_risk_level(&field), 15);
    }
}
