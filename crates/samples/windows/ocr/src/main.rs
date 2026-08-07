fn main() -> windows::core::Result<()> {
    use windows::{
        Graphics::Imaging::BitmapDecoder,
        Media::Ocr::OcrEngine,
        Storage::{FileAccessMode, StorageFile},
        core::*,
    };

    let message = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("message.png");

    if !message.exists() {
        return Err(Error::new(
            WIN32_ERROR(2).to_hresult(),
            format!("sample image not found: {}", message.display()),
        ));
    }

    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(message.to_str().unwrap()))?.join()?;
    let stream = file.OpenAsync(FileAccessMode::Read)?.join()?;

    let decode = BitmapDecoder::CreateAsync(&stream)?.join()?;
    let bitmap = decode.GetSoftwareBitmapAsync()?.join()?;

    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(&bitmap)?.join()?;

    println!("{:?}", result.Text()?);
    Ok(())
}
