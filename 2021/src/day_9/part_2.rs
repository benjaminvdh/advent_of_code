pub fn print_success_message(product: u32) {
    println!(
        "The product of the three largest basin sizes is {}.",
        product
    );
}

pub fn compute_basin_size_product(field: Vec<Vec<u32>>) -> u32 {
    let mut basin_sizes = compute_basin_sizes(field);
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn compute_basin_sizes(mut heightmap: Vec<Vec<u32>>) -> Vec<u32> {
    let mut basin_sizes = vec![];

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            basin_sizes.push(compute_basin_size(&mut heightmap, x, y));
        }
    }

    basin_sizes
}

fn compute_basin_size(heightmap: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    if heightmap[y][x] == 9 {
        return 0;
    }

    heightmap[y][x] = 9;

    let up = if let Some(prev_y) = y.checked_sub(1) {
        compute_basin_size(heightmap, x, prev_y)
    } else {
        0
    };

    let right = if x + 1 < heightmap[y].len() {
        compute_basin_size(heightmap, x + 1, y)
    } else {
        0
    };

    let down = if y + 1 < heightmap.len() {
        compute_basin_size(heightmap, x, y + 1)
    } else {
        0
    };

    let left = if let Some(prev_x) = x.checked_sub(1) {
        compute_basin_size(heightmap, prev_x, y)
    } else {
        0
    };

    1 + up + right + down + left
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

        assert_eq!(compute_basin_size_product(field), 1134);
    }
}
