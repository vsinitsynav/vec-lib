use criterion::{black_box, criterion_group, criterion_main, Criterion};

use vec_lib::vectori128::{Vec16c};

use rand::Rng; // 0.6.5

const LEN: usize = 1000000000;

fn stupid(a: &Vec<i8>, b: &Vec<i8>) {
    let sz = a.len();
    let mut c: [i8; LEN] = [0; LEN];
    for i in 0..sz {
        c[i] = black_box(a[i] * b[i]);
    }
}

fn smart(a: &Vec<i8>, b: &Vec<i8>) {
    let sz = a.len();
    let mut c: [i8; LEN] = [0; LEN];
    let mut i = 0;
    loop {
        if i >= sz {
            break;
        }
        black_box(
        unsafe {
            let mut a16 = Vec16c::new();
            let mut b16 = Vec16c::new();    
            a16.load( &a[i] as *const i8);
            b16.load( &b[i] as *const i8);
            if i + 16 >= sz {
                a16.cutoff((sz - i) as isize);
                b16.cutoff((sz - i) as isize);
            }
            let c16 = a16 * b16;
            c16.store(&mut c[i] as *mut i8);
        }
        );
        i += 16;
    }
}

fn multiplication_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let a: Vec<i8> = (0..1000000).map(|_| rng.gen_range(0..11)).collect();
    let b: Vec<i8> = (0..1000000).map(|_| rng.gen_range(0..11)).collect();

    c.bench_function("stupid multiplication 1e9", |c| c.iter(|| stupid(&a, &b)));

    c.bench_function("vec-lib multiplication 1e9", |c| c.iter(|| smart(&a, &b)));
}

// criterion_group!(benches, multiplication_benchmark);
criterion_group!(benches, multiplication_benchmark);
criterion_main!(benches);