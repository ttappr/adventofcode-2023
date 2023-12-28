[LANGUAGE: Rust]

Well this should have worked. Unfortunately, Rust's `regex` crate doesn't support lookahead expressions to help weed through overlapping matches. Ultimately, I parsed the input data in a Python REPL with a lookhead expression.

[Full code](https://topaz.github.io/paste/#XQAAAQA2CQAAAAAAAAARiEfnOfx6vWkCtaSkbrkYGSF1Vi//vyVfh81HhPFTz8E5W9TtB1qtraWyCjXLTY9+3Y42Zw5Q3K8C6+U5rHaHIxpmZmd1bJtJNQkygXXC9l9i2pRH4+yW80fBXxJGzbBYBQJW+sUyCFFI2TDi7Art68LvQwC47EhsvFUU9WVyjdXQXBUMWqZaGXLKQz/Pud25nkI1XZnqZRcBBVqxme+gybz7An/o7UONQ/jd+6s1h5/zrFT1GKkKdGMpeoPsviQLlI4PbtRDHy9fQG2Hf0o4lfVI8maOYoefwqa41pc5+E08tasKr4ggYa1t3n/Ai7wWJCx0fVpIh8pvjY5VRF7NWgpa5U/tptoaqvSF4ZAcikSuoIVco5YJCUttNaZ+zd6lju6infofcLERBY6+6T6DHxfgubxgkA0yUsxiBLN8IMKbgUvs0o5AnK9Zu+5kQyv959uQj2PKAPSM2anqnBc4Ouxz5cfAaNb7Sc92+j5Se+Kim3O0c1icjjCE3CmnrbSPTc+NJpwGs3nssqjvqjAWobSTZMgSAZI5Wfh5SKAMcHuYcivvZbra6T3VvwiZpu7Poam2IgXhAj37iOQwIWFFtzdlR0ufEg2ad3gSYprxZgIdRrZGqdm8PyPQuGONf4jLcDpCLa1SFhq0b3h6WnpdPefM0gCoAWKOQPTAG37Gc3MZ39jfgGE/P2+IbEoMYhYkFpwDP1zqB8BL5LGZvPC0o3MkQliR+kO3mx+Hhq9XukvnMBE0emon9CFwf5RlZPZ6GB89qf2ub64LEfTtBHk2EbWzBC1iRvbpXI5f6riAAmf7r5PphAJABSgPqrWdYzgaURrVOSwrt6uUWbG4HqBxM2yggsFtmNzEaS+OILlDgr3Ysjr45s+ucN6vlunr//+BI0HTiTYb+N/LpzR+EpiAUVge3LLUzQP4f0+t2DDU5X4eh2WQY+KF3BHJw4RSojrIVW7xLFFNfIUukzSbSSDWPgiQ4qVJghfeEEEQkyUzaYUpYwm2TjzCjCtnvMR1AnbZqefsvnFR7YdCoDXorWUueh7vMSs5QZvRwKGsryb3mQzNzI8IiPbCalvAWjY8BdLyL96xAmM31gV5LWuMDvoLP1FAxiFPtw9+Zj+77eMbxE/VyC98m/wNwmM=)

    fn part_2() -> Result<(), Box<dyn Error>> {
        let file   = File::open("./data/input.txt")?;    
        let reader = BufReader::new(file);
        let rexpr  = Regex::new("one|two|three|four|five|six|seven|\
                                 eight|nine|[0-9]")?;
        let mut total = 0;

        for line in reader.lines() {
            let     line   = line?;
            let mut number = 0;
            let mut caps   = rexpr.find_iter(line.as_str())
                                  .map(|m| m.as_str())
                                  .collect::<Vec<_>>();
            let digit1 = *caps.first().unwrap();
            let digit2 = *caps.last().unwrap_or(&digit1);

            for num in [digit1, digit2] {
                number *= 10;
                number += {
                    match num {
                        "1" | "one"   => 1,
                        "2" | "two"   => 2,
                        "3" | "three" => 3,
                        "4" | "four"  => 4,
                        "5" | "five"  => 5,
                        "6" | "six"   => 6,
                        "7" | "seven" => 7,
                        "8" | "eight" => 8,
                        "9" | "nine"  => 9,
                        _ => unreachable!(),
                    }
                };
            }
            total += number;
        }
        println!("Part 2 Total: {}", total);
        println!("Note that this answer will be wrong because the problem has
                  overlapping patterns in the input data that is expected to be
                  found. The regex crate does not support this.");

        Ok(())
    }