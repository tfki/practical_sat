pub mod graph;
mod test;

use crate::cnf::literal::{Literal, Variable};
use crate::ex1::graph::Graph;
use crate::solver::{Solver, SolveResult};

pub fn find_k(graph: Graph) -> u32 {
    let mut num_colors = 0;

    loop {
        num_colors += 1;

        let mut solver = Solver::new();

        // each vertex must have at least one color
        for v in 1..=graph.num_vertices {
            for c in 0..num_colors {
                let var = vertex_color_variable(graph.num_vertices, num_colors, v, c);
                solver.add_literal(Literal::new(var, false));
            }
            solver.add_literal(Literal::clause_end());
        }

        for edge in &graph.edges {
            for c in 0..num_colors {
                let a = vertex_color_variable(graph.num_vertices, num_colors, edge.0, c);
                let b = vertex_color_variable(graph.num_vertices, num_colors, edge.1, c);
                solver.add_clause(&[-a, -b]);
            }
        }

        if matches!(solver.solve(), SolveResult::Sat) {
            return num_colors;
        }
    }
}

fn vertex_color_variable(_num_vertices: u32, num_colors: u32, vertex: u32, color: u32) -> Variable {
    Variable::new(1 + vertex * num_colors + color)
}