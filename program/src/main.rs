#![no_main]

use revm_precompile::{
    bn128::{
        add::BYZANTIUM_ADD_GAS_COST,
        mul::BYZANTIUM_MUL_GAS_COST,
        pair::{BYZANTIUM_PAIR_BASE, BYZANTIUM_PAIR_PER_POINT},
        run_add, run_mul, run_pair,
    },
    Error, PrecompileErrors,
};

sp1_zkvm::entrypoint!(main);

use substrate_bn::Fq;

// fn print_fq(f: &Fq, s: &str) {
//     println!("{}", s);
//     let mut slice = [0u8; 32];
//     f.to_big_endian(&mut slice).unwrap();
//     print!("0x");
//     for byte in slice {
//         print!("{:02x}", byte);
//     }
//     println!("");
// }

// pub fn main() {
//     let mut rng = rand::thread_rng();
//     for _ in 0..50 {
//         let x = Fq::random(&mut rng);
//         let y = Fq::random(&mut rng);
//         print_fq(&x, "x");
//         print_fq(&y, "y");
//         // Test Addition
//         {
//             let mut a = x.0 .0;
//             let b = y.0 .0;

//             let lhs = x + y;

//             a.add(&b, &Fq::modulus());
//             let rhs = Fq::from_u256(a).unwrap();

//             assert_eq!(lhs, rhs);
//             println!("Addition test passed!");
//         }

//         // Test Subtraction
//         {
//             let mut a = x.0 .0;
//             let b = y.0 .0;

//             let lhs = x - y;

//             a.sub(&b, &Fq::modulus());
//             let rhs = Fq::from_u256(a).unwrap();

//             assert_eq!(lhs, rhs);
//             println!("Subtraction test passed!");
//         }

//         // Test Multiplication
//         {
//             let mut a = x.0 .0;
//             let b = y.0 .0;

//             let lhs = x * y;

//             a.mul(&b, &Fq::modulus());
//             let rhs = Fq::from_u256(a).unwrap();

//             assert_eq!(lhs, rhs);
//             println!("Multiplication test passed!");
//         }
//     }
// }

