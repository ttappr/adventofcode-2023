fn center_of_mass(matrix: &Vec<Vec<u8>>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut n = 0;

    for (i, row) in (0..).zip(matrix) {
        for (j, &col) in (0..).zip(row) {
            if col == b'#' {
                x += i;
                y += j;
                n += 1;
            }
        }
    }
    (x / n, y / n)
}

fn moment_of_intertia(m: &Vec<Vec<u8>>, cm : (i32, i32)) -> (i32, i32) {
    let mut ix = 0;
    let mut iy = 0;
    let mut n  = 0;

    let (cx, cy) = cm;

    for (i, row) in (0..).zip(m) {
        for (j, &col) in (0..).zip(row) {
            if col == b'#' {
                ix += (i - cx).pow(2);
                iy += (j - cy).pow(2);
                n  += 1;
            }
        }
    }
    (ix / n, iy / n)
}