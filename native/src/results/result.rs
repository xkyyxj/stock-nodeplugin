pub trait Result {
    //fn new() -> Self;

    fn to_string(&self) -> Option<String>;
}

pub struct InLow {
    pk_low: i32,
    ts_code: Option<String>,
    ts_name: Option<String>,
    date: Option<String>
}

pub struct StockBaseInfo {
    trade_date: Option<String>,
    ts_code: Option<String>,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    vol: f64,
    amount: f64,
    pre_close: f64,
    change: f64,
    pct_chg: f64
}

impl Result for InLow {
    fn to_string(&self) -> Option<String> {
        unimplemented!()
    }
}

