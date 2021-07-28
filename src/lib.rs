extern crate web_sys;
extern crate num_bigint;
extern crate wasm_bindgen;
extern crate num_traits;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use num_bigint::{ToBigInt, BigInt};
use num_traits::{Zero, One};
use web_sys::console::log_1;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn log(s: &String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn calc(c_str: &str, n_str: &str) -> Result<(), JsValue>{
    let n: BigInt = n_str.parse::<BigInt>().unwrap();
    let c: BigInt = c_str.parse::<BigInt>().unwrap();
    let e: BigInt = 65537.to_bigint().unwrap();

    log(&format!("c={}", &c));
    log(&format!("n={}", &n));
    log(&format!("e={}", &e));

    let first = 3.to_bigint().unwrap();
    let step = 2.to_bigint().unwrap();
    for p in num_iter::range_step(first, &n + 0.to_bigint().unwrap(), step) {
        log(&format!("{}", &p));
        // if &p % 1 == Zero::zero() {
        //     log(&format!("{}", &p));
        // }
        if &n % &p == Zero::zero(){
            let q: BigInt = &n / &p;
            //let d = (&p - 1) * (&q - 1) + 1;
            let lcm = (&p - 1) * (&q - 1);

            // let g = gcd(&e, &lcm);
            // if g == 1.to_bigint().unwrap() {
            //     continue;
            // }
            // log(&format!("g={}", &g));
            //let d = (&lcm + 1) / &e;
            let d = calc_d(&e, &lcm);
            let m = c.modpow(&d, &n);
            print_result(&p, &q, &d, &m);
            alert(&format!("よろしくお願いしまぁぁぁすっ!!"));
            log(&format!("c={}", &m.modpow(&e, &n)));
            log(&format!("p*q={}", &p * &q));
            return Ok(())
            // if &lcm % &e == One::one() {
            //     let d = (&lcm + 1) / &e;
            //     let m = c.modpow(&d, &n) / e;
            //     print_result(&p, &q, &d, &m);
            //     alert(&format!("よろしくお願いしまぁぁぁすっ!!"));
            //     return Ok(())
            // }
        }
    }
    alert(&format!("計算失敗！"));
    return Ok(())
}

fn print_result(p: &BigInt, q: &BigInt, d: &BigInt, m: &BigInt) {
    let div_p: web_sys::HtmlDivElement = get_element_by_id("div_p");
    let div_q: web_sys::HtmlDivElement = get_element_by_id("div_q");
    let div_d: web_sys::HtmlDivElement = get_element_by_id("div_d");
    let div_m: web_sys::HtmlDivElement = get_element_by_id("div_m");
    div_p.set_inner_text(&format!("p=, {}", p));
    div_q.set_inner_text(&format!("q=, {}", q));
    div_d.set_inner_text(&format!("d=, {}", d));
    div_m.set_inner_text(&format!("m=, {}", m));
}

fn print_in_progress(p: &BigInt) {
    let div_p: web_sys::HtmlDivElement = get_element_by_id("div_p");
    div_p.set_inner_text(&format!("p=, {}", p));
}

fn get_element_by_id<T: JsCast>(id: &str) -> T {
    document()
        .get_element_by_id(id)
        .expect("not found")
        .dyn_into::<T>()
        .map_err(|_| ())
        .unwrap()
}

fn gcd(a: &BigInt, b: &BigInt) -> BigInt{
    log(&format!("a={} b={}", a, b));
    if b == &(0.to_bigint().unwrap()) { 
        return a + 0.to_bigint().unwrap();
    } else {
        gcd(&b, &(a%b))
    }
}

fn calc_d(e: &BigInt, lcm: &BigInt) -> BigInt{
    let mut d = 1.to_bigint().unwrap();
    for tmpd in num_iter::range_step(1.to_bigint().unwrap(), lcm + 0.to_bigint().unwrap(), 1.to_bigint().unwrap()) {
        log(&format!("tmpd={}", &tmpd));
        if e * &tmpd % lcm == 1.to_bigint().unwrap() {
            return &tmpd + 0.to_bigint().unwrap()
        }
    }
    return d
}

// fn ext_gcd(a: BigInt, b: BigInt) -> BigInt {
//     let mut r0 = &a;
//     let mut r1 = b;
//     let mut s0 = 1.to_bigint().unwrap();
//     let mut s1 = 0.to_bigint().unwrap();
//     let mut t0 = 0.to_bigint().unwrap();
//     let mut t1 = 1.to_bigint().unwrap();
//     while &r1 != &(0.to_bigint().unwrap()) {
//         let q = r0 / &r1;
//         let r = r0 - &q * &r1;
//         let s = &s0 - &q * &s1;
//         let t = &t0 - &q * &t1;
//         r0 = &r1;
//         s0 = &s1 + 0.to_bigint().unwrap();
//         t0 = &t1 + 0.to_bigint().unwrap();
//         r1 = &r + 0.to_bigint().unwrap();
//         s1 = &s + 0.to_bigint().unwrap();
//         t1 = &t + 0.to_bigint().unwrap();
//     }
//     if &t0 < &(0.to_bigint().unwrap()) {
//         return &t0 + &a + 0.to_bigint().unwrap();
//     } else {
//         return &t0 + 0.to_bigint().unwrap();
//     }
// }

fn extended_euclidean(u: i64, v: i64) -> i64 {
    let mut r0 = u;
    let mut r1 = v;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;
    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 - q * r1;
        let s = s0 - q * s1;
        let t = t0 - q * t1;
        r0 = r1;
        s0 = s1;
        t0 = t1;
        r1 = r;
        s1 = s;
        t1 = t;
    }
    println!("{} * {} + {} * {} = {}", s0, u, t0, v, r0);
    if t0 < 0 {
        t0 + u
    } else {
        t0
    }
}