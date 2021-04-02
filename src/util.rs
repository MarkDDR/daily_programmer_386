#[derive(Copy, Clone, Debug)]
pub enum AddOrSub {
    Add,
    Sub,
}

pub fn generate_index_subtractions(up_to: usize) -> Vec<(usize, AddOrSub)> {
    let mut part_subs = vec![];
    let mut i = 0;
    loop {
        let mut n: i64 = (i + 2) / 2;
        if i % 2 == 1 {
            n = -n;
        }
        n = n * (3 * n - 1) / 2;
        if n > up_to as i64 {
            break;
        }
        let add_sub = if (i/2) % 2 == 0 {
            AddOrSub::Add
        } else {
            AddOrSub::Sub
        };
        part_subs.push((n as usize, add_sub));
        i += 1;
    }
    part_subs
}