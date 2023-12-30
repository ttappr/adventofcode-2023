[LANGUAGE: Rust]

Well this should have worked. Unfortunately, Rust's `regex` crate doesn't support lookahead expressions to help weed through overlapping matches. Ultimately, I parsed the input data in a Python REPL with a lookhead expression.

|UPDATE| I came back to this and removed the `regex` crate dependency and implemented a dynamic programming style solution to matching the numbers in the strings. The code linked below has the updated solution.

[Full code](https://topaz.github.io/paste/#XQAAAQDNDgAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKso7PnMTN9d+YxdhAEiR5YtpUUnvlic8mtEwPfofK2k0X18KgmqJylqWVLiOhCuXPwvk2/q6bzD4toyUMgzUgb+8g17m/6l+32r8yqZ82dNJKsF4w+ykDgRfOxWnOAw4qr1ATNhfyIF172yh5N30RXOPh9em5YIkXVgDq44rm2HDR2ibrNEXLSlsIYgNQrwQTl3KK6HER9D9GgwSS8R841NTFzAKAsUyVChjlIA9QLeMR+De9EZPV89KQTa0Cv6nisfUdnk/G+EK+ft2IaKT8nUDgHRIvDepv/LgulaZKkvsEJlzs6Bo9AGzJc0ThPGQ3GvSlUU9i8gkMEeSUcZX2KWTluLdJGLE3IxntytXkPwJGf8ScU5yXP8MWk9egFfmn/fDFExD6KtffdO1z8/wuat3dcj+6AaFiZ3yWH4pq1f2nYVFP7UH5JzaSHul0xcJwvFPyLJjieFJ5PMThinOVVwbAT0C+H4P1/h7e9j8ccuuM8ww2MCs2e+IWUBqtzgu2UdwEqKqlfbjhx+Fj7qJVaS7svdG6U4wkNkpCZTMQlXT99Y/N8KgVg0xdgoHyf2U9Kk+R0akQ/ejXWN/c7AOAZpCv/muQClApnyaLzWU1yAGm7d2cntOBbxcOC9vUMkkB/6pzZoTzdgIAxACmKAG0AKVruB5GuacPsuHthaJeZdnrtVupdMGoZhoF9uqNeLpTcJG1Gn3+bnRAEWYf6YvMlhJPZjYggZJLyh0aD7Z1kgzrJUTBI7Hj6S+R+Z+k/1pW8tftWIwglHkbd7j24CR44LWj3zVKl7+TqObZuVngFwKTJzB0JX1ABYR2knZ0HfCKAMXFaiHLtK9k1SCOR5dbukZA08L5TODOkIWtQIt5Ma8OOZmpSa1ytzlXe7LP84/9fwGj6Q5+1Ex9lQuo+p9+3jUsUuiABOkkqgQ4m48rNr/oY6flxh7RIL8B57SGSUMiql7t9xUzBzU2G7T0x+mjCOyQ2nJcr+qbu7vXRpL3U83QK4/A6a+3vRBVIwhahsQtLqUVnXsPwRVso7iDXwhfipMnEX6i7uZXr4vQXJg+3SNsgrNl4mnxTPn/LbJDg0NfeXTgPSRVCIASOUpdV3Ea3Lz7O/+5TehgCW4Cg0PXtKaqRww17BYt0CadGTjyYrsOl7EqyE9ePBpDGF4ag9Pvlf9U73qDKErSTH+D5E9EyPhLCX7Ytt5T9CVghkzVQiYl82dP3qw3mXfLxDYwCKX3cEE7NcrjVspG8JUXhx+9HpC/GzGNhxGvu41S181Uw3V3igKZlqAD4nHogxHLysCLn2WpvtVtzUyqk76g3+X/nRa+ZuB5su/z1U7a1exODuQtHH1Bcq+mXBYdGgNeofNc/oUQP+++ELmsmB5nfrc0KRp0p3MgmmXfZUYmx9ad/vKi5Hc8EsB8jgkivlJY8xrLQiP0FGJ+rCR2Cg4Af9KOr3yLZFWiCb82aMdN99cs6MBsEcy+7QupyXJwEftSrHqMxHx+L5CB5HU0wdPVZx0n2iZBylCt8kcD30jmxshdU0LxVW5AVbGUZ4xEY3jB9H9jxzMBhC50WzCFpWngpY1dOTTQKjSENC+SgON7Tcy8LbwIAAntNv4Y97DSwNErQmnq3S/+vB7rH6mNcWGysxrV77EQLOy134QZR5UWQKF2W2gh5BuGv4clBpJZzWWynmGWgS5bqTDmSTdQhjMLZkfF5C4sPOL1Y8eIyUZq+nkLRp1LIboor8avExxAIxyLEMBihRRU3m/GTjJ2m3qFXonluvWvMgP/fR6dg)

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