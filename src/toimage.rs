/// Convert a type into image
pub trait IntoImage {
    type Output;
    fn into_image(self) -> Self::Output;
}
