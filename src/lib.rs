use pyo3::prelude::*;
use pyo3::types::PyBytes;
use img_hash::{HasherConfig, HashAlg};
use image::DynamicImage;

/// Converts a raw image buffer from Python into an image::DynamicImage
fn numpy_to_image(image_bytes: &[u8]) -> PyResult<DynamicImage> {
    image::load_from_memory(image_bytes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to decode image: {}", e)))
}

/// Hash an image using img_hash and return an 8-byte hash
#[pyfunction]
fn hash_image(py: Python, image_bytes: &[u8]) -> PyResult<PyObject> {
    let img = numpy_to_image(image_bytes)?;
    
    let hasher = HasherConfig::with_bytes_type::<[u8; 8]>()
        .hash_alg(HashAlg::Gradient)
        .hash_size(8, 8)
        .preproc_dct()
        .to_hasher();

    let hash = hasher.hash_image(&img);
    
    Ok(PyBytes::new(py, &hash.as_bytes()).into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn img_hash_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash_image, m)?)?;
    Ok(())
}
