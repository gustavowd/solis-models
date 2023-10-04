use crate::types::*;

pub mod model1;
pub mod model2;
pub mod model4;
pub mod model5;

#[derive(Debug, Clone)]
pub struct SolModel {
    pub start_addr: u16,
    pub end_addr: u16,
    pub model_number: u16,
    pub qtd: u16,
    pub data: Vec<SDataTypes>,
}

#[derive(Debug, Clone)]
pub struct SolModels {
    pub models: Vec<SolModel>,
}

// Declare the struct
pub trait SolisModels {
    // This new function acts as a constructor
    fn new (model_number: u16) -> Self;
    fn update_data(&mut self, point: &str, value: &SDataTypes);
    fn update_data_by_index(&mut self, index: usize, value: &SDataTypes);
    fn get_data(&self, point: &str) -> SDataTypes;
    fn get_data_index(&self, point: &str) -> Option<usize>;
    fn get_u16(&self, point: &str) -> Option<u16>;
    fn get_u16_by_index(&self, idx: usize) -> Option<u16>;
    fn get_u32(&self, point: &str) -> Option<u32>;
    fn get_u32_by_index(&self, idx: usize) -> Option<u32>;
    fn get_i16(&self, point: &str) -> Option<i16>;
    fn get_i16_by_index(&self, idx: usize) -> Option<i16>;
    fn get_i32(&self, point: &str) -> Option<i32>;
    fn get_i32_by_index(&self, idx: usize) -> Option<i32>;
    fn print(&self);
}

impl Default for SolModels {
    fn default() -> Self {
        Self::new()
    }
}

impl SolModels {
    pub fn new () -> SolModels {
        SolModels { models: Vec::new() }
    }

    pub fn get_model_index(&self, model_number: u16) -> Option<usize> {
        for (idx, model) in self.models.iter().enumerate() {
            if model_number == model.model_number {
                return Some(idx);
            }
        }
        None
    }
}

impl SolisModels for SolModel {
    fn new (model_number: u16) -> SolModel {
        match model_number {
            1 => model1::model(),
            2 => model2::model(),
            4 => model4::model(),
            5 => model5::model(),
            _ => model1::model(),
        }
    }

    fn update_data(&mut self, point: &str, value: &SDataTypes) {
        for data_tmp in self.data.iter_mut() {
            match data_tmp {
                SDataTypes::SolisU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let SDataTypes::SolisU16(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                SDataTypes::SolisU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let SDataTypes::SolisU32(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                SDataTypes::SolisI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let SDataTypes::SolisI16(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                SDataTypes::SolisI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let SDataTypes::SolisI32(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                SDataTypes::SolisString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let SDataTypes::SolisString(update_value) = value {
                            data.value = update_value.value.clone();
                        }
                    }
                }
            }
        }
    }

    fn update_data_by_index(&mut self, index: usize, value: &SDataTypes) {
        match &mut self.data[index] {
            SDataTypes::SolisU16(data) => {
                if let SDataTypes::SolisU16(update_value) = value {
                    data.value = update_value.value;
                }
            },
            SDataTypes::SolisU32(data) => {
                if let SDataTypes::SolisU32(update_value) = value {
                    data.value = update_value.value;
                }
            },
            SDataTypes::SolisI16(data) => {
                if let SDataTypes::SolisI16(update_value) = value {
                    data.value = update_value.value;
                }
            },
            SDataTypes::SolisI32(data) => {
                if let SDataTypes::SolisI32(update_value) = value {
                    data.value = update_value.value;
                }
            },
            SDataTypes::SolisString(data) => {
                if let SDataTypes::SolisString(update_value) = value {
                    data.value = update_value.value.clone();
                }
            }
        }
    }

    fn get_data(&self, point: &str) -> SDataTypes {
        for data_tmp in self.data.iter() {
            match data_tmp {
                SDataTypes::SolisU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                SDataTypes::SolisU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                SDataTypes::SolisI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                SDataTypes::SolisI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                SDataTypes::SolisString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                }
            };
        }
        SDataTypes::SolisU16(Point { name: "", offset: 0, length: 1, write_access: false, value: 0 } )
    }

    fn get_data_index(&self, point: &str) -> Option<usize> {
        for (idx, data_tmp) in self.data.iter().enumerate() {
            match data_tmp {
                SDataTypes::SolisU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                SDataTypes::SolisU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                SDataTypes::SolisI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                SDataTypes::SolisI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                SDataTypes::SolisString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                }
            };
        }
        None
    }
    
    fn get_u16(&self, point: &str) -> Option<u16> {
        for data_tmp in self.data.iter() {
            if let SDataTypes::SolisU16(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_u16_by_index(&self, idx: usize) -> Option<u16> {
        match self.data[idx] {
            SDataTypes::SolisU16(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_u32(&self, point: &str) -> Option<u32> {
        for data_tmp in self.data.iter() {
            if let SDataTypes::SolisU32(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_u32_by_index(&self, idx: usize) -> Option<u32> {
        match self.data[idx] {
            SDataTypes::SolisU32(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_i16(&self, point: &str) -> Option<i16> {
        for data_tmp in self.data.iter() {
            if let SDataTypes::SolisI16(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_i16_by_index(&self, idx: usize) -> Option<i16> {
        match self.data[idx] {
            SDataTypes::SolisI16(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_i32(&self, point: &str) -> Option<i32> {
        for data_tmp in self.data.iter() {
            if let SDataTypes::SolisI32(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_i32_by_index(&self, idx: usize) -> Option<i32> {
        match self.data[idx] {
            SDataTypes::SolisI32(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn print(&self) {
        println!("Model {}:", self.model_number);
        for data in self.data.iter() {
            match data {
                SDataTypes::SolisI16(data) => println!("{}: {}", data.name, data.value),
                SDataTypes::SolisI32(data) => println!("{}: {}", data.name, data.value),
                SDataTypes::SolisU16(data) => println!("{}: {}", data.name, data.value),
                SDataTypes::SolisU32(data) => println!("{}: {}", data.name, data.value),
                SDataTypes::SolisString(data) => println!("{}: {}", data.name, data.value.clone()),
            }
        }
        println!(" ");
    }
}

impl From<SolModel> for Vec<u16> {
    fn from(from: SolModel) -> Self {
        let mut registers: Vec<u16> = Vec::new();

        for data in from.data.iter() {
            match data {
                SDataTypes::SolisU16(data) => registers.extend(u16::encode(data.value)),
                SDataTypes::SolisU32(data) => registers.extend(u32::encode(data.value)),
                SDataTypes::SolisI16(data) => registers.extend(i16::encode(data.value)),
                SDataTypes::SolisI32(data) => registers.extend(i32::encode(data.value)),
                SDataTypes::SolisString(data) => registers.extend(Point::<String>::encode(data.clone())),
            }
        }
        registers
    }
}

impl From<(Vec<u16>, u16, u16, &SolModel)> for SolModel {
    fn from(from: (Vec<u16>, u16, u16, &SolModel)) -> Self {
        let mut model1 = from.3.clone();
        let mut offset = from.1;
        let mut qtd = from.2;

        while qtd > 0 {
            for data in model1.data.iter_mut() {
                match data {
                    SDataTypes::SolisU16(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u16::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    SDataTypes::SolisU32(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u32::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    SDataTypes::SolisI16(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = i16::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    SDataTypes::SolisI32(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = i32::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    SDataTypes::SolisString(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            let hex_string = format!("{:4X}", slice[0]);
                            data.value = hex_string.chars().rev().collect();
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                }
            }
        }
        model1
    }
}
