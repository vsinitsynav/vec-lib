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

fn vec_lib_multiplication(a: &[i8], b: &[i8], c: &mut [i8], sz: usize) {
    assert_eq!(a.len(), b.len());
    assert!(sz <= a.len());
    assert!(sz <= c.len());

    let mut i = 0;

    let mut a16 = Vec16c::new();
    let mut b16 = Vec16c::new();
    let mut address = c.as_mut_ptr();

    while i + 16 < sz {
        // SAFETY: at least 16 bytes of memory are allocated
        unsafe {
            a16.load(&a[i] as *const i8);
            b16.load(&b[i] as *const i8);
        }
        let c16 = a16 * b16;
        // SAFETY: pointer is located where at least 16 bytes of memory are located
        unsafe {
            c16.store(address);
        }
        i += 16;
        unsafe {
            address = address.add(16);
        }
    }

    if i < sz {
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
    }
}

fn multiplication_benchmark(criteria: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let lens: [usize; 5] = [100, 1000, 10000, 100000, 1000000];

    let a: [i8; LEN] = core::array::from_fn(|_| rng.gen_range(0..11));
    let b: [i8; LEN] = core::array::from_fn(|_| rng.gen_range(0..11));
    let mut c: [i8; LEN] = [0; LEN];

    for i in 0..5 {
        let j = i + 2;
        let name1 = format!("VCL multiplication 1e{j}");
        let name2 = format!("vec-lib multiplication 1e{j}");

        criteria.bench_function(name1.as_str(), |criteria| {
            criteria.iter(|| {
                black_box(vcl::vcl_multiplication(
                    black_box(&a),
                    black_box(&b),
                    black_box(&mut c),
                    lens[i],
                ))
            })
        });

        criteria.bench_function(name2.as_str(), |criteria| {
            criteria.iter(|| {
                black_box(vec_lib_multiplication(
                    black_box(&a),
                    black_box(&b),
                    black_box(&mut c),
                    lens[i],
                ))
            })
        });
    }
}

criterion_group!(benches, multiplication_benchmark);
criterion_main!(benches);
