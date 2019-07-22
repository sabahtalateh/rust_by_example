struct A;

struct S(A);

struct SGen<T>(T);

fn reg_fn(_s: S) {}

// SGen is a generic type, but it parametrized with A so
//  the functions is not generic anymore.
fn gen_spec_t(_s: SGen<A>) {}

// Not a generic also
fn gen_spec_i32(_s: SGen<i32>) {}

// Generic
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(12));

    generic::<char>(SGen('c'));
    generic(SGen('c'));
}
