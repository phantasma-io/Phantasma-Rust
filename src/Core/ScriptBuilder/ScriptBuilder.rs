use byteorder::{LittleEndian, WriteBytesExt};
use num_bigint::BigInt;
use std::collections::HashMap;
use std::io::{Cursor, Write};

pub struct Scriptbuilder {
    stream: Cursor<Vec<u8>>,
    jump_locations: HashMap<i32, String>,
    label_locations: HashMap<String, i32>,
}

pub impl Scriptbuilder {
    pub fn new() -> ScriptBuilder {
        ScriptBuilder {
            stream: Cursor::new(Vec::new()),
            jump_locations: HashMap::new(),
            label_locations: HashMap::new(),
        }
    }

    pub fn emit(&mut self, opcode: Opcode, bytes: Option<Vec<u8>>) -> &mut Self {
        self.stream.write_u8(opcode as u8).unwrap();

        if let Some(bytes) = bytes {
            self.stream.write_all(&bytes).unwrap();
        }

        self
    }
    pub fn emit_load_string(&mut self, reg: u8, val: &str) -> &mut Self { /* ... */
    }
    pub fn emit_load_i64(&mut self, reg: u8, val: i64) -> &mut Self { /* ... */
    }
    pub fn emit_load_bool(&mut self, reg: u8, val: bool) -> &mut Self { /* ... */
    }
    // ... other emit_load_* methods ...

    pub fn emit_move(&mut self, src_reg: u8, dst_reg: u8) -> &mut Self {
        self.emit(Opcode::MOVE, Some(vec![src_reg, dst_reg]))
    }

    pub fn emit_copy(&mut self, src_reg: u8, dst_reg: u8) -> &mut Self {
        self.emit(Opcode::COPY, Some(vec![src_reg, dst_reg]))
    }

    pub fn emit_label(&mut self, label: &str) -> &mut Self {
        self.emit(Opcode::NOP, None);
        let position = self.stream.position();
        self.label_locations
            .insert(label.to_string(), position as i32);
        self
    }

    pub fn to_script(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut script = self.stream.get_ref().clone();

        // resolve jump offsets
        for (target_offset, label) in &self.jump_locations {
            if let Some(&label_offset) = self.label_locations.get(label) {
                let bytes = (label_offset as u16).to_le_bytes();
                script[*target_offset as usize..*target_offset as usize + 2]
                    .copy_from_slice(&bytes);
            } else {
                return Err(format!("Could not find label: {}", label).into());
            }
        }

        Ok(script)
    }
}
