[LANGUAGE: Rust]

Part 1 & 2 are nearly identical aside from the totaling logic. Regarding day 1.. overlapping matches.. ugh! Rust's \`regex\` crate doesn't support overlapping via lookahead patterns. I spent hours poring over the code thinking the puzzle itself must be bugged. It only occurred to me there were overlaps when I parsed the input data with a python script.

[Full code](https://topaz.github.io/paste/#XQAAAQBlCAAAAAAAAAA6nMjJFMpQiatRkn4GTAedgepP67iMdB1JPtfIe44XjGu4NL6C8PffJPG4/rduOYcb/lTavkvKlI34eXaxdJuv2rV2noFvV/7tCxbaaHBdY3g+zLt57hDTkBp5NHFPZOzGH56jBJrtqjWXNL5cqRXXw1LBUfRnCAzJx6sW7MUp6QiPLBMIQvRYY7ZFxaoqnXiFeuBGcHkV2kUFwgnrYG2TypfXvP7a31lRe6+lKJzXH1zyOLQP5uKyb5B+bK5rfRhS+xdIoH/j+jGz7oUQApIO2xRic7JxN2xwSdk2yK0SWni6bRegntqEXr7tSJ4eYhoiY/KjaPBSFiDru4xrxcvV0pF3Pn6LBG+90bpIClWn1JFXNDf/w0rUPeHaXLicBZRW4hEOgFf1DIAl4EbIF3fbn9k2VPiQdvEdGsjeqjY0RogHtYhu91oQwshgWfX7OmCkKG7NZF3P0EjQWu+6jGan9yG7dXOU/PyOrRmv45Cohb5Faxnv0zs50c9CB3ZS24i1fktYawNKIh4wjhOXZwcsL8wZGgs9vYjO+boCmQNzaXqh1zXj2cFATQMuKxcoHRRuQn788WzT8Y3eaKIY3/POFkPjUmFJP84dlJmK+tuv10u4gILe72qQwxl++j6jSh3NMpkB71hIO6txaooq7rVQzI8EgNzZsf6C63mXGG8106pQ36zvM4aOBSi4ntsXX4E1CnQyy0Kp/HdpVJj5bz1FxShwphCeTm9rACeWkxd9ZHxb89/EbLlAsqNnK5yqYoJ7Q+AXrQhpRInif2upjPoTqutbwqHDi0g6d/11jCATbqZlkupUHedHWaF3JwE0fxi4s/NvNVnh5w//jhAhQA==)

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