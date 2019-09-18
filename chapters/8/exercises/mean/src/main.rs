fn main() {
    println!("{}", mean(vec![1., 2., 5.]));    
}

fn mean(v: Vec<f64>) -> f64 {
    let mut sum = 0.;
    let len = v.len() as f64;
    
    for f in v {
        sum += f;
    }

    sum / len
}
