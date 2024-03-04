/// Convert a type into ndarray
pub trait IntoNdarray {
    type Output;
    fn into_ndarray(self) -> Self::Output;
}

/// Borrow a type into ndarray
pub trait RefNdarray {
    type Output;
    fn ref_ndarray(self) -> Self::Output;
}

/// Borrow a type into mutable ndarray
pub trait MutNdarray {
    type Output;
    fn mut_ndarray(self) -> Self::Output;
}
