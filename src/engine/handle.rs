use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/**
 * Wrapper to support runtime-specific operations.
 * This lets us call operations
 * without carrying around the operation-specific data in all instances.
 * The handle should always be passed as a reference.
 * */
#[derive(Debug, Clone, Copy)]
pub struct Wrap<H, T> {
    handle: H,
    pub value: T,
}

impl<H, T> Wrap<&H, T> {
    pub fn new<'a>(handle: &H, value: T) -> Wrap<&H, T> {
        Wrap { handle, value }
    }

    pub fn handle(&self) -> &H {
        self.handle
    }

    pub fn wrap<S>(&self, value: S) -> Wrap<&H, S> {
        Wrap {
            handle: self.handle,
            value,
        }
    }
}

// TODO Maybe out-of-place pattern instead of self-replacing call?

/**
 * Handle for additive groups.
 * Following rust conventions,
 * operators with results have owning view,
 * and operators which modify have non-owning mutable view.
 * */
pub trait AddGroupHandle<G> {
    fn h_zero(&self) -> G;
    fn h_is_zero(&self, val: &G) -> bool;
    fn h_add(&self, lhs: G, rhs: &G) -> G;
    fn h_sub(&self, lhs: G, rhs: &G) -> G;
    fn h_add_assign(&self, lhs: &mut G, rhs: &G);
    fn h_sub_assign(&self, lhs: &mut G, rhs: &G);
}

impl<G, H: AddGroupHandle<G>> Add<&G> for Wrap<&H, G> {
    type Output = Self;

    fn add(self, rhs: &G) -> Self {
        let Wrap { handle, value } = self;
        Wrap {
            handle,
            value: handle.h_add(value, rhs),
        }
    }
}

impl<G, Op: AddGroupHandle<G>> Sub<&G> for Wrap<&Op, G> {
    type Output = Self;

    fn sub(self, rhs: &G) -> Self {
        let Wrap { handle, value } = self;
        Wrap {
            handle,
            value: handle.h_sub(value, rhs),
        }
    }
}

impl<G, Op: AddGroupHandle<G>> AddAssign<&G> for Wrap<&Op, &mut G> {
    fn add_assign(&mut self, rhs: &G) {
        self.handle.h_add_assign(self.value, rhs);
    }
}

impl<G, Op: AddGroupHandle<G>> SubAssign<&G> for Wrap<&Op, &mut G> {
    fn sub_assign(&mut self, rhs: &G) {
        self.handle.h_sub_assign(self.value, rhs);
    }
}

/**
 * Handle for ring operations.
 * */
pub trait RingHandle<R>: AddGroupHandle<R> {
    fn h_one(&self) -> R;
    fn h_mul(&self, lhs: R, rhs: &R) -> R;
    fn h_mul_assign(&self, lhs: &mut R, rhs: &R);
}

impl<R, H: RingHandle<R>> Mul<&R> for Wrap<&H, R> {
    type Output = Self;

    fn mul(self, rhs: &R) -> Self {
        let Wrap { handle, value } = self;
        Wrap {
            handle,
            value: handle.h_mul(value, rhs),
        }
    }
}

impl<R, H: RingHandle<R>> MulAssign<&R> for Wrap<&H, &mut R> {
    fn mul_assign(&mut self, rhs: &R) {
        self.handle.h_mul_assign(self.value, rhs);
    }
}

pub trait FieldHandle<F>: RingHandle<F> {
    fn h_div(&self, lhs: F, rhs: &F) -> F;
    fn h_div_assign(&self, lhs: &mut F, rhs: &F);
}

impl<F, H: FieldHandle<F>> Div<&F> for Wrap<&H, F> {
    type Output = Self;

    fn div(self, rhs: &F) -> Self {
        let Wrap { handle, value } = self;
        Wrap {
            handle,
            value: handle.h_div(value, rhs),
        }
    }
}

impl<F, H: FieldHandle<F>> DivAssign<&F> for Wrap<&H, &mut F> {
    fn div_assign(&mut self, rhs: &F) {
        self.handle.h_div_assign(self.value, rhs);
    }
}

// /**
//  * Trivial operator which inherits native operation.
//  * */
// #[derive(Debug, Clone, Copy)]
// pub struct TrivialOp<R>(PhantomData<R>);
//
// impl<R: NativeRing> OpAddGroup<R> for TrivialOp<R> {
//     fn op_zero(&self) -> R {
//         zero()
//     }
//
//     fn is_zero(&self, val: &R) -> bool {
//         val.is_zero()
//     }
//
//     fn op_add(&self, lhs: R, rhs: &R) -> R {
//         lhs + *rhs
//     }
//
//     fn op_sub(&self, lhs: R, rhs: &R) -> R {
//         lhs - *rhs
//     }
//
//     fn op_add_assign(&self, lhs: &mut R, rhs: &R) {
//         *lhs += *rhs;
//     }
//
//     fn op_sub_assign(&self, lhs: &mut R, rhs: &R) {
//         *lhs -= *rhs;
//     }
// }
//
// impl<R: NativeRing> OpRing<R> for TrivialOp<R> {
//     fn op_one(&self) -> R {
//         one()
//     }
//
//     fn op_mul(&self, lhs: R, rhs: &R) -> R {
//         lhs * *rhs
//     }
//
//     fn op_mul_assign(&self, lhs: &mut R, rhs: &R) {
//         *lhs *= *rhs
//     }
// }
//
// impl<R: NativeRing + Div<Output = R> + DivAssign> OpField<R> for TrivialOp<R> {
//     fn op_div(&self, lhs: R, rhs: &R) -> R {
//         lhs / *rhs
//     }
//
//     fn op_div_assign(&self, lhs: &mut R, rhs: &R) {
//         *lhs /= *rhs
//     }
// }
