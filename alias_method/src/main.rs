use std::fmt::{self, Formatter};

struct Bottle {
    id: usize,
    prob: f32,
}

impl fmt::Debug for Bottle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bottle")
            .field("id", &self.id)
            .field("prob", &self.prob)
            .finish()
    }
}

fn main() {
    let probs: [f32; 5] = [0.01, 0.41, 0.08, 0.32, 0.18];
    let mean: f32 = 1 as f32 / probs.len() as f32;
    println!("The mean is {mean}.");

    let mut fulfilled: Vec<Vec<Bottle>> = Vec::new();
    let mut overfilled: Vec<Bottle> = Vec::new();
    let mut underfilled: Vec<Bottle> = Vec::new();

    // Now separate probs into three groups,
    // overfilled, underfilled and fulfilled ones
    for (id, &prob) in probs.iter().enumerate() {
        let bottle = Bottle { id, prob };
        put_back(
            bottle,
            &mean,
            &mut underfilled,
            &mut overfilled,
            &mut fulfilled,
        );
    }
    println!("Binned as {fulfilled:?},\n{overfilled:?},\n{underfilled:?}.)\n");

    // Iterate until all containers are fufilled
    while overfilled.len() != 0 || underfilled.len() != 0 {
        // Pour overfilled container to underfilled ones
        let backup_bottle = Bottle {
            id: 0,
            prob: 0 as f32,
        };
        let mut over_bottle = match overfilled.pop() {
            Some(b) => b,
            // If there is no overfilled bottles,
            // then there must be at least one underfilled bottle left.
            None => underfilled.pop().unwrap(),
        };
        let mut under_bottle = underfilled.pop().unwrap_or(backup_bottle);

        if (over_bottle.prob + under_bottle.prob) >= mean {
            let diff: f32 = mean - under_bottle.prob;
            let poured_bottle = Bottle {
                id: over_bottle.id,
                prob: diff,
            };
            fulfilled.push(vec![under_bottle, poured_bottle]);
            over_bottle.prob -= diff;
            put_back(
                over_bottle,
                &mean,
                &mut underfilled,
                &mut overfilled,
                &mut fulfilled,
            );
        } else {
            under_bottle.prob = over_bottle.prob + under_bottle.prob;
            put_back(
                under_bottle,
                &mean,
                &mut underfilled,
                &mut overfilled,
                &mut fulfilled,
            );
        }
        println!("Current bottles: fulfilled: {fulfilled:?},\noverfilled: {overfilled:?},\nunderfilled: {underfilled:?}.\n");
    }
}

fn put_back<'a>(
    bottle: Bottle,
    &mean: &f32,
    underfilled: &mut Vec<Bottle>,
    overfilled: &mut Vec<Bottle>,
    fulfilled: &mut Vec<Vec<Bottle>>,
) {
    if bottle.prob < 10e-5 {
        return;
    }

    if (bottle.prob - mean).abs() < 10e-5 {
        fulfilled.push(vec![bottle]);
        return;
    }

    if (bottle.prob) < mean {
        underfilled.push(bottle);
        return;
    }

    overfilled.push(bottle);
}
