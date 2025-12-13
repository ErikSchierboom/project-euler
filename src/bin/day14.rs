use std::collections::HashMap;

fn main() {
    let mut longest_chain: usize = 0;
    let mut longest_start: u64 = 0;
    let mut chain_lengths: HashMap<u64, usize> = HashMap::from([(1, 1)]);

    for n in 500_000..1_000_000 {
        let len = chain_length(n, &mut chain_lengths);
        if len > longest_chain {
            longest_chain = len;
            longest_start = n
        }
    }

    println!("{longest_start}")
}

fn chain_length(n: u64, memo: &mut HashMap<u64, usize>) -> usize {
    if let Some(&len) = memo.get(&n) {
        return len;
    }

    let len =  if n % 2 == 0 {
        1 + chain_length(n / 2, memo)
    } else {
        2 + chain_length((n * 3 + 1) / 2, memo)
    };
    memo.insert(n, len);
    len
}
