use anyhow::Result; // Automatically handle the error types
use opencv::{
    prelude::*,
    videoio,
    highgui
};

fn main() -> Result<()> {
    // Open a GUI window
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the web-cam data
    // Read the camera
    // and display in the window
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // quit with q
            break;
        }
    }
    Ok(())
}
