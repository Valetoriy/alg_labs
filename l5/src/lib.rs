#[inline]
pub fn floyd_shrtst_path(
    graph: &Graph,
    mut from: usize,
    to: usize,
) -> Vec<usize> {
    let num_verts = graph.num_verts;
    let mut min_dist = vec![vec![100000; num_verts]; num_verts];
    // Т.к. у нас нет несвязных графов инициализация нулём не приводит к ошибкам
    let mut min_path = vec![vec![0; num_verts]; num_verts];

    for &GraphEdge { from, to, weight } in &graph.edges {
        min_dist[from][to] = weight;
        min_dist[to][from] = weight;

        min_path[from][to] = to;
        min_path[to][from] = from;
    }

    for i in 0..num_verts {
        min_dist[i][i] = 0;
        min_path[i][i] = i;
    }

    for k in 0..num_verts {
        for i in 0..num_verts {
            for j in 0..num_verts {
                if min_dist[i][j] > min_dist[i][k] + min_dist[k][j] {
                    min_dist[i][j] = min_dist[i][k] + min_dist[k][j];
                    min_path[i][j] = min_path[i][k];
                }
            }
        }
    }

    let mut out = vec![from];
    while from != to {
        from = min_path[from][to];
        out.push(from);
    }

    out
}

pub struct GraphEdge {
    from: usize,
    to: usize,
    weight: usize,
}

impl GraphEdge {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { from, to, weight }
    }
}

pub struct Graph {
    num_verts: usize,
    edges: Vec<GraphEdge>,
}

impl Graph {
    pub fn new(num_verts: usize, edges: Vec<GraphEdge>) -> Self {
        Self { num_verts, edges }
    }

    pub fn num_verts(&self) -> usize {
        self.num_verts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floyd_shrtst_path_works() {
        let g = Graph::new(
            9,
            vec![
                GraphEdge::new(0, 1, 7),
                GraphEdge::new(0, 3, 10),
                GraphEdge::new(1, 4, 9),
                GraphEdge::new(1, 2, 27),
                GraphEdge::new(2, 8, 15),
                GraphEdge::new(3, 4, 8),
                GraphEdge::new(3, 6, 31),
                GraphEdge::new(4, 5, 11),
                GraphEdge::new(5, 7, 17),
                GraphEdge::new(5, 8, 15),
                GraphEdge::new(6, 7, 32),
                GraphEdge::new(7, 8, 21),
            ],
        );

        assert_eq!(floyd_shrtst_path(&g, 0, 8), vec![0, 1, 4, 5, 8]);
    }
}
