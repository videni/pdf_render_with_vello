use argh::FromArgs;
use pdf::file::{File, FileOptions};
use pdf_render::{Cache, render_page, font::OutlineBuilder};
use pdf_render_vello::backend::VelloBackend;
use pathfinder_geometry::transform2d::Transform2F;
use std::error::Error;

use std::path::PathBuf;

#[derive(FromArgs)]
///  PDF rasterizer
struct Options {
    /// DPI
    #[argh(option, default="150.")]
    dpi: f32,

    /// page to render (0 based)
    #[argh(option, default="0")]
    page: u32,

    /// input PDF file
    #[argh(positional)]
    pdf: PathBuf,

    /// output image
    #[argh(positional)]
    image: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let opt: Options = argh::from_env();
    
    let file = FileOptions::uncached().open(&opt.pdf)?;

    let resolver = file.resolver();
    let page = file.get_page(opt.page)?;

    let mut cache = Cache::new(OutlineBuilder{});
    let mut backend = VelloBackend::new(&mut cache);

    render_page(&mut backend, &resolver, &page, Transform2F::from_scale(opt.dpi / 25.4))?;

    //TODO: Turn Vello scene to image
    // let image = Rasterizer::new().rasterize(backend.finish(), None);

    // image.save(opt.image)?;

    Ok(())
}