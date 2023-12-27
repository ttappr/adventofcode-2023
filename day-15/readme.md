[LANGUAGE: Rust] 🦀

I had a feeling that this was brute-force-able, so I resisted the urge to optimize while coding it, and I'm glad I did. This runs pretty quick despite the multitude of `O(n)` array operations.

The code includes a custom buffered reader, `ElfReader`, which allows the reading in of small records from the input file for processing, and discarding afterward. This should be more memory efficient than reading in the entire contents at once and retaining it for the duration of the application.

[Full Code](https://topaz.github.io/paste/#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)

    fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
        let     s2int  = |s: &str| s.parse::<usize>();
        let     reader = ElfReader::new(path, ',')?;
        let     expr   = Regex::new(r"(\w+)([=-])(\d+)?")?;
        let mut boxes  = vec![vec![]; 256];
        let mut total  = 0;
        
        for record in reader {
            let rec   = record?;
            let caps  = expr.captures(&rec).ok_or("Bad record!")?;
            let label = caps[1].to_string();
            let oper  = &caps[2];
            let box_  = hash(label.as_bytes());
            let pos   = boxes[box_].iter().position(|lf: &(_,_)| lf.0 == label);

            match (oper, pos) {
                ("=", Some(p)) => { boxes[box_][p].1 = s2int(&caps[3])?; },
                ("=", None)    => { boxes[box_].push((label, s2int(&caps[3])?)); },
                ("-", Some(p)) => { boxes[box_].remove(p); },
                _              => (),
            }
        }
        for (box_n, box_) in (1..).zip(&boxes) {
            for (slot_n, &(_, focal_len)) in (1..).zip(box_) {

                total += focal_power(box_n, slot_n, focal_len);
            }
        }
        println!("Part 2 Total Focusing Power.: {}", total);
        Ok(total)
    }