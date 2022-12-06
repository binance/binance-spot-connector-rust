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
