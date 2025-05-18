use easyocr_rs::{EasyOcrRunArgs, PyEasyOcr};

#[test]
fn is_runnable() {
    let easy_ocr = PyEasyOcr::new(true).unwrap();
    let bytes = include_bytes!("./1.jpg");
    let res = easy_ocr
        .run(
            bytes,
            Some(EasyOcrRunArgs {
                rotations: &[90, 180, 270],
                ..Default::default()
            }),
        )
        .unwrap();

    dbg!(res);
}
