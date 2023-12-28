NOTE: This project requires the MS z3 solver library to work. The project is set
up correctly to use it, however the z3 crate doesn't build out of the box and
requires some tweaking to get working.

First, you'll want to install the z3 package to your system.
Here's a [link to the z3 GitHub repo](https://github.com/Z3Prover/z3). And if
scroll down to [this section](https://github.com/Z3Prover/z3?tab=readme-ov-file#building-z3-using-vcpkg),
you'll find simple instructions on how to install z3 using `vcpkg`. I did this
on my Windows system and it "just worked" - well, at least the z3 build worked
cleanly. 

I made a folder for [vcpkg](https://vcpkg.io/) where I keep my other projects
and did the simple install step (ran ./bootstrap-vcpkg.bat), then cd'd into
the folder it creates and ran the "./vcpkg.exe install z3" command.

`vcpkg` places "installed" packages in a subfolder of its own directory under
"./packages/z3_x64-windows". The path will be different if you're on Linux.

- The `z3.h` header files will be in the package folder under "include".
- The `z3.lib` file will be in "lib".
- The `z3.dll` file will be in "bin"

This is very hackish - I know - but this was the quick and dirty way I got it
working so I could complete the day-24 puzzle: you'll want to copy the .lib and
.dll files into the root folder of your day-24 Rust project.

Then update an environment variable that the z3 crate uses to locate the z3.h 
file. You'll want the full path to the package's "include" folder plus the
specific name of the file, for instance: 

`Z3_SYS_Z3_HEADER="/users/me/projects/vcpkg/packages/PLATFORM-NAME/include/z3.h"``

Use whatever command is appropriate for setting the environment variable. I 
issued `$Env:Z3_SYS_Z3_HEADER="C:/Users/me/projects/vcpkg/packages/z3_x64-windows/z3.h"` (PowerShell) in the vscode terminal while 
the project was opened so I could build and run from the vscode terminal command 
line.


[LANGUAGE: Rust]  🦀

For the first part, I figured a solution could be arrived at pretty easily by using matrices to solve the system of equations for every two hailstones. I used this [mathru](https://docs.rs/mathru/latest/mathru/) crate that has working linear algebra features.

For part 2 I saw people were having fun with [z3](https://docs.rs/z3/latest/z3/index.html), so I thought I'd give it a try. I'm certain that it's much easier to use the Python interface, but I did it the hard way and got it working for Rust. There were several time sucking nuisances along the path a working solution. Several times I had to pick through the python z3 wrappers to see why things work one way for python but not for Rust. I imagine the Rust support is better on Linux.. or maybe not.. idk.

Anyway, I'm glad I tried out z3. It's a cool tool that I can pick up again and use now that I've pushed through the initial headaches and know how to use it.

[Full Code Part 1 & Part 2](https://topaz.github.io/paste/#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)

    fn part_1(path: &str) -> Result<usize, Box<dyn Error>> {
        let hail_stones = parse_input(path)?;
        let mut intersections = 0;

        for (i, h1) in (1..).zip(&hail_stones) { 
            for h2 in &hail_stones[i..] {
                
                // Matrices are column-wise defined - like Fortran.
                let a = General::new(2, 2, vec![h1.vx,  h1.vy, -h2.vx, -h2.vy]);
                let b = vector![h2.x - h1.x; h2.y - h1.y];

                if let Ok(t) = a.solve(&b) {
                    if t[0] >= 0. && t[1] >= 0. {
                        let x1 = h1.x + t[0] * h1.vx;
                        let y1 = h1.y + t[0] * h1.vy;
                        if x1 >= 200000000000000. && x1 <=400000000000000. &&
                           y1 >= 200000000000000. && y1 <=400000000000000. {
                            intersections += 1;
                        }}}}}
                        
        println!("Part 1 Total intersections: {}", intersections);
        Ok(intersections)
    }