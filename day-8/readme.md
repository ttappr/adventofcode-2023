[LANGUAGE: Rust] 🦀

Whelp.. learned something about Chinese Remainder Theorem. Most importantly that it wasn't needed for this problem. Then haphazardously I tried LCM on the cycle lengths, and was surprised it worked.

[Full code](https://topaz.github.io/paste/#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)

    fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
        let     file    = File::open(path)?;
        let     reader  = BufReader::new(file);
        let mut lines   = reader.lines();
        let     expr    = Regex::new(r"(\w+) += +\((\w+), +(\w+)\)")?;
        let mut map     = HashMap::new();
        let     dirs    = lines.next().ok_or("No lines in file")??;
        let mut starts  = Vec::new();
        let mut cyc_lcm = 1;

        lines.next().ok_or("Bad file format.")??;

        for line in lines {
            let line  = line?;
            let caps  = expr.captures(&line).ok_or("Bad file format.")?;
            let key   = caps[1].to_string();
            let left  = caps[2].to_string();
            let right = caps[3].to_string();

            map.insert(key.clone(), (left, right));

            if key.as_bytes()[2] == b'A' { starts.push(key); }
        }
        for start in starts {
            let access_fn = |k: &(_, _)| {
                let (l, r) = map.get(&k.0).unwrap();

                if dirs.as_bytes()[k.1] == b'L' 
                     { (l.clone(), (k.1 + 1) % dirs.len()) } 
                else { (r.clone(), (k.1 + 1) % dirs.len()) }
            };
            let (lam, _) = brent((start.clone(), 0), access_fn);
            
            cyc_lcm = lcm(cyc_lcm, lam);
        }

        println!("Part 2 Total Steps: {}", cyc_lcm);

        Ok(())
    }