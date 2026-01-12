#[derive(Debug, Clone)]
pub struct IndirectPalette {
    palette: Vec<i32>
}

#[derive(Debug)]
pub enum PaletteType {
    SingleValue(i32),
    Indirect(u8, IndirectPalette),
    Direct
}

impl Default for PaletteType {
    fn default() -> Self {
        Self::SingleValue(0)
    }
}

impl PaletteType {
    pub fn get_id(&self, index: usize) -> i32 {
        match self {
            PaletteType::SingleValue(v) => { // index should always be 0, but we don't check this
                *v
            },
            PaletteType::Indirect(_, p) => p.palette[index],
            PaletteType::Direct => index as i32
        }
    }

    pub fn get_index(&self, id: i32) -> Option<usize> {
        match self {
            PaletteType::SingleValue(v) => {
                if *v == id {
                    return Some(0)
                }
                None
            },
            PaletteType::Indirect(_, p) => {
                for (i, v) in p.palette.iter().enumerate() {
                    if *v == id {
                        return Some(i)
                    }
                }
                None
            }
            PaletteType::Direct => Some(id as usize)
        }
    }

    pub fn bpe(&self, max_bpe: u8) -> u8 {
        match self {
            PaletteType::SingleValue(_) => 0,
            PaletteType::Indirect(bpe, _) => *bpe,
            PaletteType::Direct => max_bpe
        }
    }

    pub fn grow(&self, bpe: u8, max_bpe: u8) -> Self {
        if self.bpe(max_bpe) <= bpe {
            panic!("Tried resizing palette to smaller size than current size")
        }

        if bpe >= max_bpe { // Remap to direct palette
            PaletteType::Direct
        } else {
            let indirect_bpe = if bpe < 4 {
                4
            } else {
                bpe
            };

            let mut initial_palette = Vec::with_capacity(1 << indirect_bpe);
            match self {
                PaletteType::SingleValue(t) => initial_palette.push(*t),
                PaletteType::Indirect(_, p) => initial_palette.extend_from_slice(&p.palette),
                PaletteType::Direct => unreachable!()
            }

            PaletteType::Indirect(indirect_bpe, IndirectPalette { palette: initial_palette })
        }
    }
}
