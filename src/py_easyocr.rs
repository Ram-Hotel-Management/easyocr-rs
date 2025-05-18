use error_pile::PileResult;
use image::DynamicImage;

use std::{
    env::temp_dir,
    io::Cursor,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};

use pyo3::{
    Py, PyAny, Python,
    types::{PyAnyMethods, PyBytes, PyDict, PyList},
};

use crate::types::{Detail1, OCRData};

/// CRAFT Model neeeded for easy ocr
const CRAFT_MODEL_NAME: &str = "craft_mlt_25k.pth";
const CRAFT_MODEL: &[u8] = include_bytes!("../include/craft_mlt_25k.pth");
/// Language model
const EN_MODEL_NAME: &str = "english_g2.pth";
const EN_MODEL: &[u8] = include_bytes!("../include/en.pth");
static IS_WRITTEN: AtomicBool = AtomicBool::new(false);

/// Write the .pth files to the temp dir
/// this will eliminate the model needing to download the necessary
/// pytorch files
fn write_pth_files() -> PileResult<PathBuf> {
    let version = env!("CARGO_PKG_VERSION");
    let temp_dir = temp_dir().join(format!("easyocr-models-v{}", version));

    // already written, saves IO call
    if IS_WRITTEN.load(Ordering::Relaxed) {
        return Ok(temp_dir);
    }

    // create the folder if it does not exist already
    if !temp_dir.exists() {
        std::fs::create_dir(&temp_dir)?;
    }

    let en_model_path = temp_dir.join(EN_MODEL_NAME);

    if !en_model_path.exists() {
        std::fs::write(en_model_path, EN_MODEL)?;
    }

    let craft_model_path = temp_dir.join(CRAFT_MODEL_NAME);

    if !craft_model_path.exists() {
        std::fs::write(craft_model_path, CRAFT_MODEL)?;
    }

    IS_WRITTEN.store(true, Ordering::Relaxed);

    Ok(temp_dir)
}

#[derive(Debug)]
pub struct PyEasyOcr {
    reader: Py<PyAny>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EasyOcrRunArgs<'a> {
    /// "greedy", 'beamsearch', 'wordbeamsearch'
    pub decoder: &'a str,
    /// apply these rotations and returns the one with best confidence score
    pub rotations: &'a [u16],
    /// number of CPU cores to use
    pub cpus: u8,
    /// expect the result to be detailed
    /// returns bounding box based result or simple array of string
    // pub detailed: bool,
    /// Combine result into paragraph
    pub paragraph: bool,
    // /// output format
    // /// expected: 'standard', 'dict', 'json'
    // pub output_format: &'a str,
}

impl Default for EasyOcrRunArgs<'_> {
    fn default() -> Self {
        Self {
            decoder: "greedy",
            cpus: 0,
            // detailed: true,
            rotations: &[],
            // output_format: "standard",
            paragraph: false,
        }
    }
}

impl PyEasyOcr {
    /// Creates an instanc
    pub fn new(use_gpu: bool) -> PileResult<Self> {
        // need to write the .pth files to continue
        let p = write_pth_files()?;
        Python::with_gil(|py| {
            let easyocr_mod = py.import("easyocr")?;

            let reader_attr = easyocr_mod.getattr("Reader")?;

            let kwargs = PyDict::new(py);
            // list of languages
            kwargs.set_item("lang_list", PyList::new(py, ["en"])?)?;
            kwargs.set_item("model_storage_directory", p.to_string_lossy())?;
            kwargs.set_item("user_network_directory", p.to_string_lossy())?;
            // no downloading since everything is packaged
            kwargs.set_item("download_enabled", false)?;
            // set GPU
            kwargs.set_item("gpu", use_gpu)?;

            if !cfg!(debug_assertions) {
                kwargs.set_item("verbose", false)?;
            }

            let reader = reader_attr.call((), Some(&kwargs))?;

            let reader: Py<PyAny> = reader.extract()?;

            Ok(Self { reader })
        })
    }

    /// runs an inference on the provided bytes
    pub fn run(&self, bytes: &[u8], args: Option<EasyOcrRunArgs>) -> PileResult<OCRData> {
        let EasyOcrRunArgs {
            decoder,
            rotations,
            cpus,
            // detailed,
            paragraph,
        } = args.unwrap_or_default();

        // let _detail = if detailed { 1 } else { 0 };
        let decoder = match decoder {
            d if d == "beamsearch" || d == "wordbeamsearch" => d,
            _ => EasyOcrRunArgs::default().decoder,
        };

        Python::with_gil(|py| {
            let kwargs = PyDict::new(py);
            kwargs.set_item("detail", 1)?;
            kwargs.set_item("decoder", decoder)?;
            kwargs.set_item("rotation_info", PyList::new(py, rotations)?)?;
            kwargs.set_item("workers", cpus)?;
            kwargs.set_item("paragraph", paragraph)?;

            let py_bytes = PyBytes::new(py, bytes);

            let result = self
                .reader
                .call_method(py, "readtext", (py_bytes,), Some(&kwargs))?;

            let result = result.extract::<Detail1>(py)?;

            Ok(result.into())
        })
    }

    pub fn run_img(&self, img: &DynamicImage, args: Option<EasyOcrRunArgs>) -> PileResult<OCRData> {
        // Convert DynamicImage to bytes
        let mut img_bytes = Vec::new();

        img.write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::Png)?;

        self.run(&img_bytes, args)
    }
}
