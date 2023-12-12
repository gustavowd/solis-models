use super::*;

pub fn model() -> SolModel {
    let mut ret = SolModel {
        start_addr: 3050,
        end_addr: 3050,
        reg_types: 4,
        model_number: 6,
        qtd: 1,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisI16(Point { name: "Reactive power limitation", offset: 0, length: 1, write_access: true, value: 0 } ));

    ret
}