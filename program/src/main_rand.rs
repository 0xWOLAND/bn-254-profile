#![no_main]

sp1_zkvm::entrypoint!(main);

use substrate_bn::Fq;

fn print_fq(f: &Fq, s: &str) {
    println!("{}", s);
    let mut slice = [0u8; 32];
    f.to_big_endian(&mut slice).unwrap();
    print!("0x");
    for byte in slice {
        print!("{:02x}", byte);
    }
    println!("");
}

pub fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let x = Fq::random(&mut rng);
        let y = Fq::random(&mut rng);
        print_fq(&x, "x");
        print_fq(&y, "y");
        // Test Addition
        {
            let mut a = x.0 .0;
            let b = y.0 .0;

            let lhs = x + y;

            a.add(&b, &Fq::modulus());
            let rhs = Fq::from_u256(a).unwrap();

            assert_eq!(lhs, rhs);
            println!("Addition test passed!");
        }

        // Test Subtraction
        {
            let mut a = x.0 .0;
            let b = y.0 .0;

            let lhs = x - y;

            a.sub(&b, &Fq::modulus());
            let rhs = Fq::from_u256(a).unwrap();

            assert_eq!(lhs, rhs);
            println!("Subtraction test passed!");
        }

        // Test Multiplication
        {
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
