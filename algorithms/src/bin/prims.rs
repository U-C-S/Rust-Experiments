use text_io::read;

fn main() {
    let mut minimum: usize = 999;

    println!("Enter the number of vertices");
    let n: usize = read!();

    println!("Enter the Cost Matrix: ");
    let mut cost_vector: Vec<Vec<usize>> = Vec::new();
    let mut i: usize = 1;
    let mut j: usize = 1;

    while i <= n {
        while j <= n {
            cost_vector[i][j] = read!();
            j += 1;
        }
        i += 1;
    }

    let mut count = 1;
    println!("The Spanning tree's edges are: ");
    while count < n {
        minimum = 999;
        i = 1;
        j = 1;

        while i <= n {
            while j <= n {
                if cost_vector[i][j] == minimum {
                    if true {
                        //todo
                        todo!();
                    } else {
                        minimum = cost_vector[i][j];
                        todo!();
                    }
                }
                j += 1;
            }
            i += 1;
        }

        if true {
            //todo
            count += 1;
            println!("Edge()"); //todo
            todo!();
        }

        todo!();
    }

    println!("The minimum cost = {}", todo!());
}
