use super::*;

/// Writer for a _Type-1 font_.
///
/// This struct is created by [`PdfWriter::type1_font`].
pub struct Type1Font<'a> {
    dict: Dict<'a, IndirectGuard>,
}

impl<'a> Type1Font<'a> {
    pub(crate) fn start(any: Any<'a, IndirectGuard>) -> Self {
        let mut dict = any.dict();
        dict.pair(Name(b"Type"), Name(b"Font"));
        dict.pair(Name(b"Subtype"), Name(b"Type1"));
        Self { dict }
    }

    /// Write the `/BaseFont` attribute.
    pub fn base_font(&mut self, name: Name) -> &mut Self {
        self.dict.pair(Name(b"BaseFont"), name);
        self
    }
}

/// Writer for a _Type-0 (composite) font_.
///
/// This struct is created by [`PdfWriter::type0_font`].
pub struct Type0Font<'a> {
    dict: Dict<'a, IndirectGuard>,
}

impl<'a> Type0Font<'a> {
    pub(crate) fn start(any: Any<'a, IndirectGuard>) -> Self {
        let mut dict = any.dict();
        dict.pair(Name(b"Type"), Name(b"Font"));
        dict.pair(Name(b"Subtype"), Name(b"Type0"));
        Self { dict }
    }

    /// Write the `/BaseFont` attribute.
    pub fn base_font(&mut self, name: Name) -> &mut Self {
        self.dict.pair(Name(b"BaseFont"), name);
        self
    }

    /// Write the `/Encoding` attribute as a predefined encoding.
    pub fn encoding_predefined(&mut self, encoding: Name) -> &mut Self {
        self.dict.pair(Name(b"Encoding"), encoding);
        self
    }

    /// Write the `/Encoding` attribute as a reference to a character map.
    pub fn encoding_cmap(&mut self, cmap: Ref) -> &mut Self {
        self.dict.pair(Name(b"Encoding"), cmap);
        self
    }

    /// Write the `/DescendantFonts` attribute as a one-element array containing
    /// a reference to a [`CidFont`].
    pub fn descendant_font(&mut self, cid_font: Ref) -> &mut Self {
        self.dict.key(Name(b"DescendantFonts")).array().item(cid_font);
        self
    }

    /// Write the `/ToUnicode` attribute.
    ///
    /// A suitable character map can be built with [`UnicodeCmap`].
    pub fn to_unicode(&mut self, cmap: Ref) -> &mut Self {
        self.dict.pair(Name(b"ToUnicode"), cmap);
        self
    }
}

/// Writer for a _CID font_, a descendant of a [`Type0Font`].
///
/// This struct is created by [`PdfWriter::cid_font`].
pub struct CidFont<'a> {
    dict: Dict<'a, IndirectGuard>,
}

impl<'a> CidFont<'a> {
    pub(crate) fn start(any: Any<'a, IndirectGuard>, subtype: CidFontType) -> Self {
        let mut dict = any.dict();
        dict.pair(Name(b"Type"), Name(b"Font"));
        dict.pair(Name(b"Subtype"), subtype.name());
        Self { dict }
    }

    /// Write the `/BaseFont` attribute.
    pub fn base_font(&mut self, name: Name) -> &mut Self {
        self.dict.pair(Name(b"BaseFont"), name);
        self
    }

    /// Write the `/CIDSystemInfo` dictionary.
    pub fn system_info(&mut self, info: SystemInfo) -> &mut Self {
        info.write(self.dict.key(Name(b"CIDSystemInfo")));
        self
    }

    /// Write the `/FontDescriptor` attribute.
    pub fn font_descriptor(&mut self, descriptor: Ref) -> &mut Self {
        self.dict.pair(Name(b"FontDescriptor"), descriptor);
        self
    }

    /// Start writing the `/W` (widths) array.
    pub fn widths(&mut self) -> Widths<'_> {
        Widths::start(self.dict.key(Name(b"W")))
    }
}

/// The subtype of a CID font.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CidFontType {
    /// A CID font containing CFF glyph descriptions.
    Type0,
    /// A CID font containing TrueType glyph descriptions.
    Type2,
}

impl CidFontType {
    fn name(self) -> Name<'static> {
        match self {
            Self::Type0 => Name(b"CIDFontType0"),
            Self::Type2 => Name(b"CIDFontType2"),
        }
    }
}

/// Writer for the _width array_ in a [`CidFont`].
///
/// This struct is created by [`CidFont::widths`].
pub struct Widths<'a> {
    array: Array<'a>,
}

impl<'a> Widths<'a> {
    pub(crate) fn start(any: Any<'a>) -> Self {
        Self { array: any.array() }
    }

