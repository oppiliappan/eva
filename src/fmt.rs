use num::{BigInt, FromPrimitive, ToPrimitive};

fn thousand_sep(mut s: String) -> String {
    let inc = 3;
    let mut end = s.find('.').unwrap();
    let sign = if s.starts_with('-') { 1 } else { 0 };
    for _ in 0..(end - sign - 1) / inc {
        end -= inc;
        s.insert(end, ',');
    }
    s
}

pub fn pprint(base: u8, fix: usize, mut ans: f64) {
    if ans.is_infinite() {
        println!("{}inf", if ans.is_sign_positive() { "" } else { "-" });
    } else if ans.is_nan() {
        println!("nan");
    } else if base == 10 {
        // use standard library formatter since it handle printing pretty well
        let ans = format!("{:.*}", fix, ans);
        println!("{}", thousand_sep(ans));
    } else {
        ans = format!("{:.*}", fix, ans).parse().unwrap();
        let table: &[u8] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

        // format integral part of float
        let mut integral = BigInt::from_f64(ans.abs().trunc()).unwrap();
        let mut obase_int = String::new();
        let obaseb = BigInt::from_usize(base as usize).unwrap();

        while integral >= obaseb {
            obase_int.push(table[(&integral % &obaseb).to_usize().unwrap()] as char);
            integral /= &obaseb;
        }
        obase_int.push(table[integral.to_usize().unwrap()] as char);
        if ans.is_sign_negative() {
            obase_int.push('-');
        }
        let obase_int = obase_int.chars().rev().collect::<String>();

        // format fractional part of float
        // TODO: fix conversion with fractional part
        let mut fract = ans.abs().fract();
        let mut obase_fract = String::new();
        let mut i = 0;
        loop {
            fract *= base as f64;
            obase_fract.push(table[fract.trunc() as usize] as char);
            i += 1;
            if fract.fract() == 0. || i >= fix {
                break;
            }
            fract = fract.fract();
        }
        let ans = format!("{:>10}.{}", obase_int, obase_fract);
        println!("{}", thousand_sep(ans));
    }
}
