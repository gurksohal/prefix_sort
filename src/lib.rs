use std::cmp::Ordering::Equal;

const PREFIX_SIZE: usize = 12;

#[derive(Ord, Eq, PartialOrd, PartialEq, Clone)]
pub struct Record {
    pub data: Vec<u8>,
}

pub fn builtin_sort(data: &mut [Record]) {
    data.sort();
}

pub fn prefix_sort(data: &mut Vec<Record>) {
    let mut ptr_prefix_pairs: Vec<(u32, [u8; PREFIX_SIZE])> = data
        .iter()
        .enumerate()
        .map(|(i, record)| {
            let mut prefix = [0; PREFIX_SIZE];
            prefix.copy_from_slice(&record.data[0..PREFIX_SIZE]);
            (i as u32, prefix)
        })
        .collect();

    ptr_prefix_pairs.sort_by(|a, b| {
        let cmp = a.1.cmp(&b.1);
        if cmp == Equal {
            data[a.0 as usize].cmp(&data[b.0 as usize])
        } else {
            cmp
        }
    });

    let sorted = ptr_prefix_pairs.iter().map(|&i| data[i.0 as usize].clone()).collect();
    *data = sorted;
}

#[cfg(test)]
mod tests {
    use crate::{builtin_sort, prefix_sort, Record};
    use rand::Rng;
    fn gen_input() -> Vec<Record> {
        (0..1000)
            .map(|_| {
                let mut rng = rand::rng();
                let mut arr = vec![0; 100usize];
                rng.fill(arr.as_mut_slice());
                Record { data: arr }
            })
            .collect()
    }
    
    #[test]
    fn test_builtin_sort() {
        let mut input = gen_input();
        builtin_sort(input.as_mut_slice());
        assert!(input.is_sorted());
    }
    
    #[test]
    fn test_prefix_sort() {
        let mut input = gen_input();
        prefix_sort(&mut input);
        assert!(input.is_sorted());
    }
}
