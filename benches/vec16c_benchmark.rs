use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use vec_lib::vectori128::Vec16c;

const LEN: usize = 1000000;

#[inline(never)]
fn stupid(a: &[i8], b: &[i8], sz: usize) -> Vec<i8> {
    let mut c = vec![0; sz];

    for i in 0..sz {
        c[i] = a[i] * b[i];
    }
    c
}

#[inline(never)]
fn smart(a: &[i8], b: &[i8], sz: usize) -> Vec<i8> {
    let mut c = vec![0; sz];
    let mut address = c.as_mut_ptr();

    let mut i = 0;

    loop {
        let mut a16 = Vec16c::new();
        let mut b16 = Vec16c::new();

        if i + 16 >= sz {
            // SAFETY: at least sz - i bytes of memory are allocated
            unsafe {
                a16.load_partial(sz - i, &a[i..sz]);
                b16.load_partial(sz - i, &b[i..sz]);
            }
            let mut c16 = a16 * b16;
            // SAFETY: pointer is located where at least sz - i bytes of memory are located
            unsafe {
                c16.store_partial(sz - i, address);
            }
        } else {
            // SAFETY: at least 16 bytes of memory are allocated
            unsafe {
                a16.load(&a[i] as *const i8);
                b16.load(&a[i] as *const i8);
            }
            let c16 = a16 * b16;
            // SAFETY: pointer is located where at least 16 bytes of memory are located
            unsafe {
                c16.store(address);
            }
        }
        i += 16;
        if i >= sz {
            break;
        }
        unsafe {
            address = address.add(16);
        }
    }
    c
}

fn multiplication_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let lens: [usize; 5] = [100, 1000, 10000, 100000, 1000000];

    let a: [i8; LEN] = core::array::from_fn(|_| rng.gen_range(0..11));
    let b: [i8; LEN] = core::array::from_fn(|_| rng.gen_range(0..11));

    for i in 0..5 {
        let j = i + 2;
        let name1 = format!("stupid multiplication 1e{j}");
        let name2 = format!("vec-lib multiplication 1e{j}");

        c.bench_function(name1.as_str(), |c| {
            c.iter(|| black_box(stupid(black_box(&a), black_box(&b), lens[i])))
        });

        c.bench_function(name2.as_str(), |c| {
            c.iter(|| black_box(smart(black_box(&a), black_box(&b), lens[i])))
        });
    }
}

criterion_group!(benches, multiplication_benchmark);
criterion_main!(benches);
