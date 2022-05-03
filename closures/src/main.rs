use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add};
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

//////////////////////////////////////////////////////////////////////////////
struct Cacher<T, K>
where
    K: Add + Eq + Hash + ToOwned<Owned = K>,
    T: Fn(K) -> K,
{
    calculation: T,
    dict: HashMap<K, K>
}
    impl<T, K> Cacher<T, K>
    where
        T: Fn(K) -> K,
        K: Add + Eq + Hash + ToOwned<Owned = K>,
    {
        fn new(calculation: T) -> Cacher<T, K>
        where
            K: Add + Eq + Hash + ToOwned<Owned = K>,
            T: Fn(K) -> K,
        {
            Cacher { calculation, dict: HashMap::new() }
        }

        fn value(&mut self, arg: K) -> <K as ToOwned>::Owned
        where
            K: Add + Eq + Hash + ToOwned<Owned = K>,
        {
            self.dict.entry(arg.to_owned()).or_insert_with(|| {
                (self.calculation)(arg)
            }).to_owned()
        }
    }
//////////////////////////////////////////////////////////////////////////////

fn generate_workout(intensity: u32, random_number: u32){
    
    let mut cached_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num + 5
    });

    println!("Intensity: {}", intensity);
    println!("Random number: {}\n\n",random_number);

    println!("FirstCalc: {}\n", cached_result.value(1));
    println!("SecondCalc: {}\n", cached_result.value(1));
    println!("ThirdCalc: {}", cached_result.value(5));

    /*
    if intensity < 25 {
        println!(
            "{} pushups!",
            cached_result.value(intensity)
        );
        println!(
            "{} situps!",
            cached_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!(
                "Run for {} mins!",
                cached_result.value(intensity)
            );
        }
    }
    */
}