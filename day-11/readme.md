[LANGUAGE: Rust] 🦀

Just a matter of getting the [taxicab distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between pairs of points and using a [prefix sum array](https://en.wikipedia.org/wiki/Prefix_sum) to calculate their row and column distances.

The code below uses a custom 0-based [BIT/Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree). The code linked uses simple prefix sum arrays instead, but was too large to post in the thread.

If you want to play around with the Fenwick tree, [you can get it here](https://github.com/ttappr/fenwick.git).

[Full solution](https://topaz.github.io/paste/#XQAAAQDsCAAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9pwI1UAwQDraYb+g3llEsg2JG3leJVwKsieKtdK4aAAQyp5vdLs3zcJDedArjXGYUvP+vNoK6yChUYOuiFHPiVvSkOrMmvfdbHC78OY3BkfgdpmQ2aKZwHU5PjHooMZOGzcbUcnUyhD1iL8KT7HjALwLup5YanSE5z2EjeETlDztVp6Asso0txasd8U/C7X1I4HlOy85+jtLP0OXerbTDR+czHbE1FS2Cld9Sebfcf+f09umYE2Eg8TnfLEGalnEKprDnSwkzsAQ+N7alIrRob5buV1C77O6sltSYmFLFufb/YttY3AG/xuKBvddoZTzOvdQtWflmeVNvWepfnWb/gEPZOdt9Q0VtlD6bPQPgViw8rtPyoLH26yNK4DmOfgwEpojEctFDmqpHEC2zy+hDA8I48M3rksXrzws8Kt3wd2OtwejIbG0Aa0PvYvrgsIup1j+2WnjHZKqZn/GFCcDFOCo4o+h3/DdWMPuE/0SLv01sXXzygNjhS0vG3CPlJ+3N+uD5F91kqXYYc/KXTzwQL4zDBixdss61je/Y/EgkbdziEC+80rMg1UNZXNhtegtvngEaxvl+VLmbDmzyhiFo11r0l9O3VO7OJmafobEADFufpdPTuiCqEjkzGRRM1D1bAak5+kxN0Rzq25Pjhpf6pe1UMq9lu5st5ElROlRD2iT+JHbZ0SEknwupXO+3+heEPHsBpK7rJJxhPRgYDAhzvnjsUxyYn6hMDsuqVhVllo3A1ATCgPaNuBJ8H862yd3LLSekWFubF7YhQi/qowtTphnapBdWzXAHUzL0QrNqqKDBiWESOBIGlwgC2ZwpVXuSmCfRFSJ7spr7aE8L9f9Ef6nNrDB3w+SA0f48svpfdbNTC9awKUFPUENDZjvHlmuVgaL5X58ANxsTUIeaA9uS6asYyflvLuoMSsPEuRImdHRiZdz3rqyXiLl3NyzqBptvolXlCUhbaCZiBz2zYJpGlWZ2cIIPhXIHKgJ74B87HrfKBbMaivySR6O22VQ6GAJdvul1z8ZO9AsatkmuXIhT5JKRr70daqhB1s6RXP8ScMs7uPdnhjptvsH39B+85Ph4Azc+xpyo+bght2c6F6Z/68ySM=)

    fn solution(path: &str, expansion: i64) -> Result<(), Box<dyn Error>> {
        let file   = File::open(path)?;
        let reader = BufReader::new(file);
        let lines  = reader.lines();

        let mut col_dists = Fenwick::from_vec(vec![expansion; 256]);
        let mut row_dists = Fenwick::from_vec(vec![expansion; 256]);
        let mut galaxies  = Vec::new();

        for (i, line) in lines.enumerate() {
            let line = line?;
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    galaxies.push((i, j));
                    if row_dists.get(i) == expansion {
                        row_dists.sub(i, expansion - 1);
                    }
                    if col_dists.get(j) == expansion {
                        col_dists.sub(j, expansion - 1);
                    }
                }
            }
        }
        let mut sum = 0;

        for (i, g1) in (1..).zip(&galaxies) {
            for g2 in &galaxies[i..] {
                // Taxi distance.
                sum += (row_dists.prefix_sum(g1.0) - 
                        row_dists.prefix_sum(g2.0)).abs() +
                       (col_dists.prefix_sum(g1.1) - 
                        col_dists.prefix_sum(g2.1)).abs();
            }
        }
        println!("Sum of shortest paths: {:>12}", sum);
        Ok(())
    }