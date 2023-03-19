use lazy_static::lazy_static;
use std::{collections::HashMap};
use rust_decimal::Decimal;

// PROGRAM CONSTANTS
pub const NUM_ROW_ERRORS_DISP: usize = 10;

// DATE CONSTANTS
pub const QTRS_IN_YR: u32 = 4;
pub const MONS_IN_QTR: u32 = 3;
pub const QTR_OFFSET: u32 = 1;
pub const FISCAL_YR_START_MON: u32 = 7;
pub const DT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

// SPC CONSTANTS
#[derive(Debug, Clone)]
pub enum SpcConst {
    D2,
    C4,
    D3,
    D4,
}

#[derive(Debug, Clone)]
pub struct SpcConstTblRow {
    d2: &'static str,
    c4: &'static str,
    d3: &'static str,
    d4: &'static str,
}


impl SpcConstTblRow {
    pub fn new(d2: &'static str, c4: &'static str, d3: &'static str, d4: &'static str) -> SpcConstTblRow {
        SpcConstTblRow {
            d2,
            c4,
            d3,
            d4,
        }
    }

    pub fn get(self: &Self, const_name: SpcConst) -> &str {
        match const_name {
            SpcConst::D2 => self.d2,
            SpcConst::C4 => self.c4,
            SpcConst::D3 => self.d3,
            SpcConst::D4 => self.d4,
        }
    }

    pub fn dec(self: &Self, const_name: SpcConst) -> Decimal {
        let const_str = self.get(const_name);
        Decimal::from_str_exact(const_str).unwrap()
    }
}

lazy_static!{
    pub static ref SPC_CONST_TBL: HashMap<i8, SpcConstTblRow> = {
        let mut hmap = HashMap::new();
        hmap.insert(2, SpcConstTblRow::new("1.1284", "0.7979", "0.8525", "0.9539"));
        hmap.insert(3, SpcConstTblRow::new("1.6926", "0.8862", "0.8884", "1.5878"));
        hmap.insert(4, SpcConstTblRow::new("2.0588", "0.9213", "0.8798", "1.9783"));
        hmap.insert(5, SpcConstTblRow::new("2.3259", "0.9400", "0.8641", "2.2569"));
        hmap.insert(6, SpcConstTblRow::new("2.5344", "0.9515", "0.8480", "2.4717")); 
        hmap.insert(7, SpcConstTblRow::new("2.7044", "0.9594", "0.8332", "2.6455"));
        hmap.insert(8, SpcConstTblRow::new("2.8472", "0.9650", "0.8198", "2.7908"));
        hmap.insert(9, SpcConstTblRow::new("2.9700", "0.9693", "0.8078", "2.9154"));
        hmap.insert(10, SpcConstTblRow::new("3.0775", "0.9727", "0.7971", "3.0242"));
        hmap.insert(11, SpcConstTblRow::new("3.1729", "0.9754", "0.7873", "3.1205")); 
        hmap.insert(12, SpcConstTblRow::new("3.2585", "0.9776", "0.7785", "3.2069"));
        hmap.insert(13, SpcConstTblRow::new("3.3360", "0.9794", "0.7704", "3.2849"));
        hmap.insert(14, SpcConstTblRow::new("3.4068", "0.9810", "0.7630", "3.3562"));
        hmap.insert(15, SpcConstTblRow::new("3.4718", "0.9823", "0.7562", "3.4217"));
        hmap.insert(16, SpcConstTblRow::new("3.5320", "0.9835", "0.7499", "3.4821")); 
        hmap.insert(17, SpcConstTblRow::new("3.5879", "0.9845", "0.7441", "3.5383"));
        hmap.insert(18, SpcConstTblRow::new("3.6401", "0.9854", "0.7386", "3.5907"));
        hmap.insert(19, SpcConstTblRow::new("3.6890", "0.9862", "0.7335", "3.6398"));
        hmap.insert(20, SpcConstTblRow::new("3.7349", "0.9869", "0.7287", "3.6859"));
        hmap.insert(21, SpcConstTblRow::new("3.7783", "0.9876", "0.7242", "3.7294")); 
        hmap.insert(22, SpcConstTblRow::new("3.8194", "0.9882", "0.7199", "3.7706"));
        hmap.insert(23, SpcConstTblRow::new("3.8583", "0.9887", "0.7159", "3.8096"));
        hmap.insert(24, SpcConstTblRow::new("3.8953", "0.9892", "0.7121", "3.8468"));
        hmap.insert(25, SpcConstTblRow::new("3.9306", "0.9896", "0.7084", "3.8822"));
        hmap.insert(26, SpcConstTblRow::new("3.9643", "0.9901", "0.7050", "3.9159"));
        hmap.insert(27, SpcConstTblRow::new("3.9965", "0.9904", "0.7017", "3.9482"));
        hmap.insert(28, SpcConstTblRow::new("4.0274", "0.9908", "0.6986", "3.9791"));
        hmap.insert(29, SpcConstTblRow::new("4.0570", "0.9911", "0.6955", "4.0088"));
        hmap.insert(30, SpcConstTblRow::new("4.0855", "0.9914", "0.6927", "4.0374"));
        hmap
    };

    pub static ref SPC_MR_CONST: Decimal = SPC_CONST_TBL.get(&2).unwrap().dec(SpcConst::D2);
}