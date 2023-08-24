use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3004,
        end_addr: 3043,
        model_number: 2,
        qtd: 40,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU32(Point { name: "Product Model", offset: 0, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "DSP software version", offset: 2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "LCD software version", offset: 4, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "AC output type", offset: 6, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "DC output type", offset: 8, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AC output type", offset: 10, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC output type", offset: 11, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "AC output type", offset: 12, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "DC output type", offset: 14, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Product Model", offset: 16, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DSP software version", offset: 17, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "LCD software version", offset: 18, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AC output type", offset: 19, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC output type", offset: 20, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Product Model", offset: 21, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DSP software version", offset: 22, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "LCD software version", offset: 23, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AC output type", offset: 24, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC output type", offset: 25, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Product Model", offset: 26, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DSP software version", offset: 27, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "LCD software version", offset: 28, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AC output type", offset: 29, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC output type", offset: 30, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Product Model", offset: 31, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DSP software version", offset: 32, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "LCD software version", offset: 33, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AC output type", offset: 34, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC output type", offset: 35, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Product Model", offset: 36, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DSP software version", offset: 37, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "LCD software version", offset: 38, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AC output type", offset: 39, length: 1, write_access: false, value: 0 } ));

    ret
}
