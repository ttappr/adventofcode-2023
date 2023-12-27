[LANGUAGE: Rust] 🦀

Part 1 went well. Used a breadth first approach to finding the farthest point. My `Maze` class is a little bit over-engineered; but it's organized, and it simplifies the main sections of code.

Part 2 implemented using [Union Find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) to get the groups. First, the code follows the pipe path to create a UnionFind group for the path points. Then the integer space of the matrix is traversed to connect all adjacent points that aren't path points. After that, with the path group taken out, there are two groups: the enclosed points and the non-enclosed points.

Since close adjacent pipes can create islands of points that aren't enclosed, but aren't able to connect to the same root as the other outside points, I used a scaling trick. I scaled the points up by a factor of two, and added in mid points along the path. This in effect makes it impossible for adjacent pipes to touch. To get the right counts this trick had to be accounted for at the very end.

I need to review what other people have come up with. I'm sure there are much simpler ways to complete part 2.

[Full code](https://topaz.github.io/paste/#XQAAAQCmLQAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9pldIvRxUkBjF31rXqW3EWzsNSoVDIKRbkAZwN7PmgqcoX6wU88sljx2GSI5uRwnPJEbCKX3yXlPDHiwOrN/E8WoxAov3MKC/zz9GG1enoSGERMi8GxSO8FDDB0NgJksPGcpu8Zja6VdGaHSc+dfgsLUIMOQ8OGsDGLYiEE1kz+zBbdGJx2MR4hAZY2dyTjHY6xUw7sme3icsXPMMChPueYpbOP8S7QfFXYxy/cdRkIuBWWwzfuds6WkBZJr/XgEo37qiwM3N4hZLqfBSw9aLgHcBRJnlMgPS0AZhc0l/ByODmSUnwkJLJVwGoxL1MOq8GwpLlrtN354jngvatoryV28eXrrbYRSoTFu5rWybGNM+KLnrVGP/RatsbbYdqStSqiOTW06GsXr5PXT5EH41LMA42uOiF1tyT/6DKxcqt6Hcp/uMN6VbrsrXG4ZWy67Na60r35opBjceDqrpcmvNnGwsp1a4POfV2XRnrbdAm1jlYdoUSIJO75wpuLlafIrmM8C4Ng2yNz3z4Lo3Umlw/823eLZ/X23/fCiLILCJDb0HYY8PwU1ql3CfGhXj+U0NXyIImLRBcFlT20Jnd2jCA84ivE/drnlwrsGknA9ObQPigRMllF5+D+zf1J6avpx1E8ZTYn1JFEc7/QNbiCx7Dc0GGRtnDo9qEr8Uvp87TTTIvFshOPUKrB0CmNWlF9S3aXexIfFHvsCZlZiwibZsztkDjKpFaCndjIB+myvd+hx7zfs5ychMFlTyNe2nCyUgGu1tWe7jrpBJf2coLcmGZKJJAnhwCbZ6ELjFDOiVu+aB1rHtg5SgT0p6/EOAwcBGqjm7xwo6nn3NLYJFVNWcQ5gT/ppqUVzUho6gH+VAzSWQpUkHh0AonL5TMafoXR8IdyYmHnY4b1LtmBjv4UpVYFYQNPCHWuBecV42BWpS9+95FMSZEyA/DPRSev1UlhU6DJbN/KFOxadOpK8HAOfZixUbINypc7KtCC1kKCZ6IdMl27lc4XM6mUyTnyiAIJM6vebs5nvpm5YPInBCiKVDBtanXomv/I7vXcRdDxDqiYLWVqDIHuWOSZxhaNXxyiKiTbNBbv+pioNfJ9c8UdRmrQe1N6cEdlItm/dIlQ7poGkKyAOZR8g86QnKtrvgWM/yQUhmjwTjDAGnBon7D9++Yeazdv9TXiV4efAAy8Gb3o4+DSotje6gPfCThKHfgLbnei3whke79chwfWY7VQszbJfMHwzMx/DBGQeSECGBH7ZIhfx2uevj4K15CoQsPmITPRqhuMF8H0Z4w7J8d7YJSoPUcppuxr/xHq84Z7gN4wIr+QX5PLEmqIgEovMyS2c0UXQGFgpzspsCc0GUD9V7bMcMKrd/N4LhAPXd53OLKydB+6a/klQ30e9X1QV50yePPGJRQFxO+dHTdTs8sWjsOXT+bFyiFE3OeUaQUynqWpKVRfKQI0Capb6BsJo8Wvd2q+x9TcX22j8WNDrDo9QCGgg5n85GbMHyJEXWk35kCKeux5aRFqfsD1Xz4I5kcZnDShUi6OSdLIqrDQ6dj6fJExuIAF86kQGvvu+1aNXoFLprCjjTsMV9LEYdEPqfcwJFKBcGzPK3QAEwuTFp6Bny6K2uPmCO2gmUDHzbBSMwMe8tLu+aYTy/EzR65fqzf3CCsEBmhrqcWjvPc7w9TeHqjV65+gHAvCv2xPk4otDDc+Ad0cDR9WY6zgr+sCo6JYQFnkCN97HBi3oYJb1d1FQWqgvRL6qews7q8dfP/cn5PLdN5BhS64v99D8H/S2cjg7ysfPf9C4Zc9kSDuZRS54QWFj5cf7nL8yfH9OL6molSMi+GucFeICI+oWlJFoffPQWBnFQaE4TWlDHCR+K4+EOuFLY2xgGxbR3NIu2XxRj82s/5Po+bwvZOZ7uw7XJRLukkqJmzbt1O4FtouJrmH1xL09E8J/3QLQ8jLBkJzg0WjXewnksc7D7SCeB/ylEYK6S8+BilnACs8PNOkMPs86oq5JmSa1enbzNWpyFyfU0gHJcwVQKGEDqLjdXPWvhMIB6t6ah+d0Le73rg6aLXVjTPMZishlcLhvcO3mkcIb+iOzLn4fLSiyy4YucWuIywaAtOesXuntcd3rTgA9EhVaar8BX0Bgfs58LtiAklNnfmbZuOzx5oVdXB3MLzVeAcNxc8FGJl8/RfCyiicintythERxRNAD/kAV4ZNhXceY82JP3aeewK6RgCP33Wyidwmlvwftop9eA+AMXMf7v9joS9/iUlyHuzbM64JqeJ6iDdmupbjcZEiAXOPxoEP3zmofsTGMSXy2+x1iHGj+yvqh9cnHfCTx8XOWcThLBhGlMiUonSjEEUP3ZvO0AvlLz8XfIPOWQ8dcM3vl7fv8uDdu4/NYH35fbpiWlFylsG7GKdUY35sLiyQ65SIaSFyM9J47VODw1hE+xyQB0C0BynNYd0ZuOrbpV6B2dsgKqJnPMTpowYHtu0GCrRNFuqCJ4eNCwy8zrpfgZ9wemlbPfHQbMtxbq4tZ7kBJkLDeH1TQlkdPZO9KMcUi/sixNxpRPK8ivT8jzFZDzDiUv53orRr4Ukm8vgnulioaUzvwp7icErvoJMFrdqBr4jfj+TR3Fm/vHTlvDmzOssG1j7qvYAnRbyfJ7riM3UKy5nNHSEvFsb3Zxq4H8IO+edeqfe8N5m98IMzz5lI1jnbP97bHajtyCy0JuYKG6n4DgXraKeyTq+wCGQjLrd4uIE2DbXF+yFLc/aazPZC771Y0w+dwv0/CeEeKPXOIfKc9oLWq/gq3bgJq6fGF6nBoAuC30dVubNu6FioUEWt8G1uO+I4pNngVBCb8yJVN0+BfP2fSpKC92YSW/SGGEw8L9Tgqt8hicWcp1qJweRY0LM4OlzR3zRSy0ljxA8SkHdBg3LJZTnJ+J51MACa+xgxBs3z3tVWiMnDhJRPfxozi/+47d+claE+/M3AnIqs+VSc8TxKEkWxQdGEHLmRJrrdpeOFQLYrFI6pqNaqeJJla0Iuge4uezUT8AmM803v190uN12iLvtTM1HLVHwdm56sQOdmhQfBIUeE+EKp22nhDovYl5aAvW+KzgWEkejvKDjUyVlil1f0BXv4UjK6PPKS5QSaujB33jjoOc04a74R/QsljjaXp/Gk1o3MyEjOL9qM2/QWFbCFf5bsEz29KEuGnXEhVbGVZw4LGF8bBYAOkM/7ZzJCjOv/RFJSTecjIW/uv5IdyIPYWSdh3+2TvLFLsUcBsrfIvD5fxqNYoouRr5fzGN6YFdYWaGFBCOZ+oTeNPp+9CfR58TtupSvIEvRDQ28wRO4EOqQpOWqt6NchskgbNGgeMRg+NYhX9RPU1y6DLLmul/uEYiKwHMr7zmIFbaZG7Bxt6WOGNCqyHsimjfd6tL2j+zwHuwqO0Jnf1/pYYYeXoiBgirM89Bg1hC4qAJkK7pVuusrANgJcco/8v89y8V/6D12YFX+TNCWPaO/UUjqMsRbcmMNQgrqKQSGFmsGUIm1zpvFlpIhjBed1YU3PAONNzN5DJ3b39J7MC131ZyBHFaiwmBc5PjZiM8zRXXRIsYLf4XJkqbsi/grOLqItcGyBUjUt/esh5eb1meIx5gOqlZ2Cpv0nK4UR9mQxa5JZBaVBvl9UrtX7V/8hEw1M2tks/5RGdlPzQ14KvLFnu97kJkv3m/mnQ252TB7/Kg7HYAlU1UGNbnX/2nOgKXD76kwce61mhK2k8HHpp6Jzzgeckg3wMu1pfjUCC83xoo0enRSCi83lzOoVHC/EWSrXD89USD/wnVuinuGJ0/Gup7ZUoVvcAlEMajgQ+Pi/D5ZP6uFZt6PlfsMBN52cEoGrLhyMRECI9+SDcs1SKoCuwul34DDMoYv//U14hQ==)

    fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
        let mut maze  = Maze::new(path)?;
        let mut dims  = maze.get_dimensions();
        let mut start = maze.get_start();
        let mut uf    = UnionFind2D::new((dims.0 * 2, dims.1 * 2));
        let mut last  = maze.s_pipes()[0];
        let mut midp;

        maze.mark_visited(last);

        dims  = upscale(dims);
        start = upscale(start); 
        last  = upscale(last);
        midp  = midpoint(start, last);

        uf.union2d(start, last);
        uf.union2d(start, midp);
        
        // Keep scaling up the points on the matrix and generate artificial 
        // midpoints as the group of path points are added to UnionFind.
        while let Some(mut curr) = maze.next_pipe(dnscale(last)) {
            maze.mark_visited(curr);

            curr = upscale(curr);
            midp = midpoint(last, curr);

            uf.union2d(last, curr);
            uf.union2d(last, midp);

            last = curr;
        }
        uf.union2d(start, midpoint(start, last));

        let start_root = uf.find2d(start);

        // Now connect all adjacent points that aren't path points.
        connect_union_find_points(&mut uf, dims, start_root);

        // Get the UnionFind tile group counts.
        let tile_groups = get_union_find_group_counts(&uf, dims, start_root);

        println!("Part 2 Free Tile Groups: {:?}", tile_groups);

        Ok(())
    }