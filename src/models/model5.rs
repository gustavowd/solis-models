use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3060,
        end_addr: 3063,
        reg_types: 3,
        model_number: 5,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisString(Point { name: "Inverter SN_1", offset: 0, length: 1, write_access: false, value: String::new() } ));
    ret.data.push(SDataTypes::SolisString(Point { name: "Inverter SN_2", offset: 1, length: 1, write_access: false, value: String::new() } ));
    ret.data.push(SDataTypes::SolisString(Point { name: "Inverter SN_3", offset: 2, length: 1, write_access: false, value: String::new() } ));
    ret.data.push(SDataTypes::SolisString(Point { name: "Inverter SN_4", offset: 3, length: 1, write_access: false, value: String::new() } ));

    ret
}
