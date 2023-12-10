use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3079,
        end_addr: 3082,
        reg_types: 4,
        model_number: 8,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU16(Point { name: "Power control word", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisI16(Point { name: "Limit power actual value", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Reserved", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisI16(Point { name: "Limiting reactive power adjustment value", offset: 3, length: 1, write_access: false, value: 0 } ));
    
    ret
}