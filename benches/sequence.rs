#[macro_use]
extern crate criterion;

use criterion::Criterion;
use biors::alphabet::UnambiguousDnaAlphabet;
use biors::sequence::Sequence;

fn push(seq: &mut Sequence<UnambiguousDnaAlphabet>, chars: &str) {
    seq.push(chars).unwrap();
    seq.clear();
}

fn push_unchecked(seq: &mut Sequence<UnambiguousDnaAlphabet>, chars: &str) {
    seq.push_unchecked(chars).unwrap();
    seq.clear();
}

fn sequence_benchmark(c: &mut Criterion) {
    // Setup the sequence and the characters to push
    // We take 500000 symbols to test push with
    let char_vec: Vec<char> = "ATGCGTCGA".chars().cycle().take(500000).collect();
    let chars: String = char_vec.iter().collect();

    // Test the normal push method with 3.2 billion symbols
    c.bench_function(
        "push",
        move |b| {
            let a = UnambiguousDnaAlphabet;
            let mut seq = Sequence::new(&a);

            b.iter(|| {
                push(&mut seq, &chars)
            })

        }
    );

    // Reconstruct the chars
    let char_vec: Vec<char> = "ATGCGTCGA".chars().cycle().take(500000).collect();
    let chars: String = char_vec.iter().collect();

    // Bench the size unchecked push method
    c.bench_function(
        "push_uncheckd",
        move |b| {
            let a = UnambiguousDnaAlphabet;
            let mut seq = Sequence::new(&a);

            b.iter(|| {
                push_unchecked(&mut seq, &chars)
            })
        }
    );
}

criterion_group!(sequence_benches, sequence_benchmark);
criterion_main!(sequence_benches);