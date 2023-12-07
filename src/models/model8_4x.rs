use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3082,
        end_addr: 3082,
        reg_types: 4,
        model_number: 8,
        qtd: 1,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU16(Point { name: "Limiting reactive power adjustment value", offset: 0, length: 1, write_access: false, value: 0 } ));
    
    ret
}
