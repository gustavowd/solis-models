use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3050,
        end_addr: 3052,
        reg_types: 4,
        model_number: 6,
        qtd: 3,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisI16(Point { name: "Reactive Power Limitation", offset: 0, length: 1, write_access: true, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Power Limitation", offset: 1, length: 1, write_access: true, value: 0 } ));
    ret.data.push(SDataTypes::SolisI16(Point { name: "PF Setting", offset: 2, length: 1, write_access: true, value: 0 } ));

    ret
}