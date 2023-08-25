use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3055,
        end_addr: 3058,
        model_number: 1,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU32(Point { name: "Reactive power", offset: 0, length: 2, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Apparent power", offset: 2, length: 2, write_access: false, value: 0 } ));

    ret
}
