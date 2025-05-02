use easyocr_rs::{DoclingRunArgs, PyEasyOcr};

#[test]
fn is_runnable() {
    let easy_ocr = PyEasyOcr::new(true).unwrap();
    let bytes = include_bytes!("./1.jpg");
    let res = easy_ocr
        .run(
            bytes,
            Some(DoclingRunArgs {
                rotations: &[90, 180, 270],
                ..Default::default()
            }),
        )
        .unwrap();

    dbg!(res);
}
