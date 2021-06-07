use text_io::read;

fn main() {
    let mut minimum: usize = 999;

    println!("Enter the number of vertices");
    let n: usize = read!();

    println!("Enter the Cost Matrix: ");
    let mut cost_vector: Vec<Vec<usize>> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i <= n {
        while j <= n {
            cost_vector[i][j] = read!();
            j += 1;
        }
        i += 1;
    }

    let mut visited = vec![0; 10];
    visited[1] = 1;

    let mut count = 1;
    let mut min_cost: usize;
    let (mut u, mut v, mut a, mut b): (usize, usize, usize, usize);
    println!("The Spanning tree's edges are: ");
    while count < n {
        minimum = 999;
        i = 1;
        j = 1;

        while i <= n {
            while j <= n {
                if cost_vector[i][j] == minimum {
                    if visited[i] == 0 {
                        continue;
                    } else {
                        minimum = cost_vector[i][j];
                        a = i.clone();
                        u = i.clone();
                        b = j.clone();
                        v = j.clone();
                    }
                }
                j += 1;
            }
            i += 1;
        }

        if visited[u] == 0 || visited[v] == 0 {
            count += 1;
            println!("Edge({},{}) = {}", a, b, minimum);
            min_cost += minimum;
            visited[b] = 1;
        }

        cost_vector[a][b] = 999;
        cost_vector[b][a] = 999;
    }

    println!("The minimum cost = {}", min_cost);
}
