use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

struct Cacher<T, I, O>
    where T: Fn(&I) -> O,
          I: Hash + Eq + Clone,
{
    func: T,
    values: HashMap<I, O>,
}

impl<T, I, O> Cacher<T, I, O>
    where T: Fn(&I) -> O,
          I: Hash + Eq + Clone,
{
    fn new(func: T) -> Cacher<T, I, O>{
        Cacher {
            func,
            values: HashMap::new(),
        }
    }

    fn get(&mut self, arg: I) -> &O {
        self.values.entry(arg.clone())
            .or_insert((self.func)(&arg)) 
    }
}

fn main() {
    let mut cacher = Cacher::new(|x: &i32| x * x);

    println!("{}", cacher.get(5));
    println!("{}", cacher.get(6));
    println!("{}", cacher.get(5));

    println!("{:?}", cacher.values);
}
