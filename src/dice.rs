use rand::{distributions::Uniform, Rng};

pub fn throw(total: i32, faces: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..=faces);
    (0..total).map(|_| rng.sample(&die)).collect()
}

pub fn roll(total: i32, faces: i32) -> i32 {
    let throws = throw(total, faces);
    throws.iter().sum()
}

pub fn explode(total: i32, faces: i32) -> i32 {
    let initial = throw(total, faces);
    let exploded: Vec<i32> = initial
        .iter()
        .filter(|x| **x == faces)
        .map(|_| roll(1, faces))
        .collect();
    [initial, exploded].concat().iter().sum()
}
