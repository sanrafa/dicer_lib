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

fn get_minimum(die_faces: i32, threshold: f64) -> i32 {
    if threshold < 1.0 {
        (f64::from(die_faces) * threshold + 1.0).round() as i32
    } else {
        threshold.round() as i32
    }
}

pub fn throw_pool(total: i32, faces: i32, threshold: f64) -> Vec<(i32, bool)> {
    let min = get_minimum(faces, threshold);
    let pool = throw(total, faces);

    pool.iter()
        .map(|d| {
            let success = d >= &min;
            (*d, success)
        })
        .collect()
}

pub fn explode_pool(total: i32, faces: i32, threshold: f64) -> Vec<(i32, bool)> {
    let min = get_minimum(faces, threshold);
    let initial = throw(total, faces);
    let pool: Vec<(i32, bool)> = initial
        .iter()
        .map(|r| {
            let success = r >= &min;
            (*r, success)
        })
        .collect();
    let exploded: Vec<(i32, bool)> = pool
        .iter()
        .filter(|(x, _)| *x == faces)
        .map(|_| {
            let die = roll(1, faces);
            let success = die >= min;
            (die, success)
        })
        .collect();
    [pool, exploded].concat()
}
