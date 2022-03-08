
/// Solves the modular equation system with chinese remainder theorem.
pub fn crt(rm: &[i64], md: &[i64]) -> Option<i64> {
    let prod = md.iter().product::<i64>();
    if rm.len() != md.len() {
        panic!("The size of remainders and moduli is not same.");
    }
    let mut sum = 0;
    for (&r, &m) in rm.iter().zip(md) {
        if m < 1 {
            panic!("Modulus should be greater than 0, but input was {}", m);
        }
        let p = prod / m;
        let x = mod_inv(p,m)?;
        sum += r*x*p;
    }
    return Some(sum % prod);
}

fn mod_inv(a: i64, b: i64) -> Option<i64> {
    let (g, a, _) = extended_gcd(a, b);
    if g != 1 {
        return None;
    }
    return Some((a % b + b) % b);
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (d, coef_b, coef_a) = extended_gcd(b, a % b);
        (d, coef_a, coef_b - coef_a * (a / b))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_crt() {
        let r = vec![3,1,6];
        let m = vec![5,7,8];
        let x = crt(&r,&m);
        assert_eq!(Some(78),x);

        let r = vec![3,1];
        let m = vec![15,10];
        let x = crt(&r,&m);
        assert_eq!(None, x);
    }

    #[test]
    #[should_panic]
    fn test_crt_err() {
        let r = vec![3,1];
        let m = vec![0,10];
        crt(&r,&m);
    }
}
