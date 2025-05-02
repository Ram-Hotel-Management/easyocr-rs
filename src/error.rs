use pyo3::PyErr;

pub type EasyOcrResult<T = ()> = Result<T, EasyOcrErr>;

#[derive(Debug, thiserror::Error)]
pub enum EasyOcrErr {
    #[error("An error occurred while {0}")]
    IO(
        #[from]
        #[source]
        std::io::Error,
    ),

    #[error("An error occurred on Python Side: {0}")]
    Python(
        #[from]
        #[source]
        PyErr,
    ),
}
