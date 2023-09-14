use anyhow::Result; // Automatically handle the error types
use opencv::{
    prelude::*,
    videoio,
    highgui,
    core,
    imgproc
};

fn main() -> Result<()> {
    // Open a GUI window
    highgui::named_window("window", highgui::WINDOW_NORMAL)?;
    highgui::resize_window("window", 640, 480)?;
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the web-cam data
    // Read the camera
    // and display in the window
    loop {
	let timer = core::get_tick_count().unwrap() as f64;
	let fps = core::get_tick_frequency().unwrap() / (core::get_tick_count().unwrap() as f64 - timer);
	let fps_converted = String::from(fps.to_string());
	let mut img = frame.clone();
	let point: core::Point = { core::Point_ { x: 74, y: 50 } };
	let color: core::Scalar = { core::VecN([255.0, 0.0, 0.0, 0.0]) };
	// println!("{:?}", &mut frame.clone());
        cam.read(&mut frame)?;
	imgproc::put_text(&mut img, &fps_converted, point, imgproc::FONT_HERSHEY_COMPLEX, 0.7, color, 1, 8, false)?;
        highgui::imshow("window", &frame)?;
	let rect: core::Rect = { core::Rect { x: 50, y: 50, width: 2, height: 5 } };
	imgproc::rectangle(&mut img, rect, color, 1, 8, 2)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
    }
    Ok(())
}
