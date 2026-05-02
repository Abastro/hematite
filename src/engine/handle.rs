use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Wrapper to support runtime-specific operations, for e.g. modular arithmetic.
///
/// This lets us call operations without carrying around the operation-specific data in all instances.
///
/// The handle should always be passed as a reference.
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

/// Handle for additive groups.
/// Note the difference of in-place operations and out-of-place operations.
pub trait AddGroupHandle<G: ?Sized> {
    fn h_set_zero(&self, val: &mut G);
    fn h_is_zero(&self, val: &G) -> bool;
    fn h_add(&self, lhs: &G, rhs: &G, out: &mut G);
    fn h_add_assign(&self, lhs: &mut G, rhs: &G);
    fn h_neg(&self, arg: &G, out: &mut G);
    fn h_neg_assign(&self, arg: &mut G);
    fn h_sub(&self, lhs: &G, rhs: &G, out: &mut G);
    fn h_sub_assign(&self, lhs: &mut G, rhs: &G);
}

/// Operations for additive groups - Copy version of AddGroupHandle.
pub trait AddGroupOps<G: Copy> {
    fn zero(&self) -> G;
    fn add(&self, lhs: G, rhs: G) -> G;
    fn neg(&self, arg: G) -> G;
    fn sub(&self, lhs: G, rhs: G) -> G;
}

impl<G, H> AddGroupHandle<G> for H
where
    G: Copy + PartialEq,
    H: AddGroupOps<G>,
{
    fn h_set_zero(&self, val: &mut G) {
        *val = self.zero();
    }

    fn h_is_zero(&self, val: &G) -> bool {
        *val == self.zero()
    }

    fn h_add(&self, lhs: &G, rhs: &G, out: &mut G) {
        *out = self.add(*lhs, *rhs);
    }

    fn h_add_assign(&self, lhs: &mut G, rhs: &G) {
        *lhs = self.add(*lhs, *rhs);
    }

    fn h_neg(&self, arg: &G, out: &mut G) {
        *out = self.neg(*arg);
    }

    fn h_neg_assign(&self, arg: &mut G) {
        *arg = self.neg(*arg);
    }

    fn h_sub(&self, lhs: &G, rhs: &G, out: &mut G) {
        *out = self.sub(*lhs, *rhs);
    }

    fn h_sub_assign(&self, lhs: &mut G, rhs: &G) {
        *lhs = self.sub(*lhs, *rhs);
    }
}

impl<G, H> AddAssign<&G> for Wrap<&H, &mut G>
where
    H: AddGroupHandle<G>,
{
    fn add_assign(&mut self, rhs: &G) {
        self.handle.h_add_assign(self.value, rhs);
    }
}

impl<G, H> SubAssign<&G> for Wrap<&H, &mut G>
where
    H: AddGroupHandle<G>,
{
    fn sub_assign(&mut self, rhs: &G) {
        self.handle.h_sub_assign(self.value, rhs);
    }
}

impl<G, H> Add<&G> for Wrap<&H, G>
where
    H: AddGroupHandle<G>,
{
    type Output = Self;

    fn add(mut self, rhs: &G) -> Self {
        self.handle.h_add_assign(&mut self.value, rhs);
        self
    }
}

impl<G, H> Sub<&G> for Wrap<&H, G>
where
    H: AddGroupHandle<G>,
{
    type Output = Self;

    fn sub(mut self, rhs: &G) -> Self {
        self.handle.h_sub_assign(&mut self.value, rhs);
        self
    }
}

/// Handle for ring operations.
///
/// We only consider commutative rings in our convention, for HE purposes.
pub trait RingHandle<R: ?Sized>: AddGroupHandle<R> {
    fn h_set_one(&self, val: &mut R);
    fn h_mul(&self, lhs: &R, rhs: &R, out: &mut R);
    fn h_mul_assign(&self, lhs: &mut R, rhs: &R);
}

/// Copy version of RingHandle.
pub trait RingOps<R: Copy>: AddGroupOps<R> {
    fn one(&self) -> R;
    fn mul(&self, lhs: R, rhs: R) -> R;
}

