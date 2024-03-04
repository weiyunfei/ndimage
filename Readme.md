# ndimage

Provides traits that allow conversion between ndarray crate and image crate.

It can be used for debugging, testing, or for using ndarray's image processing capabilities on image crate's images.

## Drawbacks
**The conversion from ndarray to image is not zero-copy.** 
**Only supports RGB and Gray images currently.**


## Usage
Borrow as mutable and call `mut_ndarray` method to get a mutable reference to the ndarray.
```rust
use ndimage::MutNdarray;
use image::{Rgb, RgbImage};

let mut vals = RgbImage::new(2, 4);
vals.mut_ndarray();
```

Borrow as immutable and call `ref_ndarray` method to get a reference to the ndarray.
```rust
use ndimage::RefNdarray;
use image::{Rgb, RgbImage};

let vals = RgbImage::new(2, 4);
vals.ref_ndarray();
```

Convert from image to ndarray.
```rust
use ndimage::IntoNdarray;
use image::{Rgb, RgbImage};

let vals = RgbImage::new(2, 4);
let arr = vals.into_ndarray();
```