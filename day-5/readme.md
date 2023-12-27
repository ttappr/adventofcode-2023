[LANGUAGE: Rust] 🦀

Behind the scenes, the file parser is implemented as a simple state machine that produces a vector of custom `Map` structs.

To facilitate binary search, the `MapEntry`'s of each map are sorted on the key, `src_start`. And, the `Map` struct has a `Map.lookup(src) -> dst` method that performs a **binary search** to find the appropriate destination value from a given source value.

Given a range of seeds, each seed in the range is used to call `Map.lookup()` on each map in series. Each map lookup's output is used as the input to the next map's lookup method.

Part 1 completes in a flash, but Part 2 takes 69 seconds. Which is good enough to justify not spending a lot of time figuring out optimization strategies, other than what's currently implemented.

[Full code part 1 & 2 + file parser](https://topaz.github.io/paste/#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)

    fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
        let (seeds, maps) = parse_maps(path)?;

        let mut location  = usize::MAX;

        for range in seeds.chunks(2) {
            for mut key in range[0]..range[0] + range[1] {

                // Serial map relationships make this easy.
                for map in &maps {
                    match map.lookup(key) {
                        Ok(k)  |
                        Err(k) => key = k,
                    }
                }
                location = location.min(key);
            }
        }
        println!("Part 2 Lowest location number: {}", location);
        Ok(())
    }