// INFO: 240201 クレートのバージョンの組み合わせでは動かないので、注意。(https://github.com/kennytm/qrcode-rust/issues/19)
use qrcode::QrCode;  // "0.12.0"
use image::Luma;     // "0.23.14"

fn main() {
    // Encode some data into bits.
    let code = QrCode::new(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("./qrcode.png").unwrap();  // INFO: 240201 ディレクトリが存在しない場合に落ちるので注意。

    // You can also render it into a string.
    let string = code.render()
        .light_color('　')
        .dark_color('■')
        .build();
    println!("{}", string);
}