fn main() {
    // let input = hex::decode(
    //     "\
    //      18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
    //      063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
    //      07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed\
    //      06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
    // )
    // .unwrap();
    // let expected = hex::decode(
    //     "\
    //     2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703\
    //     301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915",
    // )
    // .unwrap();

    // let outcome = run_add(&input, BYZANTIUM_ADD_GAS_COST, 500).unwrap();
    // assert_eq!(outcome.bytes, expected);

    // // zero sum test
    // let input = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();
    // let expected = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();

    // let outcome = run_add(&input, BYZANTIUM_ADD_GAS_COST, 500).unwrap();
    // assert_eq!(outcome.bytes, expected);

    // // out of gas test
    // let input = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();

    // let res = run_add(&input, BYZANTIUM_ADD_GAS_COST, 499);
    // println!("{:?}", res);
    // assert!(matches!(res, Err(PrecompileErrors::Error(Error::OutOfGas))));

    // // no input test
    // let input = [0u8; 0];
    // let expected = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();

    // let outcome = run_add(&input, BYZANTIUM_ADD_GAS_COST, 500).unwrap();
    // assert_eq!(outcome.bytes, expected);

    // // point not on curve fail
    // let input = hex::decode(
    //     "\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111",
    // )
    // .unwrap();

    // let res = run_add(&input, BYZANTIUM_ADD_GAS_COST, 500);
    // assert!(matches!(
    //     res,
    //     Err(PrecompileErrors::Error(Error::Bn128AffineGFailedToCreate))
    // ));

    // let input = hex::decode(
    //     "\
    //     2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb7\
    //     21611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204\
    //     00000000000000000000000000000000000000000000000011138ce750fa15c2",
    // )
    // .unwrap();
    // let expected = hex::decode(
    //     "\
    //     070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c\
    //     031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc",
    // )
    // .unwrap();

    // let outcome = run_mul(&input, BYZANTIUM_MUL_GAS_COST, 40_000).unwrap();
    // assert_eq!(outcome.bytes, expected);

    // // out of gas test
    // let input = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0200000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();

    // let res = run_mul(&input, BYZANTIUM_MUL_GAS_COST, 39_999);
    // assert!(matches!(res, Err(PrecompileErrors::Error(Error::OutOfGas))));

    // // zero multiplication test
    // let input = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0200000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();
    // let expected = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();

    // let outcome = run_mul(&input, BYZANTIUM_MUL_GAS_COST, 40_000).unwrap();
    // assert_eq!(outcome.bytes, expected);

    // // no input test
    // let input = [0u8; 0];
    // let expected = hex::decode(
    //     "\
    //     0000000000000000000000000000000000000000000000000000000000000000\
    //     0000000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();

    // let outcome = run_mul(&input, BYZANTIUM_MUL_GAS_COST, 40_000).unwrap();
    // assert_eq!(outcome.bytes, expected);

    // // point not on curve fail
    // let input = hex::decode(
    //     "\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     0f00000000000000000000000000000000000000000000000000000000000000",
    // )
    // .unwrap();

    // let res = run_mul(&input, BYZANTIUM_MUL_GAS_COST, 40_000);
    // assert!(matches!(
    //     res,
    //     Err(PrecompileErrors::Error(Error::Bn128AffineGFailedToCreate))
    // ));

    let input = hex::decode("10206b16d596b38f8d2b95a69ab2b7da0101685a605a3d2bc96049742a2517e109ec26d6a7dfcdec0b2fdaea654288de89cdb2b715c32aab2371fae5c5f8c4e50beb9c7ba6b68ce052963eeab8cfba6e6d4981878f2910c7317d3180be72780d2c6375e80c128478dedeb6630de88ab0959d011cbde2e66662d9b7467a6b678f26b84de206e897cff58017764babc2fdbc303c29c103fc1b5e4adb281d9abccc048f4cfeb02aa9f2dfa9feed998f5542e5aae986f2fd86ea97dc90a1586f47ac2d4d9aa7e302d9df41749d5507949d05dbea33fbb16c643b22f599a2be6df2e214bedd503c37ceb061d8ec60209fe345ce89830a19230301f076caff004d19260967032fcbf776d1afc985f88877f182d38480a653f2decaa9794cbc3bf3060c0e187847ad4c798374d0d6732bf501847dd68bc0e071241e0213bc7fc13db7ab304cfbd1e08a704a99f5e847d93f8c3caafddec46b7a0d379da69a4d112346a71739c1b1a457a8c7313123d24d2f9192f896b7c63eea05a9d57f06547ad0cec8185740445a020829d8a6bf543c28acd1a4fcf2d542ca437ddbd03050a44988df267bc64ccf0ee0591b2e7119a209738c6d663dc90057367680304264a0a769f9198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa192ccebd42112ba8e54855c15df4d3e45eeb2ad007a28bb4a6f39c2e27c215422ef0742cf643169f39df3d09044a3f0ecf5c9c6abdf3c09969b66a911fe41a80168e4fddac50a40d5bcff39c7fa9207cd368444c0c01a86690a6645b52f3aa1f2139a256456825daa623957c4f2ea1a0d26f135769e450759142a7159b0a447628deba4ed0a3b79dbc6a7dac67c07051f421904de49dcd7ae91aaf1223bc6d631c3976c9a490dad50e601586279bb60f12416a8cc141710167fcdc0dc931bffd").unwrap();
    let expected =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000001").unwrap();

    println!("cycle-tracker-start: run-pair");
    let outcome = run_pair(
        &input,
        BYZANTIUM_PAIR_PER_POINT,
        BYZANTIUM_PAIR_BASE,
        260_0000000,
    )
    .unwrap();
    println!("cycle-tracker-end: run-pair");
    assert_eq!(outcome.bytes, expected);

    let input = hex::decode(
        "\
        1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
        3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
        209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf7\
        04bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a41678\
        2bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d\
        120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550\
        111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c\
        2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411\
        198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
        1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
        090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
        12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
    )
    .unwrap();
    let expected =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000001").unwrap();

    let outcome = run_pair(
        &input,
        BYZANTIUM_PAIR_PER_POINT,
        BYZANTIUM_PAIR_BASE,
        260_000,
    )
    .unwrap();
    assert_eq!(outcome.bytes, expected);

    // // out of gas test
    // let input = hex::decode(
    //     "\
    //     1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
    //     3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
    //     209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf7\
    //     04bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a41678\
    //     2bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d\
    //     120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550\
    //     111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c\
    //     2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411\
    //     198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
    //     1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
    //     090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
    //     12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
    // )
    // .unwrap();

    // let res = run_pair(
    //     &input,
    //     BYZANTIUM_PAIR_PER_POINT,
    //     BYZANTIUM_PAIR_BASE,
    //     259_999,
    // );
    // assert!(matches!(res, Err(PrecompileErrors::Error(Error::OutOfGas))));

    // // no input test
    // let input = [0u8; 0];
    // let expected =
    //     hex::decode("0000000000000000000000000000000000000000000000000000000000000001").unwrap();

    // let outcome = run_pair(
    //     &input,
    //     BYZANTIUM_PAIR_PER_POINT,
    //     BYZANTIUM_PAIR_BASE,
    //     260_000,
    // )
    // .unwrap();
    // assert_eq!(outcome.bytes, expected);

    // // point not on curve fail
    // let input = hex::decode(
    //     "\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111",
    // )
    // .unwrap();

    // let res = run_pair(
    //     &input,
    //     BYZANTIUM_PAIR_PER_POINT,
    //     BYZANTIUM_PAIR_BASE,
    //     260_000,
    // );
    // assert!(matches!(
    //     res,
    //     Err(PrecompileErrors::Error(Error::Bn128AffineGFailedToCreate))
    // ));

    // // invalid input length
    // let input = hex::decode(
    //     "\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     1111111111111111111111111111111111111111111111111111111111111111\
    //     111111111111111111111111111111\
    // ",
    // )
    // .unwrap();

    // let res = run_pair(
    //     &input,
    //     BYZANTIUM_PAIR_PER_POINT,
    //     BYZANTIUM_PAIR_BASE,
    //     260_000,
    // );
    // assert!(matches!(
    //     res,
    //     Err(PrecompileErrors::Error(Error::Bn128PairLength))
    // ));
}
