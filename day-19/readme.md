
[LANGUAGE: Rust] 🦀

OO approaches do not always result in massively more lines of code than not taking an OO approach... but this time it did =) I attribute this to lack of "domain knowledge". I didn't know what sort of effort it would take to implement part 2 (having not seen it for the first half and anticipating much trouble afterward...), so I carefully organized and thought out **everything**.

The drawback of this approach is obviously the potential for LOC to explode, but the benefit is often the main sections of code are very terse and simple. And my personal experience is having taken the implementation step by step and making understandable objects and methods, I didn't have to deal with much uncertainty while finding the solution. It all just came together and worked without much debug.

This is an iterative solution - no recursion.

[Massive Full Sources](https://topaz.github.io/paste/#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)

    fn part_2(path: &str) -> Result<i64, Box<dyn Error>> {
        use RangeResult::*;
        let (wflows, _) = parse_input(path)?;
        let mut stack   = vec![Forward("in".into(), RangePart::new(1, 4000))];
        let mut combos  = 0;
        while let Some(result) = stack.pop() {
            match result {
                Accept(part) => {
                    combos += part.num_combos();
                },
                Forward(name, part) => {
                    let flow = wflows.get(&name).unwrap();
                    for result in flow.range_match_rules(part) {
                        stack.push(result);
                    }
                },
            }}
        println!("Part 2 Number of Combinations....: {}", combos);
        Ok(0)
    }