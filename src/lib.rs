extern crate web_sys;
extern crate num_bigint;
extern crate wasm_bindgen;
extern crate num_traits;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use num_bigint::{ToBigInt, BigInt, RandBigInt};
use rand::prelude::*;
use num_traits::{Zero};
use web_sys::console::log_1;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global window")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("no document")
}

fn log(s: &String) {
    log_1(&JsValue::from(s));
}

fn get_element_by_id<T: JsCast>(id: &str) -> T {
    document()
        .get_element_by_id(id)
        .expect("not found")
        .dyn_into::<T>()
        .map_err(|_| ())
        .unwrap()
}


fn switch_loading(loading: bool){
    let div_loading: web_sys::Element = get_element_by_id("loading");

    if loading {
        let arr = js_sys::Array::new();
        let s = JsValue::from_str("loaded");
        arr.set(0, s);
        div_loading.class_list().remove(&arr).expect("no className");
    } else {
        let arr = js_sys::Array::new();
        let s = JsValue::from_str("loaded");
        arr.set(0, s);
        div_loading.class_list().add(&arr).expect("no className");
    }

}

fn print_result(p: &BigInt, q: &BigInt, d: &BigInt, m: &BigInt, plain_text: &str) {
    let div_p: web_sys::HtmlDivElement = get_element_by_id("div_p");
    let div_q: web_sys::HtmlDivElement = get_element_by_id("div_q");
    let div_d: web_sys::HtmlDivElement = get_element_by_id("div_d");
    let div_m: web_sys::HtmlDivElement = get_element_by_id("div_m");
    let div_plain_text: web_sys::HtmlDivElement = get_element_by_id("div_plain_text");
    div_p.set_inner_text(&format!("p={}", p));
    div_q.set_inner_text(&format!("q={}", q));
    div_d.set_inner_text(&format!("d={}", d));
    div_m.set_inner_text(&format!("m={}", m));
    div_plain_text.set_inner_text(&format!("plainText={}", plain_text));
}

fn print_crypt(m: &BigInt, c: &BigInt) {
    let div_m: web_sys::HtmlDivElement = get_element_by_id("div_crypt_m");
    let div_c: web_sys::HtmlDivElement = get_element_by_id("div_crypt_c");
    div_m.set_inner_text(&format!("m={}", m));
    div_c.set_inner_text(&format!("c={}", c));
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn calc(e_str: &str, c_str: &str, n_str: &str, cheat: &str) -> Result<(), JsValue>{
    // loading_onはJSでやる
    //switch_loading(true);
    let e: BigInt = e_str.parse::<BigInt>().unwrap();
    let n: BigInt = n_str.parse::<BigInt>().unwrap();
    let c: BigInt = c_str.parse::<BigInt>().unwrap();

    log(&format!("c={}", &c));
    log(&format!("n={}", &n));
    log(&format!("e={}", &e));

    let first: BigInt;
    let p: BigInt;
    if cheat == "yes" {
        // answer: 32769132993266709549961988190834461413177642967992942539798288533
        first = "32769132993266709549961988190834461413177642967992942539798211111".parse::<BigInt>().unwrap();
        //first = 3.to_bigint().unwrap();
        let step = 2.to_bigint().unwrap();
        p = find_p(&first, &n,&step);
    } else {
        p = pollard_rho(&n);
        log(&format!("p={}", &p));
    }
    let q: BigInt = &n / &p;
    let lcm = (&p - 1) * (&q - 1);
    let d = calc_d_with_ext_gcd(&e, &lcm);
    log(&format!("d={}", &d));
    let m = c.modpow(&d, &n);
    let plain_text = replace_num_to_char(&format!("{}", &m));
    alert(&format!("よろしくお願いしまぁぁぁすっ!!"));
    print_result(&p, &q, &d, &m, &plain_text);
    switch_loading(false);
    return Ok(())
}

#[wasm_bindgen]
pub fn create_crypt_num(e_str: &str, n_str: &str, plain_text: &str) -> Result<(), JsValue>{
    let e: BigInt = e_str.parse::<BigInt>().unwrap();
    let n: BigInt = n_str.parse::<BigInt>().unwrap();
    let m = replace_char_to_num(plain_text).parse::<BigInt>().unwrap();
    let c = m.modpow(&e, &n);
    switch_loading(false);
    print_crypt(&m, &c);
    return Ok(())
}

fn gcd(a: &BigInt, b: &BigInt) -> BigInt{
    //log(&format!("with gcd:  a={} b={}", a, b));
    if b == &(0.to_bigint().unwrap()) {
        return a + 0.to_bigint().unwrap();
    } else {
        gcd(&b, &(a%b))
    }
}

fn ext_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt){
    log(&format!("with ext_gcd:  a={} b={}", a, b));
    if b != &(0.to_bigint().unwrap()) {
        let (d, y, x) = ext_gcd(&b, &( a % b));
        let y2 = &y - ((a / b) * &x);
        return (&d + 0.to_bigint().unwrap(), &x + 0.to_bigint().unwrap(), &y2 + 0.to_bigint().unwrap())
    }
    return (a + 0.to_bigint().unwrap(), 1.to_bigint().unwrap(), 0.to_bigint().unwrap())
}

