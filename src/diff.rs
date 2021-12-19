type Diff<T> = Vec<(u8, u32, u32, Vec<T>, Vec<T>)>;

pub(crate) fn levenstein<T: std::cmp::PartialEq>(s: &[T], t: &[T]) -> Vec<Vec<u8>> {
    let mut distance = vec![vec![0; t.len()]; s.len()];
    let mut operation = vec![vec![0_u8; t.len()]; s.len()];
    (1..s.len()).for_each(|i| {
        distance[i][0] = i;
    });
    for j in 1..t.len() {
        distance[0][j] = j;
    }
    (1..t.len()).for_each(|j| {
        for i in 1..s.len() {
            let substitution_cost = if s[i] == t[j] { 0 } else { 1 };
            let deletion = distance[i - 1][j] + 1;
            let insertion = distance[i][j - 1] + 1;
            let substitution = distance[i - 1][j - 1] + substitution_cost;
            distance[i][j] = if substitution <= insertion && substitution <= deletion {
                operation[i][j] = 0;
                substitution
            } else if insertion < substitution && insertion < deletion {
                operation[i][j] = 1;
                insertion
            } else if deletion < substitution && deletion < insertion {
                operation[i][j] = 2;
                deletion
            } else {
                operation[i][j] = 0;
                substitution
            }
        }
    });
    operation
}

pub fn diff<T: std::cmp::PartialEq + std::clone::Clone + std::marker::Copy>(
    source: &[T],
    target: &[T],
) -> Diff<T> {
    let o = levenstein(source, target);
    let mut i = (source.len() - 1) as i64;
    let mut j = (target.len() - 1) as i64;
    let mut ret = Vec::new();
    let mut raw = Vec::with_capacity((i + j) as usize);
    let mut raw_sub = Vec::new();
    let mut raw_type = o[i as usize][j as usize];
    while i >= 0 && j >= 0 {
        if raw_type != o[i as usize][j as usize] {
            if !(raw.is_empty() && raw_sub.is_empty()) {
                ret.push((
                    raw_type,
                    (i + 1) as u32,
                    raw.len() as u32,
                    raw.drain(..).collect(),
                    raw_sub.drain(..).collect(),
                ));
            }
            raw_type = o[i as usize][j as usize];
        }
        if o[i as usize][j as usize] == 0 {
            if source[i as usize] != target[j as usize] {
                raw_type = o[i as usize][j as usize];
                raw.push(source[i as usize]);
                raw_sub.push(target[j as usize]);
            } else if !(raw.is_empty() && raw_sub.is_empty()) {
                ret.push((
                    raw_type,
                    (i + 1) as u32,
                    std::cmp::max(raw.len(), raw_sub.len()) as u32,
                    raw.drain(..).collect(),
                    raw_sub.drain(..).collect(),
                ));
            }
            j -= 1;
            i -= 1;
        } else if o[i as usize][j as usize] == 1 {
            // insert
            raw_type = o[i as usize][j as usize];
            raw.push(target[j as usize]);
            j -= 1;
        } else if o[i as usize][j as usize] == 2 {
            // delete
            raw_type = o[i as usize][j as usize];
            raw.push(source[i as usize]);
            i -= 1;
        }
    }
    if !raw.is_empty() && raw_sub.len() == raw.len() {
        ret.push((
            raw_type,
            (i + 1) as u32,
            raw.len() as u32,
            raw.drain(..).collect(),
            raw_sub.drain(..).collect(),
        ));
    }
    match i.cmp(&j) {
        std::cmp::Ordering::Less => {
            let mut add = target[..(i.abs() + j.abs()) as usize].to_vec();
            add.reverse();
            ret.push((1, 0, add.len() as u32, add, vec![]));
        }
        std::cmp::Ordering::Greater => {
            let del = &source[..(i + 1) as usize];
            ret.push((2, 0, del.len() as u32, del.to_vec(), vec![]));
        }
        _ => {}
    }
    ret
}
