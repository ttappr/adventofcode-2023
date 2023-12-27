[LANGUAGE: Rust] 🦀

Around ~120 lines of code; everything is organized and makes sense. Behind the scenes, I have a `get_hand(str) -> Hand` function that returns the best hand a string of cards represents. The `Hand` objects are sortable per the order required in the puzzle. Adding in wildcard support about doubled my code size.

The algorithm for determining what best hand the input string represents operates like a state machine: a frequency array is populated where each index represents a card, then the array is iterated over. The count of each card type transitions the current `Hand` into better hands.

[Full code](https://topaz.github.io/paste/#XQAAAQD2EgAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9rGGDf1vDSdJHen5RICXTboMRjdWoG5SXYuwk7CJW9Hqk2ytKjP+oJXPaQ8amPm2wwGMPX+EBt5sTWpmq9988hg6hpvxYpC1/6lVJw6b/Ol7QeTpbuW9rjKyMZ5f9hVYfk29upp/pGwlFGLSWAz44aVaakeJRPF3JN7wVfSnxW/XvffKFXec8koPi5QLilWeX5HaxX/8rS9H9h2b3FkmPWzhbsQEXigSs4Rj8PID1+BouUlo0/fv4F9dAb6xMgvysMGP3TyeEsTG6feWEwI4/rUM8eCtdpGHiVVpiWz6TwzhLpA5KjIzVzLC6t6hDiSQ7nWfM0UDfc2yoXv1C9fd7dt6wag9ntNtouDILep1CEmWsqS9sqZ0sxlHOZUCaeAV7gVrAGLApG002Gsags5yCITnhVIEjndrsNu/47pZZP1QzxpzdKrUZeRN00WQXXartcmlWNfziMfiiHYiDI++y6UUABZNgJ/75nNjfXyD3ZCbUo7xrKn9Q8xj6v0ksiFoCrg5SvXmUWjk2z3C37z6wxHtEbZflEj4MFtEJkKbOMpqq+U9PqwY0f9dfnv4oi+tfcMbv7wdCl+O0WZDIc+JnuaWDi/f1NMF4wzT4re21Y+XgxiMO2XTrtmMO77mLm31/5PK9kV4WfZnNBau9pwM+QicQ+q77gsW31Hp1QQRTvE+qDMyIJ0wIknw5hxZvxn3ZuG2lEGtgW5+jJbTSEP7JemScClCk9GJ0OZs9djJtVdgq67YQ36BnfZuS0NjXsV6QUTbnQTrCXdbMl3gDbGTAgBLUlGc7Mdu+rqhdiEpeweIk/Xi957yICKMnuFkVF/li1oEo9d8Ic6yBtnFaov7RIvkdRTyUusLR5QSUQVU7EoDa5h9z7Jhx1Bj8VXcdonq/SxdI8E4POF3khFNifyD+qZvW4PKo5bp3mh+5cxwAZ7+XVvRpOm51ISl5lenmKmZIKtTz/ashA+jhoD4a6pfxuJbRij55aj/ame4iA6G/yKT9Mgks+0wpiUYB5tse194x+f0TtKtQYKcuDjc35mTXT3YdIIDGZxedN9RXlrBRLatyfi4gT0+HbmpUZT6vcpi9ECqG6VPr5ffeQ4d4ZomJEKJFNT1agNTC3V72j8jHCdLelOvpUl4woOAF0g9LVdxhDhj3f2ImvGVcvFaubPx/OdlLYueRPVE+pJsoKTS0u6LpAJ8DKmDmNh9P8S7E+G54qkJ12pypN7vkUWfKwP2ktXbMtqBcWnW3go8S2OH0vRENmN3o08i+XP3U/hzxrCuQfNqzki64ccwY/XfrqgIYnfrTs2Njl0yaUb3l8SjLSfZOLLCmjN2AcS/ZRef5BoK4PeLDI6jZKS2gqVUNM2zDkrCGvvkogyd3TnqQa0I5mxKIaurXu36V3FR5MOr/MeWwnQcJUMflkarNiN91SYub7ITZOTH9wfXJvrcMOZsZ9JfBFECy8SXR68k7OeByaNFaPJolhBjaSIfcQ7BwjZhzrKNDJlY6YFK6FG7fWQPEk6kTQGOljjJuEk95M1ILK+k+CMJtjQ9g/eCZgowVAx9O2qxbfq7U4Cm3pHIaJ2I1/WsqngDwMwbsYMc9nL+DIfoAM7qcLTD3cRTLTd4RUdM8hU/X1r5W+L+bEwDsoX21/FdBteAnx4OWZVQoPYFuHtx3SejCiEy+vp1qfX+7qKE0xuhpBjewy6YiiWvSH1EgtCwTzkz+wCwXQGbWagSrSxCwR5WF8G/kpCKYT+hbMSgpvnW4hhgfQO+zyuaBM5badT2uG4fpX4cPgaYDMoEj166mTk0HhLcm5ocCN7EX8g5ITSkbb+Ihs0VNQW/eLtSQ9Gt8IpoN1LchDREJFnMEm0NGu4RH6z9gO2Xb8eYZLNvQOEKMHMi9PC9f8kQHML694daQNEd5VF2O5j6RJtD/UWC00dHIt5npH/9mgPog==)

    fn solution(path: &str, part: Part) -> Result<(), Box<dyn Error>> {
        let file   = File::open(path)?;
        let reader = BufReader::new(file);

        let mut hands = Vec::<(Hand, u64)>::new();
        let mut total = 0;

        for line in reader.lines() {
            let line = line?;
            let hand = get_hand(&line[0..5], part);
            let bet  = line[6..].parse()?;

            hands.push((hand, bet));
        }

        hands.sort_unstable();

        for (i, (_, bet)) in (1..).zip(hands.iter().rev()) {
            total += bet * i;
        }

        println!("Part {:?} Total Winnings: {}", part, total);

        Ok(())
    }