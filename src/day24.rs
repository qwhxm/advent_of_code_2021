//! <https://adventofcode.com/2021/day/24>
//!
//! Apparently it wasn't necessary to write any code at all, if one applied enough deduction:
//! <https://github.com/dphilipson/advent-of-code-2021/blob/master/src/days/day24.rs>. I was happy
//! to apply just enough deduction that brute force could comfortably do the rest.

use itertools::iproduct;

pub fn solution_1() -> String {
    let mut z;

    for (w1, w2, w3, w4) in iproduct!((1..=9).rev(), (1..=9).rev(), (1..=9).rev(), (1..=9).rev()) {
        z = 0;
        z += w1 + 8;
        z *= 26;
        z += w2 + 13;
        z *= 26;
        z += w3 + 8;
        z *= 26;
        z += w4 + 10;

        let w5 = (z % 26) - 11;
        if !(1..=9).contains(&w5) {
            continue;
        }
        z /= 26;

        let w6 = (z % 26) - 13;
        if !(1..=9).contains(&w6) {
            continue;
        }
        z /= 26;

        let z_after_w6 = z;
        for (w7, w8) in iproduct!((1..=9).rev(), (1..=9).rev()) {
            z = z_after_w6;
            z *= 26;
            z += w7 + 13;
            z *= 26;
            z += w8 + 5;

            let w9 = (z % 26) - 2;
            if !(1..=9).contains(&w9) {
                continue;
            }
            z /= 26;

            let w10 = (z % 26) - 6;
            if !(1..=9).contains(&w10) {
                continue;
            }
            z /= 26;

            let z_after_w10 = z;
            for w11 in (1..=9).rev() {
                z = z_after_w10;
                z *= 26;
                z += w11 + 2;

                let w12 = (z % 26) + 0;
                if !(1..=9).contains(&w12) {
                    continue;
                }
                z /= 26;

                let w13 = (z % 26) - 15;
                if !(1..=9).contains(&w13) {
                    continue;
                }
                z /= 26;

                let w14 = (z % 26) - 4;
                if !(1..=9).contains(&w14) {
                    continue;
                }
                z /= 26;

                if z == 0 {
                    let model_number_digits =
                        [w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14];
                    let model_number: u128 = model_number_digits
                        .iter()
                        .map(|&d| char::from_digit(d as u32, 10).unwrap())
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    return model_number.to_string();
                }
            }
        }
    }
    panic!("No valid model number");
}

pub fn solution_2() -> String {
    let mut z;

    for (w1, w2, w3, w4) in iproduct!(1..=9, 1..=9, 1..=9, 1..=9) {
        z = 0;
        z += w1 + 8;
        z *= 26;
        z += w2 + 13;
        z *= 26;
        z += w3 + 8;
        z *= 26;
        z += w4 + 10;

        let w5 = (z % 26) - 11;
        if !(1..=9).contains(&w5) {
            continue;
        }
        z /= 26;

        let w6 = (z % 26) - 13;
        if !(1..=9).contains(&w6) {
            continue;
        }
        z /= 26;

        let z_after_w6 = z;
        for (w7, w8) in iproduct!(1..=9, 1..=9) {
            z = z_after_w6;
            z *= 26;
            z += w7 + 13;
            z *= 26;
            z += w8 + 5;

            let w9 = (z % 26) - 2;
            if !(1..=9).contains(&w9) {
                continue;
            }
            z /= 26;

            let w10 = (z % 26) - 6;
            if !(1..=9).contains(&w10) {
                continue;
            }
            z /= 26;

            let z_after_w10 = z;
            for w11 in 1..=9 {
                z = z_after_w10;
                z *= 26;
                z += w11 + 2;

                let w12 = (z % 26) + 0;
                if !(1..=9).contains(&w12) {
                    continue;
                }
                z /= 26;

                let w13 = (z % 26) - 15;
                if !(1..=9).contains(&w13) {
                    continue;
                }
                z /= 26;

                let w14 = (z % 26) - 4;
                if !(1..=9).contains(&w14) {
                    continue;
                }
                z /= 26;

                if z == 0 {
                    let model_number_digits =
                        [w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14];
                    let model_number: u128 = model_number_digits
                        .iter()
                        .map(|&d| char::from_digit(d as u32, 10).unwrap())
                        .collect::<String>()
                        .parse()
                        .unwrap();
                    return model_number.to_string();
                }
            }
        }
    }
    panic!("No valid model number");
}

