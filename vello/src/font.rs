use font::{self, Encoder, FontType, FontVariant};
use vello_encoding::{Encoding, PathEncoder};

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
        let mut p: PathEncoder = self.encoding.encode_path(true);
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