impl<R, H> RingHandle<R> for H
where
    R: Copy + PartialEq,
    H: RingOps<R>,
{
    fn h_set_one(&self, val: &mut R) {
        *val = self.one();
    }

    fn h_mul(&self, lhs: &R, rhs: &R, out: &mut R) {
        *out = self.mul(*lhs, *rhs);
    }

    fn h_mul_assign(&self, lhs: &mut R, rhs: &R) {
        *lhs = self.mul(*lhs, *rhs);
    }
}

impl<R, H: RingHandle<R>> Mul<&R> for Wrap<&H, R> {
    type Output = Self;

    fn mul(mut self, rhs: &R) -> Self {
        self.handle.h_mul_assign(&mut self.value, rhs);
        self
    }
}

impl<R, H: RingHandle<R>> MulAssign<&R> for Wrap<&H, &mut R> {
    fn mul_assign(&mut self, rhs: &R) {
        self.handle.h_mul_assign(self.value, rhs);
    }
}

/// Handle for field operations.
pub trait FieldHandle<F: ?Sized>: RingHandle<F> {
    fn h_inv(&self, arg: &F, out: &mut F);
    fn h_inv_assign(&self, arg: &mut F);
    fn h_div(&self, lhs: &F, rhs: &F, out: &mut F);
    fn h_div_assign(&self, lhs: &mut F, rhs: &F);
}

/// Copy version of FieldHandle.
pub trait FieldOps<F: Copy>: RingOps<F> {
    fn inv(&self, arg: F) -> F;
    fn div(&self, lhs: F, rhs: F) -> F;
}

impl<F, H> FieldHandle<F> for H
where
    F: Copy + PartialEq,
    H: FieldOps<F>,
{
    fn h_inv(&self, arg: &F, out: &mut F) {
        *out = self.inv(*arg);
    }

    fn h_inv_assign(&self, arg: &mut F) {
        *arg = self.inv(*arg);
    }

    fn h_div(&self, lhs: &F, rhs: &F, out: &mut F) {
        *out = self.div(*lhs, *rhs);
    }

    fn h_div_assign(&self, lhs: &mut F, rhs: &F) {
        *lhs = self.div(*lhs, *rhs);
    }
}

impl<F, H: FieldHandle<F>> Div<&F> for Wrap<&H, F> {
    type Output = Self;

    fn div(mut self, rhs: &F) -> Self {
        self.handle.h_div_assign(&mut self.value, rhs);
        self
    }
}

