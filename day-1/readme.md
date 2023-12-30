[LANGUAGE: Rust]

Well this should have worked. Unfortunately, Rust's `regex` crate doesn't support lookahead expressions to help weed through overlapping matches. Ultimately, I parsed the input data in a Python REPL with a lookhead expression.

|UPDATE| I came back to this and removed the `regex` crate dependency and implemented a dynamic programming style solution to matching the numbers in the strings. The code linked below has the updated solution.

[Full code](https://topaz.github.io/paste/#XQAAAQCDDgAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKso7PnMTN9d+YxdhAEiR5YtpUUnvlic8mtEwPfofK2k0X18KgmqJylqWVLiOhCuXPwvk2/q6bzD4toyUMgzUgb+8g17m/6l+32r8yqZ82dNJKsF4w+ykDgRfOxWnOAw4qr1ATNhfyIF172yh5N30RXOPh9em5YIkXVgDq44rm2HDR2ibrNEXLSlsIYgNQrwQTl3KK6HER9D9GgwSS8R841NTFzAKAsUyVChjlIA9QLeMR+De9EZPV89KQTa0Cv6nisfUdnk/G+EK+ft2IaKT8nUDgHRIvDepv/LgulaZKkvsEJlzs6Botzba7XHl+FYK1vaGhd3scHandRruK4STJlI8u6vzyufywjDo0TlK9poDzJgtL8uAfsfMknZAplaf9BMFynwwRslXd9g5ytV57C1GL51w0+H17Eyu0o9w5gGVAVpGbKxPb8do19bR8xExbl3SkTVVnX6KkDMNpGE78fUNMxkgX7uBG1rfFswYJXUI/StiHc5KeLsmzkvlNcbXn3veqv7VUDJny+ohyGZvh4soDc5fhlp4CHRXUQTvw3iPUxrWlVgNRpLDyUa8GiBMqb6sGURDB0nXwomRppwaKzHVW9Pg/Wp/qUrB/u5iAsrj8ILL1cZYyGrq7kSJlSbZS6n+Y1lBHF3zt96z3e7/5vax/urJV3gnFNttHEkmNiCoL0WM4MdhRQ90iJnXMw1fhdmjKLcgr1xICR363Li0bKmlaj06ALt8Iq9g0ABCqWSyMF9VLaPv+vTa3WXciseGJB53dauYbZ+4UX5ECpJryNHLgk0HVqKrb2Bfvwf3yL7puBfCOErgHYY7MLtxhm5AcmzTlcW+XzeoxNG1ELY4+rSS3ifsN8aHzl5eOi5VN3kz3F5CjuuYW1N06QH76mEd4/1bPBnTYyfdIQq10V072jyBaQTBchvyfxedBmL8VW1lpwh20IkoJcBSDXUlvFWUQjuCFJp5C+pJIuXOrtLIh5HEcQsc6qLSYpdDFm0eCpZgykhqfB1zjDEkS5QL1D0nBvAh1xHVFp7aa747JVFvV5JIsJrxU238hSNAL+YVL/XCDhSS9VHm6g9M+8arwsoCUtUqMbtKuB8eCuPHZgU+5RbLNWPAYnYUD8oBy16oUZhvO2HsuWHkkiMCpqUtwJzqFZpeSxJoMhJI6wVJr7QTBBhst/4umyYK809KwZpZQbVIeeU5vFVXZn2fR6t3BYmPiFMOAM8bjbsU0UMgkwm2C1RaYxSdwGN6mrBU5GrQJUqCUztJXpGB5ZHGco/I7PzK7kT+1O3Z3+I0coCZQfOqFOEGMT/gMG2lUiL5/4UOakxEr9aKdP5j+O+HdDpDfsZhuRtJoJqJL+jvwqusWmAUSK8VzQXO98+vFblqjHsJ56eM5WgjWmIxuHKUfqF0fM+kAaBJFeM2wQAv+vMoqY/redCeQpoR5PDfx1KnkSrYzMV6AB05FdcIDRCGi7sYr5p2IFP+BgPDvzgJhq8llDd94p3/SJcSy6RdjKhTcMx32trs3W9YR15YlahpGMccFedLkJFuNBBMl0PburRhgp0JVn8n5EJPclEaZ5gj6YvkMeBQtdi1BQEF3gcvPCdMwvSk2+AM38cW6xQH72giAm4pGvO0mRNmyLMfIgt81oSaSuLSgAvidww178VDTszwGahwijjpoomeZhZQLMil3jGRNBi5fqvjOyBE3o9eDH2u9/mexD4KiimE08ysCJD32xM+VT9toATpvH5eDxYWLJVActxPao+dkGhgY52pn1v63eTv/8mYil)

    fn part_2(path: &str) -> Result<(), Box<dyn Error>> {
        let file   = File::open(path)?;    
        let reader = BufReader::new(file);

        let mut total = 0;

        for line in reader.lines() {
            let     line     = line?;
            let mut number   = 0;

            let (first, last) = match_nums(&line);

            let digit1 = first.ok_or("No first digit found")?;
            let digit2 = last.unwrap_or(digit1);

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
        Ok(())
    }