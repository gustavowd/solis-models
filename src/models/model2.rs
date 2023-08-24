use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3004,
        end_addr: 3043,
        model_number: 2,
        qtd: 40,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU32(Point { name: "Active power", offset: 0, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Total DC output  power", offset: 2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Total Energy", offset: 4, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Energy this month", offset: 6, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Energy last month", offset: 8, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Energy today", offset: 10, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Energy last day", offset: 11, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Energy this year", offset: 12, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Energy last year", offset: 14, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "HMI version", offset: 16, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC voltage 1", offset: 17, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC current 1", offset: 18, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC voltage 2", offset: 19, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC current 2", offset: 20, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC voltage 3", offset: 21, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC current 3", offset: 22, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC voltage 4", offset: 23, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC current 4", offset: 24, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Alarm code data", offset: 25, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Initialize ground voltage", offset: 26, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC busbar voltage", offset: 27, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "DC half-busbar voltage", offset: 28, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "AB line voltage", offset: 29, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "BC line voltage", offset: 30, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "CA line voltage", offset: 31, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "A phase current", offset: 32, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "B phase current", offset: 33, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "C phase current", offset: 34, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Master/slave DSP upgrade switch", offset: 35, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Working mode", offset: 36, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Inverter temperature", offset: 37, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Grid frequency", offset: 38, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Inverter status", offset: 39, length: 1, write_access: false, value: 0 } ));

    ret
}
