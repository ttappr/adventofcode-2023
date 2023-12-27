
type Graph<'a> = HashMap<&'a str, &'a [String]>;

pub struct Karger<'a> {
    graph   : Graph<'a>,
    n_edges : usize,
    s_verts : Graph,
}