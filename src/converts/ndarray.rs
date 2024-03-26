use crate::IntoImage;
use image::{DynamicImage, ImageBuffer, Luma, Rgb};

/// The ndarray should be [channels, height, width] for rgb and [height, width] for luma.
impl IntoImage for ndarray::ArrayD<u8> {
    type Output = image::DynamicImage;
    fn into_image(self) -> Self::Output {
        let shape = self.shape().to_owned();

        match shape.len() {
            2 => {
                // TODO: zero copy
                let data = self.into_raw_vec();
                let (height, width) = (shape[0] as u32, shape[1] as u32);
                let image =
                    ImageBuffer::<Luma<u8>, Vec<u8>>::from_raw(width, height, data).unwrap();
                DynamicImage::ImageLuma8(image)
            }
            3 => {
                // TODO: zero copy
                let data = self.into_raw_vec();
                let (height, width, _channels) =
                    (shape[0] as u32, shape[1] as u32, shape[2] as u32);
                let image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(width, height, data).unwrap();
                DynamicImage::ImageRgb8(image)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_into_image() {
        use super::*;
        use image::GenericImageView;
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let shape = vec![3, 2, 3];
        let array = ndarray::ArrayD::from_shape_vec(shape, data).unwrap();
        let image = array.into_image();
        assert_eq!(image.dimensions(), (3, 2));
    }
}
