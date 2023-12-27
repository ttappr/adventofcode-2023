[LANGUAGE: Rust]  🦀

One thing I like about OO, is it's always an option when I'm not entirely familiar with the problem and feel like things will get complicated, I can always abstract and delegate the complexity to objects and give them roles and methods that break the problem down into manageable pieces.

At first the problem seemed complicated, but once I had my `Brick` class implemented, everything got simpler in my mind, and I was able to complete this without too much effort. In retrospect, I view it as a graph problem which I could have addressed without creating classes. But my code is still pretty minimal as it is.

For Part 2, a breadth-first sort of traversal is done though the `Brick`s (which are vertices whose points of contact with other `Brick`s are the edges). A brick is set to `is_falling`, then its adjacent bricks resting on top of it are pushed into the queue. When the are handled they check if all bricks they rest on are falling, and if so, they also fall, and it cascades upward.

The bricks land on a 10x10 platform that serves as a sort of height map. As bricks land the height of the affected cells is incremented.

[Full Source](https://topaz.github.io/paste/#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)

    fn part_2(bricks: Vec<Brick>) -> Result<usize, Box<dyn Error>> {
        let mut count = 0;
        let mut is_falling = vec![false; bricks.len()];
        let mut queue      = VecDeque::new();

        for b in &bricks {
            is_falling[b.id()] = true;
            queue.push_back(b.id());
            
            while let Some(b) = queue.pop_front() {
                for &b in bricks[b].above() {
                    if !is_falling[b] && bricks[b].will_fall(&is_falling) {
                        is_falling[b] = true;
                        queue.push_back(b);
                        count += 1;
                    }
                }
            }
            is_falling.fill(false);
        }
        println!("Part 2 Total will fall........: {}", count);
        Ok(count)
    }