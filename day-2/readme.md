[LANGUAGE: Rust]

Part 1 & 2 are nearly identical aside from the totaling logic. Regarding day 1.. overlapping matches.. ugh! Rust's `regex` crate doesn't support overlapping via lookahead patterns. I spent hours poring over the code thinking the puzzle itself must be bugged. It only occurred to me there were overlaps when I parsed the input data with a python script.
Full code


    fn part_2() -> Result<(), Box<dyn Error>> {
        let file   = File::open("./data/input.txt")?;
        let reader = BufReader::new(file);
        let expr1  = Regex::new(r"^Game (\d+)")?;
        let expr2  = Regex::new(r"(\d+) (red|green|blue)")?;

        let mut map   = HashMap::new();
        let mut total = 0;

        for line in reader.lines() {
            let     line  = line?;
            let mut found = expr2.captures_iter(line.as_str());

            while let Some(finded) = found.next() {
                let num   = finded[1].parse::<i32>()?;
                let color = finded[2].to_string();
                let count = map.entry(color).or_insert(num);

                *count = num.max(*count);
            }
            total += map.get("red").unwrap_or(&0) *
                    map.get("green").unwrap_or(&0) *
                    map.get("blue").unwrap_or(&0);
            map.clear();
        }
        println!("Part 2 Total: {}", total);
        Ok(())
    }