impl<F, H: FieldHandle<F>> DivAssign<&F> for Wrap<&H, &mut F> {
    fn div_assign(&mut self, rhs: &F) {
        self.handle.h_div_assign(self.value, rhs);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use proptest::{prelude::*, test_runner::TestRunner};

    // TODO Add test helpers for non-copying handles

    /// Addition is associative: (a + b) + c == a + (b + c)
    pub fn ops_add_assoc<G, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        G: Copy + std::fmt::Debug + PartialEq,
        Ops: AddGroupOps<G> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = G>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy.prop_ind_flat_map2(move |ops| {
                    (
                        value_strategy(&ops),
                        value_strategy(&ops),
                        value_strategy(&ops),
                    )
                }),
                |(ops, (a, b, c))| {
                    prop_assert_eq!(
                        ops.add(ops.add(a, b), c),
                        ops.add(a, ops.add(b, c)),
                        "add-associativity"
                    );
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Addition is commutative: a + b == b + a
    pub fn ops_add_comm<G, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        G: Copy + std::fmt::Debug + PartialEq,
        Ops: AddGroupOps<G> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = G>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy
                    .prop_ind_flat_map2(move |ops| (value_strategy(&ops), value_strategy(&ops))),
                |(ops, (a, b))| {
                    prop_assert_eq!(ops.add(a, b), ops.add(b, a), "add-commutativity");
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Addition has identity: a + 0 == a == 0 + a
    pub fn ops_add_identity<G, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        G: Copy + std::fmt::Debug + PartialEq,
        Ops: AddGroupOps<G> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = G>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy.prop_ind_flat_map2(move |ops| value_strategy(&ops)),
                |(ops, a)| {
                    prop_assert_eq!(ops.add(a, ops.zero()), a, "add-identity-left");
                    prop_assert_eq!(ops.add(ops.zero(), a), a, "add-identity-right");
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Addition has inverse: a + (-a) == 0 == (-a) + a
    pub fn ops_add_inverse<G, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        G: Copy + std::fmt::Debug + PartialEq,
        Ops: AddGroupOps<G> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = G>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy.prop_ind_flat_map2(move |ops| value_strategy(&ops)),
                |(ops, a)| {
                    prop_assert_eq!(ops.add(a, ops.neg(a)), ops.zero(), "add-inverse-left");
                    prop_assert_eq!(ops.add(ops.neg(a), a), ops.zero(), "add-inverse-right");
                    Ok(())
                },
            )
            .unwrap();
    }

    // TODO Subtraction compatibility
    /// Subtraction compatibility with negate: a - b == a + (- b)
    pub fn ops_sub_compat<G, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        G: Copy + std::fmt::Debug + PartialEq,
        Ops: AddGroupOps<G> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = G>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy
                    .prop_ind_flat_map2(move |ops| (value_strategy(&ops), value_strategy(&ops))),
                |(ops, (a, b))| {
                    prop_assert_eq!(ops.sub(a, b), ops.add(a, ops.neg(b)), "sub-compat");
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Multiplication is associative: (a * b) * c == a * (b * c)
    pub fn ops_mul_assoc<R, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        R: Copy + std::fmt::Debug + PartialEq,
        Ops: RingOps<R> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = R>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy.prop_ind_flat_map2(move |ops| {
                    (
                        value_strategy(&ops),
                        value_strategy(&ops),
                        value_strategy(&ops),
                    )
                }),
                |(ops, (a, b, c))| {
                    prop_assert_eq!(
                        ops.mul(ops.mul(a, b), c),
                        ops.mul(a, ops.mul(b, c)),
                        "mul-associativity"
                    );
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Multiplication is commutative: a * b == b * a
    pub fn ops_mul_comm<R, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        R: Copy + std::fmt::Debug + PartialEq,
        Ops: RingOps<R> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = R>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy
                    .prop_ind_flat_map2(move |ops| (value_strategy(&ops), value_strategy(&ops))),
                |(ops, (a, b))| {
                    prop_assert_eq!(ops.mul(a, b), ops.mul(b, a), "mul-commutativity");
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Multiplication has identity: a * 1 == a == 1 * a
    pub fn ops_mul_identity<R, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        R: Copy + std::fmt::Debug + PartialEq,
        Ops: RingOps<R> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = R>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy.prop_ind_flat_map2(move |ops| value_strategy(&ops)),
                |(ops, a)| {
                    prop_assert_eq!(ops.mul(a, ops.one()), a, "mul-identity-left");
                    prop_assert_eq!(ops.mul(ops.one(), a), a, "mul-identity-right");
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Multiplication has inverse: a * a^-1 == 1 == a^-1 * a, for a != 0.
    pub fn ops_mul_inverse<Fld, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        Fld: Copy + std::fmt::Debug + PartialEq,
        Ops: FieldOps<Fld> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = Fld>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy.prop_ind_flat_map2(move |ops| value_strategy(&ops)),
                |(ops, a)| {
                    prop_assume!(a != ops.zero());
                    prop_assert_eq!(ops.mul(a, ops.inv(a)), ops.one(), "mul-inverse-left");
                    prop_assert_eq!(ops.mul(ops.inv(a), a), ops.one(), "mul-inverse-right");
                    Ok(())
                },
            )
            .unwrap();
    }

    /// Division compatibility with inverse: a * b^(-1) = a / b, for b != 0.
    pub fn ops_div_compat<Fld, Ops, SO, SV, F>(ops_strategy: SO, value_strategy: F)
    where
        Fld: Copy + std::fmt::Debug + PartialEq,
        Ops: FieldOps<Fld> + std::fmt::Debug,
        SO: Strategy<Value = Ops>,
        SV: Strategy<Value = Fld>,
        F: Fn(&Ops) -> SV,
    {
        let mut runner = TestRunner::default();
        runner
            .run(
                &ops_strategy
                    .prop_ind_flat_map2(move |ops| (value_strategy(&ops), value_strategy(&ops))),
                |(ops, (a, b))| {
                    prop_assume!(b != ops.zero());
                    prop_assert_eq!(ops.div(a, b), ops.mul(a, ops.inv(b)), "div-compat");
                    Ok(())
                },
            )
            .unwrap();
    }
}
