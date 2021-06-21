use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_addr: u16,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let next_addr: u16 = 16;
        let mut table = HashMap::new();
        table.insert("R0".to_string(), 0);
        table.insert("R1".to_string(), 1);
        table.insert("R2".to_string(), 2);
        table.insert("R3".to_string(), 3);
        table.insert("R4".to_string(), 4);
        table.insert("R5".to_string(), 5);
        table.insert("R6".to_string(), 6);
        table.insert("R7".to_string(), 7);
        table.insert("R8".to_string(), 8);
        table.insert("R9".to_string(), 9);
        table.insert("R10".to_string(), 10);
        table.insert("R11".to_string(), 11);
        table.insert("R12".to_string(), 12);
        table.insert("R13".to_string(), 13);
        table.insert("R14".to_string(), 14);
        table.insert("R15".to_string(), 15);

        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);

        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24567);


        SymbolTable{
            table,
            next_addr,
        }
    }

    pub fn get(&mut self, k: String) -> u16 {
        let k = k.trim().to_string();
        match &self.table.get(&k) {
            Some(v) => **v,
            None => {
                let v = self.next_addr;
                &self.table.insert(k, v);
                self.next_addr += 1;
                v
            }
        }
    }

    pub fn insert(&mut self, k: String, v: u16) {
        let k = k.trim().to_string();
        &self.table.insert(k, v);
    }
}
