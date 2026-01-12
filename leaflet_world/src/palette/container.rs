use crate::palette::content::PaletteContent;
use crate::palette::palette::PaletteType;

pub struct PalettedContainer<const L: usize> {
    pub palette: PaletteType,
    pub content: PaletteContent<L>
}

impl <const L: usize> PalettedContainer<L> {
    pub fn get(&self, index: usize) -> i32 {
        self.palette.get_id(self.content.get(index))
    }

    pub fn set(&mut self, index: usize, value: i32) {
        let p = self.palette.get_index(value);
        if let Some(value) = p {
            self.content.set(index, value);
        } else {
            self.grow();
            self.set(index, value);
        }
    }


    fn grow(&mut self) {
        let new_palette = self.palette.grow(self.palette.bpe(7) + 1, 7);
        let mut new_content: PaletteContent<L> = PaletteContent::new(new_palette.bpe(7) as usize);

        match &new_palette {
            PaletteType::Indirect(_, _p) => {
                // Semantics of palette grow for indirect palettes guarantee we do not need to remap indices
                for i in 0..L * L * L {
                    new_content.set(i, self.content.get(i));
                }
            }
            PaletteType::Direct => {
                // Remap to the raw ids
                for i in 0..L * L * L {
                    new_content.set(i, self.palette.get_id(self.content.get(i)) as usize)
                }
            }
            PaletteType::SingleValue(_) => unreachable!()
        }

        self.palette = new_palette;
        self.content = new_content;
    }
}
