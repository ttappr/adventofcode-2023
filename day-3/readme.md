[LANGUAGE: Rust]

🦀 This one was kind of fun. We have to avoid counting the same number more than once, and find numbers in a matrix adjacent to coordinates. Here's what I came up with for part 2, prettied up for public consumption. Full code linked below.

The method is very simple. The input data is converted into a 2-D array of bytes. Then each row is iterated over. When a '\*' character is encountered, the previous row's adjacent indices are checked for any digit characters. If one is detected, the full string of digit characters is isolated and converted to a number. The next row is examined and processed the same way. So each cell adjacent to the '\*' is checked.

The time-complexity should be linear, or very close to it if not. No special tricks or complex algorithms need to be applied to this problem. The only gotcha would be the potential for the same number to be counted more than once. A hash set is utilized to keep track of what numbers have already been processed.

[Full code](https://topaz.github.io/paste/#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)

    fn part_2() -> Result<(), Box<dyn Error>> {
        use Direction::*;
        let matrix = get_matrix("./data/input1.txt")?;

        let (m, n) = (matrix.len(), matrix[0].len());

        let mut total = 0;
        let mut seen  = HashSet::new();
        let mut nums  = Vec::new();

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == b'*' {
                    nums.clear();
                    for (dir_i, dir_j) in [(Up, Lt), (Up, At), (Up, Rt),
                                           (At, Lt), /* 🦀 */  (At, Rt),
                                           (Dn, Lt), (Dn, At), (Dn, Rt)] {
                        let opt_i = checked_delta(dir_i, i, m);
                        let opt_j = checked_delta(dir_j, j, n);

                        if let (Some(i2), Some(j2)) = (opt_i, opt_j) {

                            if is_digit!(matrix[i2][j2]) {

                                let (num, k, l) = grab_number(&matrix[i2], j2);
                                
                                if seen.insert((i2, k, l)) {
                                    nums.push(num);
                                }
                            }
                        }
                    }
                    if nums.len() == 2 {
                        total += nums[0] * nums[1];
                    }
                }
            }
        }
        println!("Part 2 Total: {}", total);
        Ok(())
    }