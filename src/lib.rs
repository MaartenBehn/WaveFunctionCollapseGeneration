
type CellID = usize;
type StateTableUnit = usize;
type StateTable = Vec<StateTableUnit>;

struct StateTableBuilder{
    table: StateTable,
    max_cell_id: usize,
    max_neigbors: usize,
    bits_per_unit: usize,
    units_per_neigbor_range: usize,
}

struct Cell<'a>{
    id: CellID,
    neigbors: Vec<& 'a Cell<'a>>,
}

struct GenerationData {
    
}

impl StateTableBuilder {
    pub fn builder (max_cell_id: usize, max_neigbors: usize) -> Self {

        let bits_per_unit = usize::BITS as usize;
        let units_per_neigbor_range = if max_cell_id % bits_per_unit != 0 {  max_cell_id / bits_per_unit + 1 } else { max_cell_id / bits_per_unit };

        let size = max_cell_id * max_neigbors * units_per_neigbor_range;
        let unit_size = if size % bits_per_unit != 0 { size / bits_per_unit + 1 } else { size / bits_per_unit };

        StateTableBuilder { 
            table: vec![0; unit_size],
            max_cell_id,
            max_neigbors,
            bits_per_unit,
            units_per_neigbor_range
        }
    }

    fn get_indecies(&self, cell_id: CellID, neigbor: usize, neigbor_id: CellID ) -> (usize, usize) { 
        let index = cell_id *  self.max_neigbors * self.units_per_neigbor_range + neigbor * self.units_per_neigbor_range + neigbor_id;
        let unit_index = index / self.bits_per_unit;
        let in_unit_index = index % self.bits_per_unit;

        (unit_index, in_unit_index)
    }

    pub fn set_possible_neigbor (mut self, cell_id: CellID, neigbor: usize, neigbor_id: CellID, is_neigbor: bool) -> Self {
        if is_neigbor{
            return self.add_possible_neigbor(cell_id, neigbor, neigbor_id);
        }
        else{
            return self.remove_possible_neigbor(cell_id, neigbor, neigbor_id);
        }
    }

    pub fn add_possible_neigbor (mut self, cell_id: CellID, neigbor: usize, neigbor_id: CellID) -> Self {
        let (unit_index, in_unit_index) = self.get_indecies(cell_id, neigbor, neigbor_id);
        self.table[unit_index] |= 1 << in_unit_index;

        self
    }

    pub fn remove_possible_neigbor (mut self, cell_id: CellID, neigbor: usize, neigbor_id: CellID) -> Self {
        let (unit_index, in_unit_index) = self.get_indecies(cell_id, neigbor, neigbor_id);
        self.table[unit_index] &= !(1 << in_unit_index);

        self
    }

    pub fn unwarp (mut self) -> StateTable {
        self.table
    }
}



#[cfg(test)]
mod test {
    use crate::StateTableBuilder;

    #[test]
    fn test_state_table_builder() {
        let table = StateTableBuilder::builder(10, 4)
            .add_possible_neigbor(0, 0, 1)
            .add_possible_neigbor(1, 2, 5)
            .unwarp();

    }
}



