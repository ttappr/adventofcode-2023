
[LANGUAGE: Rust] 🦀

Looking at this from an OO perspective, it's not a very difficult puzzle. A `LightPath` class was created that keeps track of where a single path of light is, and what direction it's headed. 

I treated this more as a game simulation than a graph problem which may have been faster in execution, but treating it as a game seemed faster in terms of writing the code. It turned out that the speed of execution was more than acceptable with this approach.

The `LightPath` has a `step()` method that moves it forward a single step. At each step the `LightPath` checks the grid to determine if it encountered a mirror or splitter. If a mirror is encountered, the direction of the light is changed. If a splitter is encountered, the light path splits in two going the appropriate directions.

If a `LightPath` encounters a tile that was already crossed from the same direction, or falls off the grid, the light path's state is `End`, since it would start traveling a circuit if it continued; it is no longer enqueued for stepping at this point.

The function below creates a `LightPath` for each tile along the perimeter of the grid with their directions set inward. Each is added to a `VecDeque` which is popped from until all the light sources finish and the queue is empty. After that, the original paths report how many tiles they covered.

Both part 1 and 2 run together take less than 1 second to complete.

[Full Code](https://topaz.github.io/paste/#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)

    fn part_2(path: &str) -> Result<usize, Box<dyn Error>> {
        use {PathStepResult::*, Direction::*};
        let     grid    = get_grid(path)?;
        let     m       = grid.len();
        let     n       = grid[0].len();
        let mut paths   = VecDeque::new();
        let mut enqueue = |row, col, dir| {
            match LightPath::new(row, col, dir).step(false, &grid) {
                Single(p) => {
                    paths.push_back(p);
                },
                Split(p1, p2) => {
                    paths.push_back(p1);
                    paths.push_back(p2);
                },
                End => (),
            }};
        for row in 0..m {
            enqueue(row,     0, Right);
            enqueue(row, n - 1, Left);
        }
        for col in 0..n {
            enqueue(    0, col, Down);
            enqueue(m - 1, col, Up);
        }
        let init = paths.clone();
        while let Some(mut path) = paths.pop_front() {
            match path.step(true, &grid) {
                Single(p) => {
                    paths.push_back(p);
                },
                Split(p1, p2) => {
                    paths.push_back(p1);
                    paths.push_back(p2);
                },
                End => (),
            }}
        let n_tiles = init.iter().map(|path| path.coverage()).max().unwrap();
        println!("Part 2) Tiles covered: {}", n_tiles);
        Ok(n_tiles)
    }