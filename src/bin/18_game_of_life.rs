const SIZE: usize = 100;

// I don't find Clippy's suggestion to be particularly good:
// for (yy, <item>) in read.iter().enumerate().take(maxy + 1).skip(miny)
#[allow(clippy::needless_range_loop)]
fn run(read: &[[bool; SIZE]], write: &mut [[bool; SIZE]], actual_size: usize) {
    for (y, row) in read.iter().enumerate() {
        let miny = if y == 0 { 0 } else { y - 1 };
        let maxy = if y >= actual_size - 1 { y } else { y + 1 };
        for (x, &cell) in row.iter().enumerate() {
            let minx = if x == 0 { 0 } else { x - 1 };
            let maxx = if x >= actual_size - 1 { x } else { x + 1 };
            let mut count = 0;
            for yy in miny..=maxy {
                for xx in minx..=maxx {
                    if yy == y && xx == x {
                        continue;
                    }
                    if read[yy][xx] {
                        count += 1;
                    }
                }
            }
            write[y][x] = count == 3 || (cell && count == 2);
        }
    }
}

fn count(grid: &[[bool; SIZE]], actual_size: usize) -> usize {
    grid.iter()
        .take(actual_size)
        .map(|row| row.iter().take(actual_size).filter(|&&x| x).count())
        .sum()
}

fn set_corners(grid: &mut [[bool; SIZE]], actual_size: usize) {
    grid[0][0] = true;
    grid[0][actual_size - 1] = true;
    grid[actual_size - 1][0] = true;
    grid[actual_size - 1][actual_size - 1] = true;
}

fn main() {
    let input = adventofcode::read_input_file();
    let mut grid1 = [[false; SIZE]; SIZE];
    let mut grid2 = [[false; SIZE]; SIZE];
    let mut swap1 = [[false; SIZE]; SIZE];
    let mut swap2 = [[false; SIZE]; SIZE];

    // This actual_size thing is bolted on after the fact,
    // since I wasn't prepared to handle the sample test case of size 6.
    let actual_size = input.lines().count();
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                grid1[y][x] = true;
                grid2[y][x] = true;
            }
        }
    }

    for _ in 0..50 {
        run(&grid1, &mut swap1, actual_size);
        run(&swap1, &mut grid1, actual_size);

        set_corners(&mut grid2, actual_size);
        run(&grid2, &mut swap2, actual_size);
        set_corners(&mut swap2, actual_size);
        run(&swap2, &mut grid2, actual_size);
    }

    set_corners(&mut grid2, actual_size);

    println!("{}", count(&grid1, actual_size));
    println!("{}", count(&grid2, actual_size));
}