    /// Specifies individual widths for a range of CIDs starting at `start`.
    pub fn individual(
        &mut self,
        start: u16,
        widths: impl IntoIterator<Item = f32>,
    ) -> &mut Self {
        self.array.item(i32::from(start));
        self.array.any().array().typed().items(widths);
        self
    }

    /// Specifies the same width for all CIDs between `first` and `last`.
    pub fn same(&mut self, first: u16, last: u16, width: f32) -> &mut Self {
        self.array.item(i32::from(first));
        self.array.item(i32::from(last));
        self.array.item(width);
        self
    }
}

/// Writer for a _font descriptor_.
///
/// This struct is created by [`PdfWriter::font_descriptor`].
pub struct FontDescriptor<'a> {
    dict: Dict<'a, IndirectGuard>,
}

impl<'a> FontDescriptor<'a> {
    pub(crate) fn start(any: Any<'a, IndirectGuard>) -> Self {
        let mut dict = any.dict();
        dict.pair(Name(b"Type"), Name(b"FontDescriptor"));
        Self { dict }
    }

    /// Write the `/FontName` attribute.
    pub fn font_name(&mut self, name: Name) -> &mut Self {
        self.dict.pair(Name(b"FontName"), name);
        self
    }

    /// Write the `/Flags` attribute.
    pub fn font_flags(&mut self, flags: FontFlags) -> &mut Self {
        self.dict.pair(Name(b"Flags"), flags.bits() as i32);
        self
    }

    /// Write the `/FontBBox` attribute.
    pub fn font_bbox(&mut self, bbox: Rect) -> &mut Self {
        self.dict.pair(Name(b"FontBBox"), bbox);
        self
    }

    /// Write the `/ItalicAngle` attribute.
    pub fn italic_angle(&mut self, angle: f32) -> &mut Self {
        self.dict.pair(Name(b"ItalicAngle"), angle);
        self
    }

    /// Write the `/Ascent` attribute.
    pub fn ascent(&mut self, ascent: f32) -> &mut Self {
        self.dict.pair(Name(b"Ascent"), ascent);
        self
    }

    /// Write the `/Descent` attribute.
    pub fn descent(&mut self, descent: f32) -> &mut Self {
        self.dict.pair(Name(b"Descent"), descent);
        self
    }

    /// Write the `/CapHeight` attribute.
    pub fn cap_height(&mut self, cap_height: f32) -> &mut Self {
        self.dict.pair(Name(b"CapHeight"), cap_height);
        self
    }

    /// Write the `/StemV` attribute.
    pub fn stem_v(&mut self, stem_v: f32) -> &mut Self {
        self.dict.pair(Name(b"StemV"), stem_v);
        self
    }

    /// Write the `/FontFile2` attribute.
    pub fn font_file2(&mut self, true_type_stream: Ref) -> &mut Self {
        self.dict.pair(Name(b"FontFile2"), true_type_stream);
        self
    }
}

pub use flags::*;
mod flags {
    #![allow(missing_docs)]
    bitflags::bitflags! {
        /// Bitflags describing various characteristics of fonts.
        pub struct FontFlags: u32 {
            const FIXED_PITCH = 1 << 0;
            const SERIF = 1 << 1;
            const SYMBOLIC = 1 << 2;
            const SCRIPT = 1 << 3;
            const NON_SYMBOLIC = 1 << 5;
            const ITALIC = 1 << 6;
            const ALL_CAP = 1 << 16;
            const SMALL_CAP = 1 << 17;
            const FORCE_BOLD = 1 << 18;
        }
    }
}

/// Writer for a _character map_.
///
/// This struct is created by [`PdfWriter::cmap`].
pub struct CmapStream<'a> {
    stream: Stream<'a>,
}

impl<'a> CmapStream<'a> {
    pub(crate) fn start(mut stream: Stream<'a>) -> Self {
        stream.inner().pair(Name(b"Type"), Name(b"CMap"));
        Self { stream }
    }

    /// Write the `/CMapName` attribute.
    pub fn name(&mut self, name: Name) -> &mut Self {
        self.stream.inner().pair(Name(b"CMapName"), name);
        self
    }

    /// Write the `/CIDSystemInfo` attribute.
    pub fn system_info(&mut self, info: SystemInfo) -> &mut Self {
        info.write(self.stream.inner().key(Name(b"CIDSystemInfo")));
        self
    }

    /// Access the underlying stream writer.
    pub fn inner(&mut self) -> &mut Stream<'a> {
        &mut self.stream
    }
}

