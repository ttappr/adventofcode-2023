[LANGUAGE: Rust] 🦀

Started trying to find a DP solution, but found it easier to go with a recursive approach with memoization. Below is the recursive routine that counts the number of ways the springs can be grouped.

[Full code](https://topaz.github.io/paste/#XQAAAQDPEQAAAAAAAAAX4IBAR1bGui774bxYSsb6EPKsnwDg3V7l0W1Vp9ploOHRAmXwtgmhurQ8sZBE/9xO/o7swzf31xtNa3nMKfcC2Wc3zvXPYrtkgQMseneBwYChk/EU/CLEdNIs7dTDO8mJFZLzaZ5gflAPKyg+WaZVU4viGDsDAZcVYIzdYF4Mwl1RNNqOZvqZFeUPdkJD0iOkgeiheuJXdtv3SjDpbp1mqlzMPAvmKt34hfvpsPyQCtMef4GVUtJT4LfX8nnHNE0y+EsVApAAs8gVuvG6/+fmNeaGWpJRXKh4/pYFqFbeu2XXAs/uo2JLZV5Y+V8BGaLEnR2bQqKrMFSBrgyb7ER8dJvwUrfjDeihV5bnshdyJxP5PKW9ZeR/94pMWJFaoldmAZfH/sfbVh8tuDFbNc2MIeKiMOMS9S8mNw2U5L+BMame2i9/EGsZ9/9n6rRLOgfFwnxtH1podW2TYR94BO+cXUf6g/nzHwWOr4WRAulsM7NqeRhJ96M+l28kuxFcmdrxAWm+HTCwCnicVbhZV36WE4Lw0HoPYBwWXJxh9Vg8U1D9aosPB/dhsGNAqGDbQg2OuSL7zDYFSaWfRl6Cg7dYvxYbDEPE6rXoQT0054LHnUw62i9xdYqZyhNCeWAAAd3wJaAylTRVBgrxGwfOeshVKQXxWvxC9Zv3MiCGieUnhp7a06wBODeqZWNdrZjsTa8A893PDNOykPqsF+Br1Gpzb70OeAQq7foaKr80tSsI1CnN8kzg6WgNvcIBUnmHF1XDV2vzYwJIqgJgWcwR8O2pRvE3JLGyljgxlp68Dyl6GCtjrVhnMiP80C0x9swMeKcnp3aLvJH8S7QK/gffVb6pWK1NvaB2p+ZkWX2iBLHzvhq/+0Hsap/APCafQUwN0EXHbOWtyxQvQfFj22bnFxq6jFMKnoOsMotJF7GFARwQqBQ8MuOOPVwxXE+0t3LyKREutIeSJxXIciJeeyR4VYIjt8wqmsuLCVFuOb/qP/0skRx0XPgcD4OFqgOxVxI7785pSy6AE0nOAj1vlBW/MUwetJPxChm0CWJubURnsM74VISPQ4ohgWYzbQNP0fhPoB44co3qwj2UoPkaaojf5Wugt0RWjoqwN3mcNOk8JuGUcAtVeLfT/ZMk4x2Wf5GB3ZLUgIGceD9HxUMr8aVAYQ2ihqn18UWnwvAI9ilSip3QxLbkH2SwZTrglhN94Iviwe9NPNOmXVgd4f1zSw+WJZkUR0xhw+GQMtZ2BtTuZHWD2XqJ99iOUeWo7hJDWHx7hpYZ85NDu3/V/WAT7OZ+TpjcRPJjvSL/f771z0RDB8ZR9HVtvaqp3tmvVX00UzdTVbHB9Kfz4jusC13oe1dZKlFxQZaMuQi6SuRCTioB0NfX9vzx6f3qDdV9UJigEjxx5nYM8P56B36ahgFnwmLYnF7pH4BTaMg26ltZPkNxj23qYRr++ftUHUR6bu3g00dYvqR763+ej/L17Af+snLR3B8S0hKZ5v0WSTWxCCgtiFSTijagPlAEsg+1q2YSFaMnNFCi2CQQFsZK2h3Gs8stv1o+am6zY1fY/hbsLgJv9HVVHz3AJcYQP37Qb8vuIhbll/lBmhhxMPa777YpiAi+CtaEZNaot6zO6Ewx4fbAs8daWmezMpie5fBpRi2/hP16w2YElFRGFtvlgpSwPK2hFkjRH32wrzsxFRSvtL4Imj5nNX8yRQCmNovUcG6CRvZcliEPJw3Zy+j39mKabdotBTQCqrFvmgJpNYoN+yjgb38b7Kmk/bKHduruvzK9p70RYa2I72Wz5TYQQu1VRtPSPzgRCV0/Si+U/+Qy4w4=)

`dmg` is the list of numbers represeting continguous damaged springs.
`unk` is the ruined map with many springs in unknown condition.


    fn recurse(dmg   : &[usize], 
               unk   : &[u8], 
               memo  : &mut HashMap<(usize, usize), usize>) 
        -> usize 
    {
        if let Some(&count) = memo.get(&(dmg.len(), unk.len())) {
            count
        } else {
            let mut count  = 0;
            let     space  = dmg.iter().sum::<usize>();
            let     limit  = unk.len() - space;
            let     span   = dmg[0];
            let     ulen   = unk.len();

            for i in 0..=limit {
                if i > 0 && unk[i - 1] == b'#' { break; }

                if unk[i..i + span].iter().all(|&b| b != b'.') {
                    if dmg.len() == 1 {
                        if unk[i + span..].iter().all(|&b| b != b'#') {
                            count += 1;
                        }
                    } else if (i + span == ulen || unk[i + span] != b'#')
                        && ulen > i + space {
                            count += recurse(&dmg[1..], 
                                             &unk[i + span + 1..], 
                                             memo);
                    }
                }
            }
            memo.insert((dmg.len(), unk.len()), count);

            count
        }
    }