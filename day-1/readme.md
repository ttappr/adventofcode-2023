[LANGUAGE: Rust]

Well this should have worked. Unfortunately, Rust's `regex` crate doesn't support lookahead expressions to help weed through overlapping matches. Ultimately, I parsed the input data in a Python REPL with a lookhead expression.

|UPDATE| I came back to this and removed the `regex` crate dependency and implemented a dynamic programming style solution to matching the numbers in the strings. The code linked below has the updated solution.

[Full code](https://topaz.github.io/paste/#XQAAAQAKDwAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKso7PnMTN9d+YxdhAEiR5YtpUUnvlic8mtEwPfofK2k0X18KgmqJylqWVLiOhCuXPwvk2/q6bzD4toyUMgzUgb+8g17m/6l+32r8yqZ82dNJKsF4w+ykDgRfOxWnOAw4qr1ATNhfyIF172yh5N30RXOPh9em5YIkXVgDq44rm2HDR2ibrNEXLSlsIYgNQrwQTl3KK6HER9D9GgwSS8R841NTFzAKAsUyVChjlIA9QLeMR+De9EZPV89KQTa0Cv6nisfUdnk/G+EK+ft2IaKT8nUDgHRIvDepv/LgulaZKkvsEJlzs6Bo9AGzJc0ThPGQ3GvSlUU9i8gkMEeSUcZX2KWTluLdJGLE3IxntytXkPwJGf8ScU5yXP8MWk9egFfmn/fDFExD6KtffdO1z8/wuat3dcj+6AaFiZ3yWH4pq1f2nYVFP7UH5JzaSHul0xcJwvFPyLJjieFJ5PMThinOVVwbAT0C+H4P1/h7e9j8ccuuM8ww2MCs2e+IWUBqtzgu2UdwEqKqlfbjhx+Fj7qJVaS7svdG6U4wkNkpCZTMQlXT99Y/N8KgVg0xdgoHyf2U9Kk+R0akQ/ejXWN/c7AOAZpCv/muQClApnyaLzWU1yAGm7d2cntOBbxcOC9vUMkkB/6pzZoTzdgIAxACmKAG0AKVruB5GuacPsuHthaJeZdnrtVupdMGoZhoF9uqNeLpTcJG1Gn3+bnRAEWYf6YvMlhJPZjYggZJLyh0aD7Z1kgzrJUTBI7Hj6S+R+Z+k/1pW8tftWIwglHkbd7j24CR44LWj3zVKl7+TqObZuVngFwKTJzB0JX1ABYR2knZ0HfCKAMXFaiHLtK9k1SCOR5dbukZAyBv8AUeJlVTjWE12QVY7SvhGpRdIizJ0gY+es2q290x9CTiFbnBfp6Ql6pU+ebhlOWZO+mgS+FuF8td+J7VkzLgVeXknUYtDCm5GEYOPRfoI0PjwYiP72F4Mlb2gTgiQmLS8WVeDDuoDHgxfaNUydZA3OwHRP4xjGeJGpCWvQqtb246dpmZ0zXUCE6umf0MlYy4lh0mZ7sH5XgXqGpCu11geE13CJx0NOIQCEyW53rHJf/8mRj7jEs7j9Lss8XpaMq4mxsXm899ywG/j/Yt8kf6XvTuFR/zb1f4IQa0dgRc0DpozkP83o3lLHe5TzmsTpUlmpZYfdSQb3YuV6a0S7XGLyH9jjtygLTkluj+z1eSDhYzCFSdpTErMnkfT8kN/YriFckV3Z8GtIDhLqKPT8V0hOAhaGcu3PJ14oIE3RPf69nzM54yBoSh8GllfBGGayxdKzTliL6u7eM9T4eIgNenrGgz/CH4gJ1vmNlSxRTv5gdYmySHrkepYH44IzCRS0aXo5o9SDKa11RgDF6tJ2PbCH+1Jjmg7X552N+DJgcVWExSGDSaHNZq7CcBdNlstRNQiud/x703SAxwZNlv5qYGiW+83RCPrzBw6Px1HV62bxcNLxQmFc3YaPz0+S/krCxa+LqBTppKw7y0WGh3YDQ3eiiTcqp4X80AsLvYxF3r3KuC8W1GJsXIUiOC4Jc6NYEWrIohr4KTZ8iMpf3qZ9AwEhrT5uFvz7wM8oXVVSk0bPo34pPU6f0SG8zr0JrM0U74d07ybrs9DDS9nB+MWk13XCjwsz9mHuIpT6xdemR++0Z6+AV1YLvo5+V5cCySkiE8g6uHDFnuM0rbIZWrzcQRc/pEO021ctxvVg/hXdZ3h8XBaDAGzreGD243NP87AozL/FkOcNuJ0nARHY4tA04CzjD1lDKmsLks/Z/8ZHDG4=)

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