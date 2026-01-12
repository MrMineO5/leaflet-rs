pub struct PaletteContent<const L: usize> {
    pub bits_per_entry: usize,
    pub data: Vec<u64>
}

impl<const L: usize> PaletteContent<L> {
    pub fn new(bits_per_entry: usize) -> Self {
        if bits_per_entry == 0 {
            return Self { bits_per_entry, data: vec![] }
        }
        let entries_per_long = 64 / bits_per_entry;
        let total_entries = L * L * L;
        let long_count = (total_entries + entries_per_long - 1) / entries_per_long;
        Self { bits_per_entry, data: vec![0; long_count] }
    }

    pub fn get(&self, index: usize) -> usize {
        if self.bits_per_entry == 0 {
            return 0
        }
        
        let entry_mask = (1 << self.bits_per_entry) - 1;

        let entries_per_long = 64 / self.bits_per_entry;
        let l_index = index / entries_per_long;
        let r_index = index % entries_per_long;
        ((self.data[l_index] >> (r_index * self.bits_per_entry)) & entry_mask) as usize
    }

    pub fn set(&mut self, index: usize, value: usize) {
        if self.bits_per_entry == 0 {
            return
        }
        
        let entry_mask = (1 << self.bits_per_entry) - 1;

        let entries_per_long = 64 / self.bits_per_entry;
        let l_index = index / entries_per_long;
        let r_index = (index % entries_per_long) * self.bits_per_entry;
        self.data[l_index] &= !(entry_mask << r_index);
        self.data[l_index] |= (value as u64) << r_index;
    }
}
