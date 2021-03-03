use std::io;
use std::str::{SplitWhitespace};


// TODO: runtime error:
// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', Main.rs:15:40
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// formatar print de valores float???



#[allow(unused_variables)]
fn main() -> io::Result<()> {
    let mut buffer: String;
    let stdin = io::stdin();
    let mut iter: SplitWhitespace;

    // tres iteracoes
    for i in 0..3{
        
        // primeiro produto
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        iter = buffer.split_whitespace(); 

        let codigo1: i32 = iter.next().unwrap().parse::<i32>().unwrap();
        let quantidade1: i32 = iter.next().unwrap().parse::<i32>().unwrap();
        let preco1: f64 = iter.next().unwrap().parse::<f64>().unwrap();

        // segundo produto
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        iter = buffer.split_whitespace();

        let codigo2: i32 = iter.next().unwrap().parse::<i32>().unwrap();
        let quantidade2: i32 = iter.next().unwrap().parse::<i32>().unwrap();
        let preco2: f64 = iter.next().unwrap().parse::<f64>().unwrap();

        println!("VALOR A PAGAR: R$ {}", 
            (quantidade1 as f64 * preco1) + (quantidade2 as f64 * preco2)
        );
    }

    Ok(())

}
