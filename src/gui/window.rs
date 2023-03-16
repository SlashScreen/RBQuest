//For launching the window.
use speedy2d::{
    color::Color,
    error::BacktraceError,
    image::ImageFileFormat,
    window::{WindowCreationError, WindowHandler, WindowHelper},
    Window,
};

pub struct RBQuest {}

impl WindowHandler for RBQuest {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut speedy2d::Graphics2D) {
        graphics.clear_screen(Color::from_rgb(1.0, 1.0, 1.0));
        let image = graphics
            .create_image_from_file_path(
                Some(ImageFileFormat::PNG),
                speedy2d::image::ImageSmoothingMode::NearestNeighbor,
                "./res/ruby.png",
            )
            .unwrap();
        graphics.draw_image((50.0, 50.0), &image);
    }
}

pub fn launch() -> Result<Window<()>, BacktraceError<WindowCreationError>> {
    let window = Window::<()>::new_centered("RBQuest", (640, 480))?;
    window.run_loop(RBQuest {});
}
