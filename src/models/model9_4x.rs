use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3069,
        end_addr: 3071,
        reg_types: 4,
        model_number: 9,
        qtd: 3,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU16(Point { name: "Power limitation switch", offset: 0, length: 1, write_access: true, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Reactive power switch", offset: 1, length: 1, write_access: true, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Working mode", offset: 2, length: 1, write_access: true, value: 0 } ));

    ret
}