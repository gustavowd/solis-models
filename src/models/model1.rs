use super::*;

pub fn model1() -> SolModel {
    let mut ret = SolModel {
        start_addr: 30000,
        end_addr: 30034,
        model_number: 1,
        qtd: 35,
        data: Vec::new(),
    };
    ret.data.push(SDataTypes::SolisU16(Point { name: "Model ID", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Number of PV strings", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU16(Point { name: "Number of MPPT trackers", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(SDataTypes::SolisU32(Point { name: "Rated Power", offset: 3, length: 2, write_access: false, value: 0 } ));

    ret
}
