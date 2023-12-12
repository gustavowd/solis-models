use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3051,
        end_addr: 3051,
        reg_types: 4,
        model_number: 7,
        qtd: 1,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU16(Point { name: "Power limitation", offset: 0, length: 1, write_access: true, value: 0 } ));

    ret
}
