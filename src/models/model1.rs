use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 2999,
        end_addr: 3003,
        model_number: 1,
        qtd: 5,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU16(Point { name: "Product Model", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DSP software version", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "LCD software version", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AC output type", offset: 3, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC output type", offset: 4, length: 1, write_access: false, value: 0 } ));

    ret
}
