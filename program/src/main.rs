#![no_main]

sp1_zkvm::entrypoint!(main);

use substrate_bn::Fq;

pub fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        // Test Addition
        {
            let x = Fq::random(&mut rng);
            let y = Fq::random(&mut rng);

            let mut a = x.0 .0;
            let b = y.0 .0;

            let lhs = x + y;

            a.add(&b, &Fq::modulus());
            let rhs = Fq::from_u256(a).unwrap();

            assert_eq!(lhs, rhs);
            println!("Addition test passed!");
        }

        // Test Subtraction
        // {
        //     let x = Fq::random(&mut rng);
        //     let y = Fq::random(&mut rng);

        //     let mut a = x.0 .0;
        //     let b = y.0 .0;

        // let lhs = x - y;

        // a.sub(&b, &Fq::modulus());
        // let rhs = Fq::from_u256(a).unwrap();

        // assert_eq!(lhs, rhs);
        //     println!("Subtraction test passed!");
        // }

        // Test Multiplication
        {
            let x = Fq::random(&mut rng);
            let y = Fq::random(&mut rng);

            let mut a = x.0 .0;
            let b = y.0 .0;

            let lhs = x * y;

            a.mul(&b, &Fq::modulus());
            let rhs = Fq::from_u256(a).unwrap();

            assert_eq!(lhs, rhs);
            println!("Multiplication test passed!");
        }
    }
}
