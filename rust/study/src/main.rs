trait Node {
    type Coord;
    fn new(&self, x: Self::Coord, y: Self::Coord) -> Self;
}

trait G
    where Self::N: Node<Coord= <Self as G>::Coord>,
{
    type N;
    type Coord;
    fn as_node(&self) -> Self::N;
    fn create_node(&self, x: Self::Coord, y: Self::Coord) -> Option<Self::N> {
        Some(self.as_node().new(x, y))
        // None
    }
}

#[derive(Clone, Debug, Default)]
struct SN {
    x: usize,
    y: usize,
}

impl Node for SN {
    type Coord = usize;

    fn new(&self, x: Self::Coord, y: Self::Coord) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Default, Debug)]
struct Graph {
    n: SN,
}

impl G for Graph {
    type N = SN;
    type Coord = <SN as Node>::Coord;

    fn as_node(&self) -> Self::N {
        self.n.clone()
    }
}

fn main() {
    let g = Graph::default();
    println!("{:?}", g);
    let m = g.create_node(1, 5);
    println!("{:?}", m);
}