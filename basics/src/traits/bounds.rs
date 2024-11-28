use crate::traits::primitives::Operations;

fn quadruple<T: Operations>(x: T) -> T {
    x.double().double()
}
