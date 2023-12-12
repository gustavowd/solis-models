use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3052,
        end_addr: 3053,
        reg_types: 4,
        model_number: 8,
        qtd: 2,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisI16(Point { name: "PF Setting", offset: 0, length: 1, write_access: true, value: 0 } ));
    ret.data.push(SDataTypes::SolisI16(Point { name: "PF Setting 02", offset: 1, length: 1, write_access: true, value: 0 } ));

    ret
}
