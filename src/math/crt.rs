use std::mem::swap;

/// Solves the modular equation system with the Chinese remainder theorem.
/// Original theorem has restriction on the moduli being coprime,
/// but this method solves even if the moduli are not coprime as long as
/// the following restriction is met, otherwise None is returned.
///   For each pair of the congruences,
///     x≡a (mod m)
///     x≡b (mod n)
///   a-b can be divided by gcd(m,n).
///     i.e. a≡b (mod gcd(m,n))
///
/// # panic
///  - The size of remainders and moduli is not the same.
///  - There is a modulo that is less than 1.
///
// reference: https://math.stackexchange.com/questions/1644677/what-to-do-if-the-modulus-is-not-coprime-in-the-chinese-remainder-theorem
pub fn crt(rm: &[i64], md: &[i64]) -> Option<(i64,i64)> {
    if rm.len() != md.len() {
        panic!("The size of remainders and moduli is not same.");
    }
    let mut r1 = 0;
    let mut m1 = 1;
    for (&(mut r2), &(mut m2)) in rm.iter().zip(md) {
        r2 = (r2+m2)%m2;
        if m2 < 1 {
            panic!("Modulus should be greater than 0, but input was {}", m2);
        }
        if m1 < m2 {
            swap(&mut r1,&mut r2);
            swap(&mut m1,&mut m2);
        }
        if m1%m2 == 0 {
            if r1%m2 != r2 {
                return None;
            }
            continue;
        }
        let (g,u,_v) = extended_gcd(m1,m2);
        if (r1-r2)%g != 0{
            return None;
        }
        let w = (r1-r2)/g;
        let m12 = m1/g*m2;
        let x = r1-((((m1*u)%m12)*w)%m12);
        r1 = (x+m12)%m12;
        m1 = m12;
    }
    return Some((r1,m1));
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
        assert_eq!(Some((78,280)),x);

        let r = vec![1,2,5,5];
        let m = vec![2,3,6,12];
        let x = crt(&r,&m);
        assert_eq!(Some((5,12)),x);

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
