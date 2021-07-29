extern crate web_sys;
extern crate num_bigint;
extern crate wasm_bindgen;
extern crate num_traits;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use num_bigint::{ToBigInt, BigInt};
use num_traits::{Zero, One};
use web_sys::console::log_1;
use js_sys::Array;

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
pub fn calc(e_str: &str, c_str: &str, n_str: &str, cheat: &str) -> Result<(), JsValue>{
    // loading_onはJSでやる
    //switch_loading(true);
    let e: BigInt = e_str.parse::<BigInt>().unwrap();
    let n: BigInt = n_str.parse::<BigInt>().unwrap();
    let c: BigInt = c_str.parse::<BigInt>().unwrap();

    log(&format!("c={}", &c));
    log(&format!("n={}", &n));
    log(&format!("e={}", &e));

    let mut first: BigInt;
    if cheat == "yes" {
        first = "32769132993266709549961988190834461413177642967992942539798288533".parse::<BigInt>().unwrap();
    } else {
        first = 3.to_bigint().unwrap();
    }
    let step = 2.to_bigint().unwrap();
    for p in num_iter::range_step(first, &n + 0.to_bigint().unwrap(), step) {
        log(&format!("{}", &p));
        if &n % &p == Zero::zero(){
            let q: BigInt = &n / &p;
            let lcm = (&p - 1) * (&q - 1);
            let d = calc_d(&e, &lcm, cheat);
            let m = c.modpow(&d, &n);
            let plain_text = replace_char(&format!("{}", &m));
            switch_loading(false);
            alert(&format!("よろしくお願いしまぁぁぁすっ!!"));
            print_result(&p, &q, &d, &m, &plain_text);
            return Ok(())
        }
    }
    switch_loading(false);
    alert(&format!("計算失敗！"));
    return Ok(())
}

fn switch_loading(loading: bool){
    let div_loading: web_sys::Element = get_element_by_id("loading");

    if loading {
        let arr = js_sys::Array::new();
        let s = JsValue::from_str("loaded");
        arr.set(0, s);
        div_loading.class_list().remove(&arr);
    } else {
        let arr = js_sys::Array::new();
        let s = JsValue::from_str("loaded");
        arr.set(0, s);
        div_loading.class_list().add(&arr);
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
    div_plain_text.set_inner_text(plain_text);
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

fn calc_d(e: &BigInt, lcm: &BigInt, cheat: &str) -> BigInt{
    let mut d = 1.to_bigint().unwrap();
    let mut first: BigInt;
    if cheat == "yes" {
        first = "106698614368578024442868771328920154780709906633937862801226224496631063125911774470873340168597462306553968544513277109053606095".parse::<BigInt>().unwrap();
    } else {
        first = 1.to_bigint().unwrap();
    }
    for tmpd in num_iter::range_step(first, lcm + 0.to_bigint().unwrap(), 1.to_bigint().unwrap()) {
        log(&format!("tmpd={}", &tmpd));
        if e * &tmpd % lcm == 1.to_bigint().unwrap() {
            return &tmpd + 0.to_bigint().unwrap()
        }
    }
    return d
}

fn replace_char(number_str: &str) -> String{
    let char_count = number_str.chars().count();
    let mut result = String::from("");
    for i in (0..char_count).step_by(2) {
        let num1 = number_str.chars().nth(i).unwrap();
        let num2 = number_str.chars().nth((i + 1)).unwrap();
        let ch = (num1.to_string() + &num2.to_string()).replace("01", "A").replace("02", "B").replace("03", "C").replace("04", "D").replace("05", "E").replace("06", "F").replace("07", "G").replace("08", "H").replace("09", "I").replace("10", "J").replace("11", "K").replace("12", "L").replace("13", "M").replace("14", "N").replace("15", "O").replace("16", "P").replace("17", "Q").replace("18", "R").replace("19", "S").replace("20", "T").replace("21", "U").replace("22", "V").replace("23", "W").replace("24", "X").replace("25", "Y").replace("26", "Z").replace("00", " ");
        result = result.clone().to_string() + &ch;
    }
    return result.to_string()
}

fn create_crypt() {
    let e = "65537".parse::<BigInt>().unwrap();
    let p = "32769132993266709549961988190834461413177642967992942539798288533".parse::<BigInt>().unwrap();
    let q = "3490529510847650949147849619903898133417764638493387843990820577".parse::<BigInt>().unwrap();
    let n = p * q;
    log(&format!("nnnnnnnnnnn={}", &n));
    let m = "11610410132109971031059932119111114100115329711410132115113117101971091051151043211111511510510211497103101328411132107110111119321051153211611132107110111119321161049711632121111117321071101111193211011111610410511010332841049711632105115321161041013211611411710132109101971101051101033211110232107110111119108101100103101".parse::<BigInt>().unwrap();
    let c = m.modpow(&e, &n);
    log(&format!("cccccccccc={}", &c));
}