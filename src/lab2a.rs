//Zadanie 1 Zadania numer 6 oraz 9 z Zestawu 1 zrób na dwa sposoby (każde) — z użyciem pętli while/loop oraz z użyciem pętli for.
//Zadanie 6
pub fn factorial(n:i32) -> i32{
    //let n = 4;
    let mut result = 1;
    for i in 1..=n{
        result *= i;
    }
    // println!("{}! = {}", n, result);
    result
}
pub fn factorial2(n:i32) -> i32{
    //let n = 4;
    let mut result = 1;
    let mut i = 1;
    while i <=n{
        result *= i;
        i += 1;
    }
    // println!("{}! = {}", n, result);
    result
}
pub fn factorial3(n:i32) -> i32{
    //let n = 4;
    let mut result = 1;
    let mut i = 1;
    loop{
        result *= i;
        i += 1;
        if i > n{
            break;
        }
    }
    // println!("{}! = {}", n, result);
    result
}
//Zadanie 9
pub fn find_pythagorean_triples(n: u32) {
    for a in 1..=n {
        for b in a + 1..=n {
            for c in b + 1..=n {
                if a * a + b * b == c * c {
                    println!("({a}, {b}, {c})");
                }
            }
        }
    }
}
pub fn find_pythagorean_triples2(n: u32) {
    let mut a = 1;

    while a <= n {
        let mut b = a + 1;

        while b <= n {
            let mut c = b + 1;

            while c <= n {
                if a * a + b * b == c * c {
                    println!("({}, {}, {})", a, b, c);
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
}
pub fn find_pythagorean_triples3(n: u32) {
    let mut a = 1;

    loop{
        let mut b = a + 1;

        loop{
            let mut c = b + 1;

            loop{
                if a * a + b * b == c * c {
                    println!("({}, {}, {})", a, b, c);
                }
                c += 1;
                if c > n{
                    break;
                }
            }
            b += 1;
            if b > n{
                break;
            }
        }
        a += 1;
        if a > n{
            break;
        }
    }
}
//Zadanie 2 Zrealizującą znajdowanie przybliżonego miejsca zerowego metodą Newtona — w czterech wersjach:
fn f(x: f64) -> f64{x*x-2.0}
fn fp(x: f64) -> f64{2.0*x}
//  -iteracyjnej z pętlą loop (z ewentualnymi break continue return):
pub fn met_newt_loop(x0:f64, eps:f64, n:u128) -> f64{
    let mut x = x0;
    let mut i = 0;
    loop{
        x = x-(f(x)/fp(x));
        i += 1;
        if f(x).abs() <= eps || i > n{
            return x;
        }
    }
}
//  -iteracyjnej z pętlą while (bez żadnych break continue return):
pub fn met_newt_while(x0:f64, eps:f64, n:u128) -> f64{
    let mut x = x0;
    let mut i = 0;
    while f(x).abs() >= eps && i < n{
        x = x-(f(x)/fp(x));
        i += 1;
    }
    x
}
//  -rekurencyjnej:
pub fn met_newt_recursive(x0:f64, eps:f64, n:u128, i:u128) -> f64{
    let mut x = x0;
    let mut i = 0;

    if f(x).abs() <= eps || i > n{
        x
    } else{
        met_newt_recursive(x-(f(x)/fp(x)), eps, n, i + 1)
    }

}
//  -iteracyjnej z pętlą for (z ewentualnymi break continue return):
pub fn met_newt_for(x0: f64, eps: f64, n: u128) -> f64 {
    let mut x = x0;

    for _ in 0..=n {
        x = x - (f(x) / fp(x));
        if f(x).abs() <= eps {
            return x;
        }
    }
    x
}

//Zadanie 2 ADVANCED Zaimplementuj wyznaczanie pierwiastków funkcji rzeczywistej f metodą Newtona w postaci funkcji,
// która zrealizuje liczbę kroków algorytmu przekazaną w argumencie.
// Wyodrębnij funkcję, która zwróci znak pochodnej f' w punkcie.
// Obliczanie wartości funkcji f zrealizuj funkcją wpisaną "na twardo" w implementację metody Newtona.
fn fun(x: f64) -> f64{
    x*x-2.0
}
fn signum(x:f64) -> i8{
    // if x < 0.0{
    //     -1
    // } else if x > 0.0{
    //     1
    // } else{
    //     0
    // }
    let epsilon = 0.0000001;
    if x.abs() < epsilon{
        0
    } else if x > epsilon{
        1
    } else{
        -1
    }
}
fn sgn_f_deriv(x:f64) -> i8{
    let epsilon = 0.0000001;
    let d = fun(x + epsilon) - fun(x);
    signum(d)
}

pub fn newton(n:u64, func: &dyn Fn(f64) -> f64) -> f64{
    let mut x = 7.0;
    let mut delta = 1.0;
    let mut prvs_jump_left = false;
    for i in 0..n{
        let f_val = func(x);
        let deriv_sgn = sgn_f_deriv(x);

        // println!("f: {}", f_val);
        // println!("f': {}", deriv_sgn);
        // println!("x: {}", x);
        // println!("f: {}", f_val);
        // println!("delta: {}", delta);
        // println!("");

        let mut jump_left = false;
        if (signum(f_val) > 0 && deriv_sgn < 0) || (signum(f_val) < 0 && deriv_sgn > 0){
            x += delta;
        } else{
            x -= delta;
            jump_left = true;
        }
        if jump_left != prvs_jump_left {
            delta /= 2.0;
        }
        prvs_jump_left = jump_left;
    }
    x
}
