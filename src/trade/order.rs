use rust_decimal::Decimal;
use strum::Display;

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum NewOrderResponseType {
    Ack,
    Result,
    Full,
}

#[derive(Copy, Clone, Display)]
pub enum CancelReplaceMode {
    #[strum(serialize = "STOP_ON_FAILURE")]
    StopOnFailure,
    #[strum(serialize = "ALLOW_FAILURE")]
    AllowFailure,
}

pub struct WorkingMandatoryParams {
    pub working_type: String,
    pub working_side: Side,
    pub working_price: Decimal,
    pub working_quantity: Decimal,
}

impl WorkingMandatoryParams {
    pub fn new(
        working_type: &str,
        working_side: Side,
        working_price: Decimal,
        working_quantity: Decimal,
    ) -> Self {
        Self {
            working_type: working_type.to_owned(),
            working_side,
            working_price,
            working_quantity,
        }
    }
}
