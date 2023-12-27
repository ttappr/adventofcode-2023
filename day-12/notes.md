

`?###????????` 3,2,1

- only one way to place 3.
- a dot has to go before and after the 3.
- 2 and 1 left and given 7 locations.

0. `???????` 2, 1
1. `##.#...`
2. `##..#..`
3. `##...#.`
4. `##....#`
5. `.##.#..`
6. `.##..#.`
7. `.##...#`
8. `..##.#.`
9. `..##..#`
10. `...##.#`

- group of 2 gets total of 4 positions.
- group of 1 progressively gets fewer
    - 4 first round.
    - 3 next round.
    - 2 next.
    - 1 finally.

- how to get number of positions for first group.
    - each group to the right takes up 2 + length.
    - 7 places - (2 + len) = 4. works!
- how to get the number of positions for 2nd group (12).
    - In the first 4 rounds it has 4.
    - round 1: 7 - (2 + len) = 4.
    - round 2: 6 - (2 + len) = 3.
    - so it's 4 + 3 + 2 + 1

`???????????? 3,2,1`

 
 1. `###.##.#....` `0,1,1`
 2. `###.##..#...` `0,1,2`
 3. `###.##...#..` `0,1,3`
 4. `###.##....#.` `0,1,4`
 5. `###.##.....#` `0,1,5` <-
 6. `###..##.#...` `0,2,1`
 7. `###..##..#..` `0,2,2`
 8. `###..##...#.` `0,2,3`
 9. `###..##....#` `0,2,4` <-
10. `###...##.#..` `0,3,1`
11. `###...##..#.` `0,3,2`
12. `###...##...#` `0,3,3` <-
13. `###....##.#.` `0,4,1`
14. `###....##..#` `0,4,2` <-
15. `###.....##.#` `0,5,1` <-
16. `.###.##.#...` `1,1,1`
17. `.###.##..#..` `1,1,2`
18. `.###.##...#.` `1,1,3` 
19. `.###.##....#` `1,1,4` <-
20. `.###..##.#..` `1,2,1`
21. `.###..##..#.` `1,2,2`
22. `.###..##...#` `1,2,3` <-
23. `.###...##.#.` `1,3,1`
24. `.###...##..#` `1,3,2` <-
25. `.###....##.#` `1,4,1` <-
26. `..###.##.#..` `2,1,1`
27. `..###.##..#.` `2,1,2`
28. `..###.##...#` `2,1,3` <-
29. `..###..##.#.` `2,2,1`
30. `..###..##..#` `2,2,2` <-
31. `..###...##.#` `2,3,1` <-
32. `...###.##.#.` `3,1,1`
33. `...###.##..#` `3,1,2` <-
34. `...###..##.#` `3,2,1` <-
35. `....###.##.#` `4,1,1` <-
 
- 15 + 10 + 6 + 3 + 1 = 35

- len(field) = 12, len(g1) = 3, len(g2) = 2, len(g3) = 1.
- first group.
    - group 1 will get: `12 - (2 + 1) - (1 + 1) = 5`
        - `len(field) - (1 + len(g2)) - (1 + len(g3))`
- second group.
    - `len(field) - (1 + len(g1)) - (len(g2) - 1) - (1 + len(g3))`
    - 12 - 4 - 1 - 2 = 5
    - for each round: `[5, 4, 3, 2, 1]`
- third group.
    - `len(field) - (1 + len(g1)) - (1 + len(g2))`
    - 12 - 4 - 3 = 5
- so that's the behavior in this scenario. predict the number of permutations.
    - they're additive.
    
Subproblem:

- Given n positions, how many positions does my group get?
- 