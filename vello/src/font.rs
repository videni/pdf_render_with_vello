use font::{self, Encoder, FontType, FontVariant, Pen};
use vello_encoding::{Encoding, PathEncoder as VelloPathEncoder};
use pathfinder_geometry::vector::Vector2F;

pub struct GlyphData {
    encoding: vello_encoding::Encoding,
    offsets: Vec<Offset>
}
struct Offset {
    path_tag: usize,
    path_data: usize,
    n_path_segments: u32,
}
impl GlyphData {
    pub fn new() -> Self {
        GlyphData { encoding: Encoding::new(), offsets: vec![] }
    }
}

impl font::Encoder for GlyphData {
    type Pen<'a> = PathEncoder<'a>;
    type GlyphRef = u32;
    fn encode_shape<'f, O, E>(&mut self, mut f: impl for<'b> FnMut(&mut Self::Pen<'b>) -> Result<O, E> + 'f) -> Result<(O, Self::GlyphRef), E> {
        let mut p: PathEncoder = PathEncoder(self.encoding.encode_path(true));
        let o = f(&mut p)?;
        p.finish(true);
        self.offsets.push(Offset {
            path_tag: self.encoding.path_tags.len(),
            path_data: self.encoding.path_data.len(),
            n_path_segments: self.encoding.n_path_segments,
        });

        Ok((o, self.encoding.n_paths))
    }
}

pub struct PathEncoder<'a>(VelloPathEncoder<'a>);

impl <'a> PathEncoder<'a> {
    pub fn finish(self, insert_path_marker: bool) -> u32 {
        self.0.finish(insert_path_marker)
    }
}

impl<'a> Pen for PathEncoder<'a> {
    fn move_to(&mut self, p: Vector2F) {
        self.0.move_to(p.x(), p.y())
    }

    fn line_to(&mut self, p: Vector2F) {
        self.0.line_to(p.x(), p.y())
    }

    fn quad_to(&mut self, p1: Vector2F, p2: Vector2F) {
        self.0.quad_to(p1.x(), p1.y(), p2.x(), p2.y())
    }

    fn cubic_to(&mut self, p1: Vector2F, p2: Vector2F, p3: Vector2F) {
        self.0.cubic_to(p1.x(), p1.y(), p2.x(), p2.y(), p3.x(), p3.y())
    }

    fn close(&mut self) {
        self.0.close()
    }
}