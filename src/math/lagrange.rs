
/// Calculates Lagrange polynomial in O(N^2) time.
/// For the given data set (x_0,y_0) ... (x_n,y_n),
/// returns the array of coefficients b_i where f(x)=b_n-1*x^n-1 ... + b_1*x + b_0
// Example: https://atcoder.jp/contests/abc137/submissions/32074247
pub fn lagrange_polynomial(vx: &Vec<i64>, vy: &Vec<i64>, modulus: usize) -> Vec<i64> {
    let n = vx.len();
    let md = modulus as i64;
    let vx = vx.iter().map(|x| (x%md+md)%md).collect::<Vec<_>>();
    let vy = vy.iter().map(|y| (y%md+md)%md).collect::<Vec<_>>();

    // (x - x_0) * (x - x_1) ... (x - x_n-1)
    // vprod[i] = coefficient of x^i
    let mut vprod = vec![0;n+1];
    vprod[0] = vx[0];
    vprod[1] = 1;
    for i in 1..n {
        let x = vx[i];
        let mut next = vec![0;n+1];
        for j in 0..i+1 {
            next[j] += md - (vprod[j]*x)%md;
            next[j] %= md;
            next[j+1] += vprod[j];
            next[j+1] %= md;
        }
        vprod = next;
    }

    // vconst[i] = y_i/prod((x_i - x_j)) where j=0~n-1  (j != i)
    let mut vconst = vec![0;n];
    for i in 0..n {
        let xi = vx[i];
        let mut co = 1;
        for j in 0..n {
            let xj = vx[j];
            if xi == xj { continue; }
            co *= (xi+md-xj)%md;
            co %= md;
        }
        co = vy[i]*inv(co,md);
        vconst[i] = co % md;
    }

    // Pre-calculation to save O(logN) time.
    let mut vinv = vec![0;n];
    for i in 0..n {
        let x = vx[i];
        if x == 0 { continue; }
        if x == md-1 {
            vinv[i] = 1;
            continue;
        }
        vinv[i] = inv(md-x, md);
    }

    // Coefficient of x^i
    let mut vcoef = vec![0;n];
    for i in 0..n {
        let x = vx[i];
        if x==0 {
            // a_n*x^n + a_n-1*x^n-1 ... a_1*x
            //  -> a_n*x^n-1 + a_n-1*x^n-2 ... a_1
            // vcoef[i] = a_i+1
            for j in 0..n {
                vcoef[j] += vprod[j+1]*vconst[i];
                vcoef[j] %= md;
            }
        } else {
            // a_n*x^n + a_n-1*x^n-1 ... a_1*x + a_0
            //  -> (b_n*x^n-1 + b_n-1*x^n-2 ... b_1)*(x - x_i)
            // vcoef[i] = b_i+1
            let mut co = 0;
            for j in 0..n {
                co = (vprod[j]+md-co)*vinv[i];
                co %= md;
                vcoef[j] += co*vconst[i];
                vcoef[j] %= md;
            }
        }
    }
    return vcoef;
}

fn pow(val:i64, mut power: i64, modulus:i64) -> i64 {
    let mut square = val;
    let mut ret = 1;
    while 0 < power {
        if (power & 1) == 1{
            ret *= square;
            ret %= modulus;
        }
        square *= square;
        square %= modulus;
        power >>= 1;
    }
    return ret;
}
fn inv(val: i64, modulus:i64) -> i64 {
    return pow(val, modulus - 2, modulus);
}
