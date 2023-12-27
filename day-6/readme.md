[LANGUAGE: Rust] 🦀

My math is a bit rusty, but I managed to work it out in this order:

- Get basic formula
    - `distance = time * rate`
    - `rate = time_hold * 1mm/s`
    - `time = time_race - time_hold`
- Plug the subformulas back in to `d = t * r`
    - `distance = (time_race - time_hold) * time_hold * 1mm/s`
    - `d = (t_r - t_h) * t_h`
- Move all terms to one side (look familiar?)
    - `0 = t_h^2 - t_h * t_r + d`
- Apply [quadratic formula](https://en.wikipedia.org/wiki/Quadratic_formula) to solve for `t_h`
    - `t_h = (t_r +/- sqrt(t_r^2 - 4 * d)) / 2`

[Full code](https://topaz.github.io/paste/#XQAAAQBcCQAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9q31JSJUclob3zfDB9n5ByjvLyIC3rmTJ8ZEnLEG/8GQS/8i3bIUrTMqaoh4ZGZVBDj8Qn8CE9QDtZJKZWHFHjOb5Ukwaqb201n3humEMyf3Fn3Sw6YKRUpQZLC8AcwPs0lwsz1XyBqEM+15Zic0Ju/iWjJ11QojR6eLSwz36zqcSRTU6i2O05EAV1QShLgsZnZheXlkuQq2gvHTK9e68dW8JFskRAvivWzSE1Sv9z0M52exhPkDZ9IDbU2mn2/5LDJm/gEtVyXcZHXoqNbB4boaC35MmrRD59A+ELqvCXViPNMGnwdgfvz7fCi3kgJ3v4W5yBv2RgA8VPn9l7tTATaNOiNDgW64Rbo5/NWRssDl8VsWvPGA0RqBWNSE4Id1uKiPnpDQ59yBu9QY/RxApZiOvtNJuRmBIVvCjmvf7qBHKud8J+utds3huGildb3Q8VpVK/H3xL4FIHyuU4be8AXjDdxx0ZDGfsr99bQqHO1sXslA8oBIIkW6LyENZxI6xIQ4hPRF5PX42mKbKv+SLa1DVGtGufC2Y4kMcyee3vdyUOzBR/60YMHTb4jvqNBcARhmxftS40HmLMihJqEZmhCMy5NoZRhLYWwfM+d8x7BfGBAY6BX+8VVGuCejrU0eZsoahYCWftVEpzJJfwnFPiewGU3XOp+7n+0PxpreWSjPbe9xdbhP3+T/k/BTmOUCGwmVVEU8FHKzkiAoAf41GN+OPiZUqNok1ul3bXHLVPOUZd32ndiNUOE9uukG74pMfpKi8i7nwd46KzgtlF6qGn3PqXpZkuSMntk99JuEs6t+HmhkXqeCuywUkR01cQnk5Vj7Z32VHHX8gAf01XvuMDP6AME8rVvBMw/+KJOXal8WZWaS5E09c/JAyL4yHDQNM4BlF74aanO8ghclUOPrL/g+pc7oxvhAu6VhPVgHbe6ynCf3MnQO5KwxmF0zSdCuGBIcMJqX9nLeTOaU3WHiYAjYvmPGyzTURt9GUeXKCjSLz/2TNRm8wL3ph67xu3wKX6DPjMmvPhIXRY6aQacrtoeCSS/WKgPJ2u7oin/+rS0Ig==)

    fn part_1(path: &str) -> Result<(), Box<dyn Error>> {
        let lines = get_lines(path)?;
        let times = get_f64_vec(&lines[0][10..])?;
        let dists = get_f64_vec(&lines[1][10..])?;

        let mut prod = 1.0;

        for (&t_r, &d) in times.iter().zip(&dists) {

            let t_h1 = (t_r - (t_r.powf(2.0) - 4.0 * d).sqrt()) / 2.0;
            let t_h2 = (t_r + (t_r.powf(2.0) - 4.0 * d).sqrt()) / 2.0;

            prod *= t_h2.floor() - t_h1.ceil() + 1.0;
        }
        println!("Part 1, product: {}", prod);

        Ok(())
    }