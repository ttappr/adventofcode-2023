[LANGUAGE: Rust] 🦀

Intuitively, Dijkstra is an obvious option for the puzzle, but figuring out how to implement adjacency lists made it interestingly difficult at first. I actually had my code producing correct answers for about an hour while I pored over it looking for a bug; then I figured out I had been looking at the wrong expected value for the sample grid on the puzzle description page.

[Full Code](https://topaz.github.io/paste/#XQAAAQA1FQAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9pmAcjlY97+7/rzJ1/57GANCQoh/Sds7WSwOIt2GkHHjwxitpmy+lL3wPkfAEQj12MwvZ+sbS+MPpx4hR0zVA2VxIY0XfoJBUJQAfPoD9GGjUo62cuDxIpJjhdUK7a7C6zOzweWClrXnxHoMeQce0BAkSEvEPZalUNOjLgZ+X+cYeIGj7R3u8kxxi176KhWpgbb+spWXJ3s1Bu/damU3QW1DYKHh4S6JIcmJBWE4O0vsjed0Pb9XhuFy6HXCRIt2pvWwgCe2UrX+b1AIzueutmoQ76JTiGLUcfgnwyy378B6Mv8tueonFcqyB4RXE0uRqtKE1J/gM4/Fmdl32ZSJ8+dMTUKgdztZyAoBG+ft6Z44ztXX1yJpOMt/ocgmAELSrzS4OYW6z2RZxNYaXyuFTWUDXcb9LK1Baq3cm5lfrKJ6UWijK9xl8KxwInuz4wYAvk7/VIgsC7ovk/XSEfsM1CpvDR2ieD0402WN+UN/anEQ4q2ZPqWomD8m4oojrnPdOupyuFriS73DXGIm9IsaKGw0Tgzl0JZpX4U/JQD/pkk0ttWUW2QQ1Q3G6z1LDoezwj+MVV09rCCrGqpWRxYY1DXfy/ImDXfKUgDQA2QV0131qdLAT9gt5wuQ9arhPpP1+mXE63WG63607jD0R7cd0VqTHcl2M+rqIDYAc0B75qrIWvgYFTTetXpsRmie3kreU6ZybEV6WlW0DICe/kU7LnlF4EOnULlDrYxO4cxHiIvbY8U1S6T7JEuo0DBN+KsrTUE8jfrSGdauxDrLrViz3Xa0xRae7yylGdJ1StxF+n7IozBrF43nrb+jTsYNi+8bwmqw6Mq5VAPBSzMVY5otUAt2xehjovDlViMq+/4zIUfrfc7xbgYlO3ZdzyppwmvxsshALxdV03oxV1fR5tpadwR+PDqgVN2+3BBY9i8j3RdzubWCZihESd2YL5UuaAV3fZ9lRG+ltPJpN7nIKUzqlUL3LiclLTHaGTVweRC67ABs+5kwEt/r3PP6bIScyAbejBOsYZYi9gjBTii3swCfdoZauWLng11xbNF/0CKSHquivDzFDpNEzTZMbXOTGazLvrMYsqz/SdHdq86YMfyZCYAJtUTn48bVF0VI/kFxi/GeQJIVkjt2Z7ZFTEYyfOMAKUPtZ8cQu3P/y/JYpZqATwX/s8bvIwCxq+1Vw0Lhj3UtJgO80hHMVM21gYZotc7rl5sTaIbUFC6PmtwKp1Vq9o8Y/6BL20QKWFnZ2IQlkHX6EYb7HnAS79Nkr36D+htJYLPny38q7SG+cRa6PXqP6r5dWYpfEfYaYnxdtpOFKU+BlDvLVFb3eEdPyaRvkoURN9AhCrShW7qIUO5z/JSS3ElvgOepAEdBBGgxZs7VOAGRG2tjFMkzahNg+5auvTFohyGYONEysQPN+WAugYSNn/ZIDNYx55Ur8xFxH/kpkd17JtkgNboUBHabLMc17UToqsTpxQdS5YiLwoi9Fbp2+DIUDknsyCIuOTCX8nwbMOH58F3o6f2nyNQ0FmVjDUH+ElQ/DpQCPYYYctaHPft35Bnt0TAMkqaMiQRBcZ+qBu3G+MooNuy8C9ifa56umwdTMeJ1ZnXucF3pLKr5y3gCgcnUMRxUJujIubsuc+b/hbv6TM8CZxKEDe9d2T1LweapURvcWlbs2tOaTzo4wfuKZAI9TK0AxF5MJfC5qz0HokUy1YtmJ8M3j1ukfk68H5ScZdiy6HcINVmNcy8E+ZcOX9lmiukh6BClvSn+x90xr0y8Lr0JP8GLB0gNyG1oLeg5fCTWw6htI9qs3AvIP5bYO10A2C8g3mfJuizOIEIfxQWAi66xanASBgSmoczynvHfA0WOeT+Ge90s+lsw0YN7bFOvgOVgewDuFDC3uVohDkJ/UdYuh9sx0LTSlhNVnoYGkFw/c+1qSMrT1Ej/jLpFe7sGg8Z16JqXL8cGKUvzG1w8KI0oAJvJK8X7L6TgN6b/5/U6LyCelF31iFCKWhLHRsPxEV3J1JQ5g+OBm1vzHPoKG95t5H8Ppqu5TUmX6IkY0ErXeZEJya3y/povf5f7dgBXqPRizTm0dgm09aq9VLAjAs2W9KdxfKWwImYDY5+Qbd6kP5xsH2liCtkctc3u+3xcCnk8H7iaiql3p/rM8nJJogJnm4NPq+j0eBLC1fDzl70rh514oPasjTdVZu3F6gxhGWK97DlRXA54MreMnruO2UwP+sFyj0MdBMVcsZGrGoJW//ty/9i)

    fn dijkstra(g     : &[Vec<u8>], 
                start : Vertex, 
                min   : u8, 
                max   : u8) 
        -> i32
    {
        let (m, n) = (g.len(), g[0].len());
        let mut heap = BinaryHeap::new();
        let mut cost = HashMap::new();
        let mut seen = HashSet::new();
        let mut best = i32::MAX;

        cost.insert(start, 0);
        heap.push(Reverse((0, start)));

        while let Some(Reverse((_, v))) = heap.pop() {
            seen.insert(v);

            for adj in v.adjacent(m, n, min, max) {
                if !seen.contains(&adj) {
                    let f = |c| c + (g[adj.i][adj.j] - b'0') as i32;
                    let c = cost.get(&v).map_or(i32::MAX, f);

                    if c < *cost.get(&adj).unwrap_or(&i32::MAX) {
                        if adj.pos() == (m - 1, n - 1) { 
                            best = best.min(c); 
                        }
                        cost.insert(adj, c);
                        heap.push(Reverse((c, adj)));
                    }
                }
            }
        }
        best
    }