/// Builder for a `/ToUnicode` character map stream.
pub struct UnicodeCmap {
    buf: Vec<u8>,
    mappings: Vec<u8>,
    count: i32,
}

impl UnicodeCmap {
    /// Create a new, empty unicode character map.
    pub fn new(name: Name, info: SystemInfo) -> Self {
        // https://www.adobe.com/content/dam/acom/en/devnet/font/pdfs/5014.CIDFont_Spec.pdf

        let mut buf = Vec::new();

        // Static header.
        buf.push_bytes(b"%!PS-Adobe-3.0 Resource-CMap\n");
        buf.push_bytes(b"%%DocumentNeededResources: procset CIDInit\n");
        buf.push_bytes(b"%%IncludeResource: procset CIDInit\n");

        // Dynamic header.
        buf.push_bytes(b"%%BeginResource: CMap ");
        buf.push_bytes(name.0);
        buf.push(b'\n');
        buf.push_bytes(b"%%Title: (");
        buf.push_bytes(name.0);
        buf.push(b' ');
        buf.push_bytes(info.registry.0);
        buf.push(b' ');
        buf.push_bytes(info.ordering.0);
        buf.push(b' ');
        buf.push_int(info.supplement);
        buf.push_bytes(b")\n");
        buf.push_bytes(b"%%Version: 1\n");
        buf.push_bytes(b"%%EndComments\n");

        // General body.
        buf.push_bytes(b"/CIDInit /ProcSet findresource begin\n");
        buf.push_bytes(b"12 dict begin\n");
        buf.push_bytes(b"begincmap\n");
        buf.push_bytes(b"/CIDSystemInfo 3 dict dup begin\n");
        buf.push_bytes(b"    /Registry ");
        buf.push_val(info.registry);
        buf.push_bytes(b" def\n");
        buf.push_bytes(b"    /Ordering ");
        buf.push_val(info.ordering);
        buf.push_bytes(b" def\n");
        buf.push_bytes(b"    /Supplement ");
        buf.push_val(info.supplement);
        buf.push_bytes(b" def\n");
        buf.push_bytes(b"end def\n");
        buf.push_bytes(b"/CMapName ");
        buf.push_val(name);
        buf.push_bytes(b" def\n");
        buf.push_bytes(b"/CMapVersion 1 def\n");
        buf.push_bytes(b"/CMapType 0 def\n");

        // We just cover the whole unicode codespace.
        buf.push_bytes(b"1 begincodespacerange\n");
        buf.push_bytes(b"<0000> <ffff>\n");
        buf.push_bytes(b"endcodespacerange\n");

        Self { buf, mappings: vec![], count: 0 }
    }

    /// Add a mapping from a glyph ID to a unicode codepoint.
    pub fn pair(&mut self, glyph: u16, codepoint: char) {
        self.mappings.push(b'<');
        self.mappings.push_hex_u16(glyph);
        self.mappings.push_bytes(b"> <");

        let mut utf16 = [0u16; 2];
        for &mut part in codepoint.encode_utf16(&mut utf16) {
            self.mappings.push_hex_u16(part);
        }

        self.mappings.push_bytes(b">\n");
        self.count += 1;

        // At most 100 lines per range.
        if self.count >= 100 {
            self.flush_range();
        }
    }

    /// Finish building the character map.
    pub fn finish(mut self) -> Vec<u8> {
        // Flush the in-progress range.
        self.flush_range();

        // End of body.
        self.buf.push_bytes(b"endcmap\n");
        self.buf
            .push_bytes(b"CMapName currentdict /CMap defineresource pop\n");
        self.buf.push_bytes(b"end\n");
        self.buf.push_bytes(b"end\n");
        self.buf.push_bytes(b"%%EndResource\n");
        self.buf.push_bytes(b"%%EOF");

        self.buf
    }

    fn flush_range(&mut self) {
        if self.count > 0 {
            self.buf.push_int(self.count);
            self.buf.push_bytes(b" beginbfchar\n");
            self.buf.push_bytes(&self.mappings);
            self.buf.push_bytes(b"endbfchar\n");
        }

        self.count = 0;
        self.mappings.clear();
    }
}

/// Specifics about a character collection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SystemInfo<'a> {
    /// The issuer of the collection.
    pub registry: Str<'a>,
    /// A unique name of the collection within the registry.
    pub ordering: Str<'a>,
    /// The supplement number (i.e. the version).
    pub supplement: i32,
}

impl SystemInfo<'_> {
    fn write(&self, any: Any<'_>) {
        any.dict()
            .pair(Name(b"Registry"), self.registry)
            .pair(Name(b"Ordering"), self.ordering)
            .pair(Name(b"Supplement"), self.supplement);
    }
}