// Notes from "decompiling" the input code:
// * First pass:
// inp w
// mul x 0     x = 0
// add x z     x = z
// mod x 26    x = z % 26
// div z 1     noop
// add x 13    x = (z % 26) + 13
// eql x w     x = if (z % 26) + 13 == w then 1 else 0
// eql x 0     x = !(if (z % 26) + 13 == w then 1 else 0)
// mul y 0     y = 0
// add y 25    y = 25
// mul y x     y = 25 * !(if (z % 26) + 13 == w then 1 else 0)
// add y 1     y = 25 * !(if (z % 26) + 13 == w then 1 else 0) + 1
// mul z y     z *= 25 * !(if (z % 26) + 13 == w then 1 else 0) + 1 // doesn't do anything, z is 0
// mul y 0     y = 0
// add y w     y = w
// add y 8     y = w + 8
// mul y x     y = (w + 8) * !(if (z % 26) + 13 == w then 1 else 0)
// add z y     z += (w + 8) * !(if (z % 26) + 13 == w then 1 else 0) // z is 0, or 9..=17
//
// inp w
// mul x 0
// add x z     x = z
// mod x 26    x = z % 26
// div z 1     noop
// add x 12    x = (z % 26) + 12
// eql x w     x = ...
// eql x 0     x = if (z % 26) + 12 != w then 1 else 0
// mul y 0
// add y 25    y = 25
// mul y x     y = ...
// add y 1     y = 25 * x + 1
// mul z y     z *= 25 * x + 1 OR: if x == 1 then z *= 26 // noop if z == 0, else z can get big
// mul y 0
// add y w
// add y 13    y = w +13
// mul y x     y = (w + 13) * x
// add z y     z += (w + 13) * x OR: if x == 1 then z+= w + 13
//
// (...)
//
// * Second pass:
// 1
// x = (z % 26) + 13 != w      // x can't be 0
// if x == 1 then z += w + 8
//
// 2
// x = (z % 26) + 12 != w      // x can't be 0
// if x == 1 then z *= 26
// if x == 1 then z += w + 13
//
// 3
// x = (z % 26) + 12 != w      // x can't be 0
// if x == 1 then z *= 26
// if x == 1 then z += w + 8
//
// 4
// x = (z % 26) + 10 != w      // x can't be 0
// if x == 1 then z *= 26
// if x == 1 then z += w + 10
//
// 5
// x = (z % 26) - 11 != w      // x can be 0
// z /= 26
// if x == 1 then z *= 26
// if x == 1 then z += w + 12
//
// 6
// x = (z % 26) - 13 != w      // x can be 0
// z /= 26
// if x == 1 then z *= 26
// if x == 1 then z += w + 1
//
// 7
// x = (z % 26) + 15 != w      // x can't be 0
// if x == 1 then z *= 26
// if x == 1 then z += w + 13
//
// 8
// x = (z % 26) + 10 != w      // x can't be 0
// if x == 1 then z *= 26
// if x == 1 then z += w + 5
//
// 9
// x = (z % 26) - 2 != w       // x can be 0
// z /= 26
// if x == 1 then z *= 26
// if x == 1 then z += w + 10
//
// 10
// x = (z % 26) - 6 != w       // x can be 0
// z /= 26
// if x == 1 then z *= 26
// if x == 1 then z += w + 3
//
// 11
// x = (z % 26) + 14 != w      // x can't be 0
// if x == 1 then z *= 26
// if x == 1 then z += w + 2
//
// 12
// x = (z % 26) + 0 != w       // x can be 0
// z /= 26
// if x == 1 then z *= 26
// if x == 1 then z += w + 2
//
// 13
// x = (z % 26) - 15 != w      // x can be 0
// z /= 26
// if x == 1 then z *= 26
// if x == 1 then z += w + 12
//
// 14
// x = (z % 26) - 4 != w       // x can be 0
// z /= 26
// if x == 1 then z *= 26
// if x == 1 then z += w + 7
//
// * Third pass:
// 1
// z += w + 8
//
// 2
// z *= 26
// z += w + 13
//
// 3
// z *= 26
// z += w + 8
//
// 4
// z *= 26
// z += w + 10
//
// 5
// x = (z % 26) - 11 != w      // need 0
// z /= 26
//
// 6
// x = (z % 26) - 13 != w      // need 0
// z /= 26
//
// 7
// z *= 26
// z += w + 13
//
// 8
// z *= 26
// z += w + 5
//
// 9
// x = (z % 26) - 2 != w       // need 0
// z /= 26
//
// 10
// x = (z % 26) - 6 != w       // need 0
// z /= 26
//
// 11
// z *= 26
// z += w + 2
//
// 12
// x = (z % 26) + 0 != w       // need 0
// z /= 26
//
// 13
// x = (z % 26) - 15 != w      // need 0
// z /= 26
//
// 14
// x = (z % 26) - 4 != w       // need 0
// z /= 26
