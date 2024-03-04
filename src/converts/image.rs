use crate::{IntoNdarray, MutNdarray, RefNdarray};

use ::image::{flat::SampleLayout, ImageBuffer, Pixel};
use ndarray::{
    Array2, Array3, ArrayD, ArrayView2, ArrayView3, ArrayViewD, ArrayViewMut2, ArrayViewMut3,
    ArrayViewMutD, ShapeBuilder,
};
extern crate alloc;
use alloc::vec::Vec;

impl<P> IntoNdarray for ImageBuffer<P, Vec<P::Subpixel>>
where
    P: Pixel + 'static,
{
    type Output = ArrayD<P::Subpixel>;
    fn into_ndarray(self) -> Self::Output {
        let SampleLayout {
            channels,
            channel_stride,
            height,
            height_stride,
            width,
            width_stride,
            ..
        } = self.sample_layout();
        let data = self.into_raw();
        match channels {
            1 => {
                let shape = (height as usize, width as usize);
                let strides = (height_stride, width_stride);
                let array = Array2::from_shape_vec(shape.strides(strides), data).unwrap();
                array.into_dyn()
            }
            3 => {
                let shape = (channels as usize, height as usize, width as usize);
                let strides = (channel_stride, height_stride, width_stride);
                let array = Array3::from_shape_vec(shape.strides(strides), data).unwrap();
                array.into_dyn()
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a, P> RefNdarray for &'a ImageBuffer<P, Vec<P::Subpixel>>
where
    P: Pixel + 'static,
{
    type Output = ArrayViewD<'a, P::Subpixel>;
    fn ref_ndarray(self) -> Self::Output {
        let SampleLayout {
            channels,
            channel_stride,
            height,
            height_stride,
            width,
            width_stride,
            ..
        } = self.sample_layout();
        let ndimage = match channels {
            1 => {
                let shape = (height as usize, width as usize);
                let strides = (height_stride, width_stride);
                ArrayView2::from_shape(shape.strides(strides), &**self)
                    .unwrap()
                    .into_dyn()
            }
            3 => {
                let shape = (channels as usize, height as usize, width as usize);
                let strides = (channel_stride, height_stride, width_stride);
                ArrayView3::from_shape(shape.strides(strides), &**self)
                    .unwrap()
                    .into_dyn()
            }
            _ => unimplemented!(),
        };
        ndimage
    }
}

impl<'a, P> MutNdarray for &'a mut ImageBuffer<P, Vec<P::Subpixel>>
where
    P: Pixel + 'static,
{
    type Output = ArrayViewMutD<'a, P::Subpixel>;
    fn mut_ndarray(self) -> Self::Output {
        let SampleLayout {
            channels,
            channel_stride,
            height,
            height_stride,
            width,
            width_stride,
            ..
        } = self.sample_layout();
        let ndimage = match channels {
            1 => {
                let shape = (height as usize, width as usize);
                let strides = (height_stride, width_stride);
                ArrayViewMut2::from_shape(shape.strides(strides), &mut **self)
                    .unwrap()
                    .into_dyn()
            }
            3 => {
                let shape = (channels as usize, height as usize, width as usize);
                let strides = (channel_stride, height_stride, width_stride);
                ArrayViewMut3::from_shape(shape.strides(strides), &mut **self)
                    .unwrap()
                    .into_dyn()
            }
            _ => unimplemented!(),
        };
        ndimage
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn luma_img_to_ndarray() {
        use crate::IntoNdarray;
        use image::GrayImage;
        use ndarray::Ix2;

        let zeros = GrayImage::new(2, 4);
        let mut nd = zeros.into_ndarray();
        nd.fill(255);
        // convert arrayD to array2
        let nd = nd.into_dimensionality::<Ix2>().unwrap();
        // ndarray uses (row, col), so the dims get flipped.
        assert_eq!(nd.dim(), (4, 2));
    }

    #[test]
    fn rgb_img_to_ndarray() {
        use crate::IntoNdarray;
        use image::RgbImage;
        use ndarray::Ix3;

        let zeros = RgbImage::new(2, 4);
        let nd = zeros.into_ndarray();
        // convert arrayD to array3
        let nd = nd.into_dimensionality::<Ix3>().unwrap();
        // ndarray uses (channels, row, col).
        assert_eq!(nd.dim(), (3, 4, 2));
    }

    #[test]
    fn luma_img_to_ref_ndarray() {
        use crate::RefNdarray;
        use image::{GrayImage, Luma};
        use ndarray::{s, Ix2};

        let mut vals = GrayImage::new(2, 4);
        vals[(1, 0)] = Luma([255]);
        let nd = vals.ref_ndarray();
        let nd = nd.into_dimensionality::<Ix2>().unwrap();
        // ndarray uses (row, col), so the dims get flipped.
        assert_eq!(nd.dim(), (4, 2));
        // The first row should sum to 255.
        assert_eq!(nd.slice(s![0, ..]).sum(), 255);
    }

    #[test]
    fn rgb_img_to_ref_ndarray() {
        use crate::RefNdarray;
        use image::{Rgb, RgbImage};
        use ndarray::{s, Ix3};

        let mut vals = RgbImage::new(2, 4);
        vals[(1, 0)] = Rgb([0, 255, 0]);
        let nd = vals.ref_ndarray();
        let nd = nd.into_dimensionality::<Ix3>().unwrap();
        // ndarray uses (channels, row, col).
        assert_eq!(nd.dim(), (3, 4, 2));
        // The first row should sum to 255.
        assert_eq!(nd.slice(s![1, 0, ..]).sum(), 255);
        // The first row red should sum to 0.
        assert_eq!(nd.slice(s![0, 0, ..]).sum(), 0);
    }

    #[test]
    fn luma_img_to_mut_ndarray() {
        use crate::MutNdarray;
        use image::GrayImage;
        use ndarray::Ix2;

        let mut vals = GrayImage::new(2, 4);
        let nd = vals.mut_ndarray();
        let nd = nd.into_dimensionality::<Ix2>().unwrap();
        // ndarray uses (row, col), so the dims get flipped.
        assert_eq!(nd.dim(), (4, 2));
    }

    #[test]
    fn rgb_img_to_mut_ndarray() {
        use crate::MutNdarray;
        use image::{Rgb, RgbImage};
        use ndarray::s;

        let mut vals = RgbImage::new(2, 4);
        vals.mut_ndarray().slice_mut(s![2, .., ..]).fill(255);
        // ndarray uses (channels, row, col).
        // blue channel should be filled with 255.
        assert_eq!(vals[(0, 0)], Rgb([0, 0, 255]));
    }
}