fn calc_d_with_ext_gcd(e: &BigInt, lcm: &BigInt)-> BigInt {
    let (_, x, _) = ext_gcd(e, lcm);
    return &x + lcm
}

fn replace_num_to_char(number_str: &str) -> String{
    // Aのユニコードは65: i32なので文字列に置換
    let char_count = number_str.chars().count();
    let mut result = String::from("");
    for i in (0..char_count).step_by(2) {
        let num1 = number_str.chars().nth(i).unwrap();
        let num2 = number_str.chars().nth(i + 1).unwrap();
        let char_num_string: &str = &(num1.to_string() + &num2.to_string());
        let char_num: u32 = char_num_string.parse::<u32>().unwrap() + 64;
        let ch = std::char::from_u32(char_num);
        if let Some(v) = ch {
            if v.to_string() == "@" {
                result = result.clone().to_string() + " ";
            } else {
                let ch_string: String = v.to_string();
                result = result.clone().to_string() + &ch_string;
            }
        }
    }
    return result.to_string()
}

fn replace_char_to_num(char_str: &str) -> String{
    let char_count = char_str.chars().count();
    let mut result = String::from("");
    for i in 0..char_count {
        let ch = char_str.chars().nth(i).unwrap();
        if ch.to_string() == " " {
            result = result.clone().to_string() + "00";
        } else {
            let char_num: i32 = ch as i32 - 64;
            if char_num < 10 {
                result = result.clone().to_string() + "0" + &char_num.to_string();
            } else {
                result = result.clone().to_string() + &char_num.to_string();
            }
        }
    }
    return result.to_string()
}

fn find_p(first: &BigInt, n: &BigInt, step: &BigInt) -> BigInt{
    for p in num_iter::range_step(first + 0.to_bigint().unwrap(), n + 0.to_bigint().unwrap(), step + 0.to_bigint().unwrap()) {
        log(&format!("try to find p...= {}", &p));
        if n % &p == Zero::zero() {
            log(&format!("find p!! p={}", &p));
            return p
        }
    }
    switch_loading(false);
    alert(&format!("計算失敗！"));
    return  0.to_bigint().unwrap();
}

fn rho_f(x: &BigInt, a: &BigInt, n: &BigInt) -> BigInt{
    log(&format!("rho f:  x={} a={} n={}", x, a, n));
    return (x * x + a) % n
}

fn generate_bigint_rand(high: &BigInt) -> BigInt {
    let mut rng = thread_rng();
    let low = 0.to_bigint().unwrap();
    let b = rng.gen_bigint_range(&low, high);
    return b
}

fn pollard_rho(n: &BigInt) -> BigInt{
    loop {
        let x = generate_bigint_rand(n);
        let y = generate_bigint_rand(n);
        let a =  1.to_bigint().unwrap() +  generate_bigint_rand(&(n - 3.to_bigint().unwrap()));
        let x2 = rho_f(&x, &a, n);
        let y2 = rho_f(&y, &a, n);
        let y3 = rho_f(&y2, &a, n);
        let d = gcd(&(x2 - y3), n);
        if d > 1.to_bigint().unwrap() {
            return d
        }
    }
}
