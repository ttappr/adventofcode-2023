[LANGUAGE: Rust]

The algorithm declares an array set to all `1`'s to keep the counts of cards and their copies. Then for each scratcher, its winning numbers are processed into a [Counting sort](https://en.wikipedia.org/wiki/Counting_sort) array, then the elf's numbers are iterated over while checking the sort array for hits. Then the count of the number of hits is used to update the DP array.

And finally, the DP array is tallied to give the final number of cards the elf had and won.

🦀Time Complexity: `O(n)`
- The maximum number of winning numbers per scratcher is the constant `10`, which sets a constant worst-case upper bound on the number of cycles required to update the DP array for each card. And Counting sort was used to accommodate the unordered numbers, which also has a linear TC of `O(n)`.

🦀Space Complexity: `O(n + len(line))`
- A DP array of size `n` is allocated. The input is read line-by-line from the file, and only one line at a time uses memory.


Click here for [full source, parts 1 & 2](https://topaz.github.io/paste/#XQAAAQCECgAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9qbTWCJFvNga2QWVOVJZUi9iB5++VsNzhT7H4hQIOTK38G9+GXcNKoLjk359Y4HBkmAlmqnmrGXKBGFW+HWFQlWztXAJWCZoPwCltVWQG/vWYI9junVhX3oZqzYqVMQWw7pVwVNZKiJSem/vd8SSlRU0c19S/hmp7zwC7Zj1aKWBtk9/FoZEflQdJ/mADHozVFRppeV/+qpqKDMPI0ebbDIevUfeWSdisFsdx4QT+D2D08RsjeYKLzmS3LRdDyXYKt6NLbMap/7XLyoe7C3fM98Sz73vSPtZZdEhumUDL0XUU+t1Hp0v5swt9WCWk+kHt5/wRMv7N/uNywVXwMgMCmav0O0jM6YaHVViuzIYkbiCK1N5f69Le8T1z6anWnVeO/L1ms11LlJWrpxe5Qs8sjM8aw6e8RY81q5w/AAkCBwn3mE6AtpGS18+BZ5rO/xGWutT4ksU3bE/2z51sMcQc79kwzadmMKPlH0Fh0LVFv/d6wQwhK8N8QP97OB2sZ33Sti6X/cpagQlhOgus31XP+A9FaTCS3Emdqm7nN5cBRFlRz/dNKps+X1wDUK5oEREuCllb+aMbBWMY89tdFqSFz46DrkGUDm7f5QJf39ePaErwUcubOFpm06AQ/iB8MQDJrIA9v6CBEpi2Q+R82ywMNf6ELy5DqiR8ZgRls8FAbhHvhvq5Lxh6/8AkGcVxtywYYfcPNju8ZMGTOKCiSrOWnmJOGgZ2j8YHs0IiIDIPGEMT4nn9+D5HPb47xVlqrL8kYb03e8OMDFhS3KOrAPGhg54m17AJTfA9aI3bVMx2eteygWKxGhmipD7vuzNo+uXZdt5cejqXkB9YXCJ6p+e4yC86Ql9wptEqqtYwKeT64PVpygZpqj/dN7jxCaN507vhtuXgAy7GtUuT+2O0JljY0OvF1i88zPh2r16BeomEo9yvlIaPj+4/lzsBIZI10Ww4UPqfOPNpN6uBhnyUlPwnxCHFFSvNNicd/f/+DII+ifHoxHnu3twvxC6G+/g5IZGY4fW2Dyf8xD7j7LJj79aKu1vIPTDFwXZKNKT5UUZfFYCw1RYG7nQFkGIBXuaViPuTZejnZjGnJFiRXmiwXnyy5AyhIOVczb//KxTt4=)

    fn part_2(path: &str, n_cards: usize) -> Result<(), Box<dyn Error>> {
        let file   = File::open(path)?;
        let reader = BufReader::new(file);
        let rexpr  = Regex::new(r"^Card +\d+: +(.*?) \| (.*)")?;
        let to_vec = |s: &str| s.split_whitespace()
                                .map(|s| s.parse())
                                .collect::<Result<Vec<usize>,_>>();

        let mut cards = vec![1; n_cards];

        for (card_i, line) in (1..).zip(reader.lines()) {
            let line = line?;
            let caps = rexpr.captures(line.as_str()).ok_or("No match")?;
            let win  = to_vec(&caps[1])?;
            let mine = to_vec(&caps[2])?;

            let mut win_nums = [false; 100];
            let mut n_wins   = 0;

            for w in win {
                win_nums[w] = true;
            }
            for m in mine {
                if win_nums[m] {
                    n_wins += 1;
                }
            }
            for i in card_i..(card_i + n_wins).min(n_cards) {

                cards[i] += cards[card_i - 1];

            }
        }
        println!("Part 2 Total: {}", cards.iter().sum::<usize>());
        Ok(())
    }