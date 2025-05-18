type BBoxPointType = f32; // TODO can this be switched over to usize?
type BBoxPoint = [BBoxPointType; 2];
type EasyOcrBBox = [BBoxPoint; 4];

#[repr(C)]
/// Returns the Bounding Box of the provided
/// text in the format of (x,y)
/// Need to
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BoundingBox {
    pub tl: BBoxPoint,
    pub tr: BBoxPoint,
    pub br: BBoxPoint,
    pub bl: BBoxPoint,
}

impl From<EasyOcrBBox> for BoundingBox {
    fn from(value: EasyOcrBBox) -> Self {
        Self {
            tl: value[0],
            tr: value[1],
            br: value[2],
            bl: value[3],
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DetectedText {
    pub bbox: BoundingBox,
    pub text: String,
    pub confidence: f32,
}

/// Represents the simplest form the EasyOcr can return data
pub(crate) type Detail0 = Vec<String>;

/// Represents the Detected Text in the Image
/// 0: Bounding Box - [[189, 75], [469, 75], [469, 165], [189, 165]]
/// 1: The text detected
/// 2: The confidence score - returns a result between 0 - 1
/// Full example: ([[189, 75], [469, 75], [469, 165], [189, 165]], 'Text', 0.3754989504814148)
pub(crate) type Detail1 = Vec<(EasyOcrBBox, String, f32)>;

impl From<Detail1> for OCRData {
    fn from(value: Detail1) -> Self {
        Self {
            texts: value
                .into_iter()
                .map(|d1| DetectedText {
                    bbox: d1.0.into(),
                    text: d1.1,
                    confidence: d1.2,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OCRData {
    pub texts: Vec<DetectedText>,
}

impl OCRData {
    /// extracts only a text without the bounding box
    pub fn into_txt_vec(self) -> Detail0 {
        self.into()
    }
}

impl From<OCRData> for Detail0 {
    fn from(value: OCRData) -> Self {
        let details = value.texts;
        details.into_iter().map(|dt| dt.text).collect()
    }
}
