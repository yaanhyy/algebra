use ark_algebra_test_templates::*;

use crate::{Bn254, G1Projective, G2Projective};

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<Bn254>; msm);
test_pairing!(pairing; crate::Bn254);
test_group!(g1_glv; G1Projective; glv);
test_group!(g2_glv; G2Projective; glv);

mod pairing_test {
    pub const ITERATIONS: usize = 100;
    use ark_ec::{pairing::*, CurveGroup, PrimeGroup};
    use ark_ff::{BigInt, CyclotomicMultSubgroup, PrimeField};
    use ark_std::{test_rng, One, UniformRand};
    use ark_ec::short_weierstrass::SWCurveConfig;
    use crate::g1::{G1Affine, G1_GENERATOR_X, G1_GENERATOR_Y};
    use crate::{Config, Fq, g1, G1Projective};
    use crate::g2::{G2_GENERATOR_X, G2_GENERATOR_Y, G2Affine};

    #[test]
    fn test_bilinearity() {
        for _ in 0..1 {
            let mut rng = test_rng();
            // let a: <crate::Bn254 as Pairing>::G1 = UniformRand::rand(&mut rng);
            // let b: <crate::Bn254 as Pairing>::G2 = UniformRand::rand(&mut rng);
            // let s: <crate::Bn254 as Pairing>::ScalarField = UniformRand::rand(&mut rng);
            let a: <crate::Bn254 as Pairing>::G1 =  g1::Config::GENERATOR.into();//G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y).into();
            let b: <crate::Bn254 as Pairing>::G2 = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y).into();
            let s: <crate::Bn254 as Pairing>::ScalarField = BigInt::new([5,0,0,0]).into();
            let msg: <crate::Bn254 as Pairing>::ScalarField = BigInt::new([135,0,0,0]).into();

            println!("s:{:?}", s.to_string());
            println!("msg:{:?}", msg.to_string());

            let msb = b * s*msg;
            let sa = a*s;
            let mb = b*msg ;

            let ans1 = <crate::Bn254>::pairing(a, msb);
            println!("res:{:?}", ans1.to_string());
            let _ans2 = <crate::Bn254>::pairing(sa, mb);
            println!("res1:{:?}", ans1.to_string());
            // let _ans3 = <crate::Bn254>::pairing(a, b) * s;

            // match (&ans1, &ans2) {
            //     (left_val, right_val) => {
            //         if !(*left_val == *right_val) {
            //             let kind = panicking::AssertKind::Eq;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val, None);
            //         }
            //     }
            // };
            // match (&ans2, &ans3) {
            //     (left_val, right_val) => {
            //         if !(*left_val == *right_val) {
            //             let kind =panicking::AssertKind::Eq;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val, None);
            //         }
            //     }
            // };
            //
            // match (&ans1, &(PairingOutput::zero())) {
            //     (left_val, right_val) => {
            //         if *left_val == *right_val {
            //             let kind =panicking::AssertKind::Ne;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val,None);
            //         }
            //     }
            // };
            // match (&ans2, &(PairingOutput::zero())) {
            //     (left_val, right_val) => {
            //         if *left_val == *right_val {
            //             let kind =panicking::AssertKind::Ne;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val,None);
            //         }
            //     }
            // };
            // match (&ans3, &(PairingOutput::zero())) {
            //     (left_val, right_val) => {
            //         if *left_val == *right_val {
            //             let kind =panicking::AssertKind::Ne;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val,None);
            //         }
            //     }
            // };
            // let group_order = <<crate::Bn254 as Pairing>::ScalarField>::characteristic();
            //
            // match (&(ans1.mul_bigint(group_order)), &(PairingOutput::zero())) {
            //     (left_val, right_val) => {
            //         if !(*left_val == *right_val) {
            //             let kind =panicking::AssertKind::Eq;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val,None);
            //         }
            //     }
            // };
            // match (&(ans2.mul_bigint(group_order)), &(PairingOutput::zero())) {
            //     (left_val, right_val) => {
            //         if !(*left_val == *right_val) {
            //             let kind =panicking::AssertKind::Eq;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val,None);
            //         }
            //     }
            // };
            // match (&(ans3.mul_bigint(group_order)), &(PairingOutput::zero())) {
            //     (left_val, right_val) => {
            //         if !(*left_val == *right_val) {
            //             let kind =panicking::AssertKind::Eq;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val,None);
            //         }
            //     }
            // };
        }
    }

    #[test]
    fn test_multi_pairing() {
        for _ in 0..ITERATIONS {
            let rng = &mut test_rng();

            let a = <crate::Bn254 as Pairing>::G1::rand(rng).into_affine();
            let b = <crate::Bn254 as Pairing>::G2::rand(rng).into_affine();
            let c = <crate::Bn254 as Pairing>::G1::rand(rng).into_affine();
            let d = <crate::Bn254 as Pairing>::G2::rand(rng).into_affine();
            let ans1 = <crate::Bn254>::pairing(a, b) + &<crate::Bn254>::pairing(c, d);
            let ans2 = <crate::Bn254>::multi_pairing(&[a, c], &[b, d]);
            // match (&ans1, &ans2) {
            //     (left_val, right_val) => {
            //         if !(*left_val == *right_val) {
            //             let kind =panicking::AssertKind::Eq;
            //
            //
            //            panicking::assert_failed(kind, &*left_val, &*right_val,None);
            //         }
            //     }
            // };
        }
    }

    #[test]
    fn test_final_exp() {
        for _ in 0..ITERATIONS {
            let rng = &mut test_rng();
            let fp_ext = <crate::Bn254 as Pairing>::TargetField::rand(rng);
            let gt = <crate::Bn254 as Pairing>::final_exponentiation(MillerLoopOutput(fp_ext))
                .unwrap()
                .0;
            let r = <crate::Bn254 as Pairing>::ScalarField::MODULUS;
            assert!(gt.cyclotomic_exp(r).is_one());
        }
    }
}