use ocaml_interop::{
    OCaml, OCamlInt, OCamlRuntime
};

mod tz_calls {
    use ocaml_interop::{ocaml, OCamlInt};

    ocaml! {
        pub fn tz_multiply(_x: OCamlInt, _y: OCamlInt) -> OCamlInt;
    }
}

fn tz_multiply(cr: &mut OCamlRuntime, x: isize, y: isize) -> i64 {
    let ocaml_x : OCaml<'static, OCamlInt> =
        unsafe { OCaml::of_i64_unchecked(x as i64) } ;

    let ocaml_y : OCaml<'static, OCamlInt> =
        unsafe { OCaml::of_i64_unchecked(y as i64) } ;

    let result = tz_calls::tz_multiply(cr, &ocaml_x, &ocaml_y);

    result.to_rust(cr)
}


fn main() {
    let mut args = std::env::args();
    args.next();

    let x = args.next().and_then(|x| x.parse::<isize>().ok()).unwrap_or_else(|| 6isize);
    let y = args.next().and_then(|x| x.parse::<isize>().ok()).unwrap_or_else(|| 7isize);

    let mut cr = OCamlRuntime::init();

    let z = tz_multiply(&mut cr, x, y);

    println!("{} * {} -> {}", x, y, z)
}
