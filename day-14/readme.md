[LANGUAGE: Rust] 🦀

The solution I came up with reuses some code from a previous puzzle for cycle detection. The code below cycles the platform while taking hashes of the grid after each cycle. This goes on for a short period to generate enough points to feed into the cycle detection function.

The `brent()` cycle detection function finds the start position and period of the cycle. And from that the minimum number of cycles that need to be run to put the grid into the same state it would be in after a million cycles is calculated. The grid/platform is cycled and totalled.

[Full Code](https://topaz.github.io/paste/#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)

    fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
        const N_CYCLES    : usize = 1_000_000_000;
        const N_TEST_VALS : usize = 256;

        let mut grid   = get_grid(path)?;
        let mut hashes = Vec::new(); 

        // Generate a vector of hashes to use for cycle detection.
        for _ in 0..N_TEST_VALS {
            roll_n(&mut grid); roll_w(&mut grid);
            roll_s(&mut grid); roll_e(&mut grid);
            hashes.push(grid_hash(&grid));
        }
        // Find the cycle length and start index of the cycle.
        let (lam, mu) = brent((hashes[0], 1), |(_, i)| (hashes[*i], (*i + 1)));

        // Calculate the cycles needed, get a fresh grid and run the cycles.
        let n_cyc = (N_CYCLES - mu - 1) % lam + mu + 1;

        grid = get_grid(path)?;

        for _ in 0..n_cyc {
            roll_n(&mut grid); roll_w(&mut grid);
            roll_s(&mut grid); roll_e(&mut grid);
        }
        
        let total = get_total_load(&grid);

        println!("Part 2 Total: {}", total);
        Ok(total)
    }