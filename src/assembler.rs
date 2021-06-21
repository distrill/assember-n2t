use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

use crate::parser::Line;
use crate::symbol_table::SymbolTable;

#[derive(Debug)]
pub struct Assembler {
  src: Vec<String>,
  bin: Vec<String>,
  table: SymbolTable,
}

impl Assembler {
    pub fn new(filename: &impl AsRef<Path>) -> Result<Assembler> {
	let file = File::open(filename)?;
	let buf = BufReader::new(file);
	let src = buf.lines()
	    .map(|l| l.expect("Could not parse line"))
	    .collect();

        let bin = Vec::new();
        let table = SymbolTable::new();

        Ok(Assembler{ src, bin, table })
    }

    pub fn process(&mut self) -> Result<()> {
        // first pass - swap out the symbols
        let mut ln = 0;
        for l in &self.src {
            let line = Line::new(l, &mut SymbolTable::new())?;
            match line {
                Line::Label(label) => {
                    &self.table.insert(label.clone(), ln);
                },
                Line::A{..} | Line::C{..} => {
                    ln += 1;
                }
                _ => {},
            };
        }
        
        // second pass - generate the binary
        for l in &self.src {
            let line = Line::new(l, &mut self.table)?;
            match line {
                Line::A{..} | Line::C{..} => {
                    self.bin.push(line.to_bin()?);
                },
                _ => {},
            };

        }
        Ok(())
    }

    pub fn write_bin(&self, binname: &String) -> Result<()> {
        let mut buf = "".to_string();
        for binline in &self.bin {
            buf.push_str(binline);
            buf.push('\n');
        }

        fs::write(binname, buf)?;
    
        Ok(())
    }
}
