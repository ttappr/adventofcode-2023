[LANGUAGE: Rust] 🦀

Nice to have an easy one after yesterday's. The code below does pretty much what the puzzle's description describes.

I understand the algorithm described in the puzzle to be a portion of Newton's interpolation method called "Newton's divided differences" This part alone may be referred to as the "method of finite differences", which is used to construct the coefficients of the Newton’s interpolating polynomial. Well, anyway, the salient point is, this exercise has some relationship to mathematical theory beyond being just a set of mildlly challenging summersaults. 

[Full code](https://topaz.github.io/paste/#XQAAAQAyCwAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9rg+xLQcHF3jX9gzJdovdH+vagH71qvTTZF23w3O8yj9GJorRi5s6JJjUnQz4h/E2QtftbHYbkpMYw45A8nVCCNNEwNKB9gfIN20wXciG4LBAMV6iU3iEkSlfW7GBl1D5HAZWnyk49Wr3T0CZR6am2CRjCIk7/thRRuukPjOFlZLRoOCGTtYqJIu86G/tZUPF1kFYwhXS+zsyBBeHKoACsl1xk9g2NSaD/ZgtnOGddi4Yy3EGOd/vsq6kVz2cg+4e5GbObKn7YVzN3YJdK57XRI4Clqvvor4npCpDqwz3NWYxPv3BklFN2xD37X+o+OcUMrQDKUSdSTdovbhjaYfUyyqIujZcRSEO7+NwjVTxYS/gHLr9gQjauJZhqJ1OudlkbI7mbYcB9H7ArOnZJKt+nz/cLMzW5VLb4nazBHPEBV/nmCY05vhlO1B3wspi7gejsSrZWdKDFTkKaHvnBWA1h7KL6UX+gm7JXT3SmnCpmkHKVDW1TvEBIYqw0CfYgT63GryfzKmZkvIqURwlMPcEJW2GEEnrrStxbifermpktJjuwSljGvkDr44KUrTadABYNpYwNAMnhVvFeMXcO5qxpM4udGjhqNAScxTzrOGctdW7u+Cpy9CXIb+j8helRHjKsJgLFblbKCK/BWn57miJ22/jmP8Rc9PPQBfp2LeuyKn1Cg6984m9e7zFlcZ4aXApZZhFiPgCh1Z72Cg4HLzUD+HXpBzadormy/oOwjpFoXwtqD5ISn27eJP8GItbztLSy/DTt0OZwt5pfGoJZVy2BeSGne+svFpCPmGoXmtzK/+YsCWVOiKnRVYmqRd4QFZoH37OKgFS2ncpV3h7OIWpbhF0anlnVpKfQ9btUhi8Cicy5GYz/oGmCEvGqJjUDYXZTalxiE3Bzy3MzmizZbZ+zh/r7fI+pcWHC3i8C75MyOyJ5kP1nynWrzLn3ma/mu1kdnE+/aa5nSuuwP9GTed3edr7XTFhqIG+Qy2JndwVUbdyu9tkMN5LRg1rMoiKLrpPiA9CZBKt2JQWhoxa/Sx+W5D4vLm0+mJWWbYWcYns5F8PEVQ36ss9MHTtBApRwpywVnyqWR9FAzCKMH+zpqp9XLBxZSjMrbzQ4AStC2/7LK+1oZ/rSqVQ==)

    fn part_1(path: &str) -> Result<(), Box<dyn Error>> {
        let     file   = File::open(path)?;
        let     reader = BufReader::new(file);
        let     lines  = reader.lines();
        let mut total  = 0;

        for line in lines {
            let     line = line?;
            let mut rows = vec![line.split_whitespace()
                                    .map(|s| s.parse())
                                    .collect::<Result<Vec<i32>,_>>()?];

            while vlast!(rows).iter().any(|&x| x != 0) {
                let row = vlast!(rows).windows(2)
                                      .map(|p| p[1] - p[0])
                                      .collect::<Vec<_>>();
                rows.push(row);
            }
            total += rows.iter().map(|r| nlast!(r)).sum::<i32>();
        }
        println!("Part 1 Last Column Total: {}", total);
        Ok(())
    }