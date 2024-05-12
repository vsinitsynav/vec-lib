use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use vec_lib::vectori128::Vec16c;

const LEN: usize = 1000000;

mod vcl {
    #[cxx::bridge(namespace = "bench")]
    mod vcl_mul {
        unsafe extern "C++" {
            include!("vec-lib/benches/vcl_benchmark.hpp");
            unsafe fn MulVec8iVCL(vec1: *const i8, vec2: *const i8, result: *mut i8, sz: usize);
        }
    }

    pub fn vcl_multiplication(a: &[i8], b: &[i8], c: &mut [i8], sz: usize) {
        assert_eq!(a.len(), b.len());
        assert!(sz <= a.len());

        // SAFETY: obvious
        unsafe {
            vcl_mul::MulVec8iVCL(
                a as *const [i8] as *const i8,
                b as *const [i8] as *const i8,
                c.as_mut_ptr(),
                sz,
            );
        }
    }
}


fn stupid_multiplication(a: &[i8], b: &[i8], c: &mut [i8], sz: usize) {
    assert_eq!(a.len(), b.len());
    assert!(sz <= a.len());
    assert!(sz <= c.len());

    for i in 0..sz {
        c[i] = black_box(a[i] * b[i]);
    }
}

fn vec_lib_multiplication(a: &[i8], b: &[i8], c: &mut [i8], sz: usize) {
    assert_eq!(a.len(), b.len());
    assert!(sz <= a.len());
    assert!(sz <= c.len());

    let mut a16 = Vec16c::new();
    let mut b16 = Vec16c::new();
    let mut c16;
    let  address = c.as_mut_ptr();
    let  tmpa = a.as_ptr();
    let  tmpb = b.as_ptr();

    for i in (0..sz).step_by(16) {
        // SAFETY: at least 16 bytes of memory are allocated
        let j = i.try_into().unwrap();
        unsafe {
            a16.load(tmpa.add(j));
            b16.load(tmpb.add(j));
        }
        c16 = a16 * b16;
        // SAFETY: pointer is located where at least 16 bytes of memory are located
        unsafe {
            c16.store(address.add(j));
        }
    }

    if (sz & 15) > 0 {
        // SAFETY: at least sz - i bytes of memory are allocated
        let i = (sz >> 4) << 4;
        unsafe {
            a16.load_partial(sz & 15, &a[i..sz]);
            b16.load_partial(sz & 15, &b[i..sz]);
        }
        c16 = a16 * b16;
        // SAFETY: pointer is located where at least sz - i bytes of memory are located
        unsafe {
            c16.store_partial(sz - i, address.add(i));
        }
    }
}

fn multiplication_benchmark(criteria: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let lens: [usize; 7] = [1000, 5000, 10000, 50000, 100000, 500000, 1000000];

    let a: [i8; LEN] = core::array::from_fn(|_| rng.gen_range(0..11));
    let b: [i8; LEN] = core::array::from_fn(|_| rng.gen_range(0..11));
    let mut c: [i8; LEN] = [0; LEN];

    for i in 0..7 {
        let name1 = format!("VCL C++ multiplication {}", lens[i]);
        let name2 = format!("vec-lib Rust multiplication {}", lens[i]);
        let name3 = format!("stupid Rust multiplication {}", lens[i]);

        criteria.bench_function(name1.as_str(), |criteria| {
            criteria.iter(|| {
                black_box(vcl::vcl_multiplication(
                    black_box(&a),
                    black_box(&b),
                    black_box(&mut c),
                    lens[i]
                ))
            })
        });

        criteria.bench_function(name2.as_str(), |criteria| {
            criteria.iter(|| {
                black_box(vec_lib_multiplication(
                    black_box(&a),
                    black_box(&b),
                    black_box(&mut c),
                    lens[i]
                ))
            })
        });

        criteria.bench_function(name3.as_str(), |criteria| {
            criteria.iter(|| {
                black_box(stupid_multiplication(
                    black_box(&a),
                    black_box(&b),
                    black_box(&mut c),
                    lens[i]
                ))
            })
        });
    }
}

criterion_group!(benches, multiplication_benchmark);
criterion_main!(benches);
