use text_io::read;

fn main() {
    println!("Enter the number of vertices");
    let n: usize = read!();

    println!("Enter the Cost Matrix: ");
    //let mut cost_vector: Vec<Vec<usize>> = Vec::new();
    let mut cost_matrix: [[usize; 10]; 10];
    let mut i: usize = 1;
    let mut j: usize = 1;

    while i <= n {
        while j <= n {
            cost_matrix[i][j] = read!();
            j += 1;
        }
        println!("got here");
        i += 1;
        j = 1;
    }

    println!("{:?}", &cost_matrix);
    let mut visited = [0usize; 10];
    visited[1] = 1;

    let mut count = 1;
    let mut min_cost: usize = 0;
    let (mut u, mut v, mut a, mut b): (usize, usize, usize, usize) = (0, 0, 0, 0);
    println!("The Spanning tree's edges are: ");
    while count < n {
        let mut minimum = 999;
        i = 1;
        j = 1;

        while i <= n {
            while j <= n {
                if cost_matrix[i][j] == minimum {
                    if visited[i] == 0 {
                        continue;
                    } else {
                        minimum = cost_matrix[i][j];
                        a = i.clone();
                        u = i.clone();
                        b = j.clone();
                        v = j.clone();
                    }
                }
                j += 1;
            }
            i += 1;
            j = 1;
        }

        if visited[u] == 0 || visited[v] == 0 {
            count += 1;
            println!("Edge({},{}) = {}", a, b, minimum);
            min_cost += minimum;
            visited[b] = 1;
        }

        cost_matrix[a][b] = 999;
        cost_matrix[b][a] = 999;
    }

    println!("The minimum cost = {}", min_cost);
}
