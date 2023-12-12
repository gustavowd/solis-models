use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3080,
        end_addr: 3080,
        reg_types: 4,
        model_number: 10,
        qtd: 1,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisI16(Point { name: "Limit power actual value", offset: 0, length: 1, write_access: false, value: 0 } ));
    
    ret
}
