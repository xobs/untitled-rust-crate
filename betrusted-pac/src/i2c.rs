#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bits 8-15 of `I2C_PRESCALE`. Prescaler value. Set to (module clock / (5 * I2C freq) - 1). Example: if module clock is equal to sysclk; syclk is 100MHz; and I2C freq is 100kHz, then prescaler is (100MHz / (5 * 100kHz) - 1) = 199. Reset value: 0xFFFF"]
    pub prescale1: PRESCALE1,
    #[doc = "0x04 - Bits 0-7 of `I2C_PRESCALE`."]
    pub prescale0: PRESCALE0,
    #[doc = "0x08 - "]
    pub control: CONTROL,
    #[doc = "0x0c - Next byte to transmit to slave devices. LSB indicates R/W during address phases, `1` for reading from slaves, `0` for writing to slaves"]
    pub txr: TXR,
    #[doc = "0x10 - Data being read from slaved devices"]
    pub rxr: RXR,
    #[doc = "0x14 - "]
    pub command: COMMAND,
    #[doc = "0x18 - "]
    pub status: STATUS,
    #[doc = "0x1c - "]
    pub ev_status: EV_STATUS,
    #[doc = "0x20 - "]
    pub ev_pending: EV_PENDING,
    #[doc = "0x24 - "]
    pub ev_enable: EV_ENABLE,
}
#[doc = "Bits 8-15 of `I2C_PRESCALE`. Prescaler value. Set to (module clock / (5 * I2C freq) - 1). Example: if module clock is equal to sysclk; syclk is 100MHz; and I2C freq is 100kHz, then prescaler is (100MHz / (5 * 100kHz) - 1) = 199. Reset value: 0xFFFF\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prescale1](prescale1) module"]
pub type PRESCALE1 = crate::Reg<u32, _PRESCALE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESCALE1;
#[doc = "`read()` method returns [prescale1::R](prescale1::R) reader structure"]
impl crate::Readable for PRESCALE1 {}
#[doc = "`write(|w| ..)` method takes [prescale1::W](prescale1::W) writer structure"]
impl crate::Writable for PRESCALE1 {}
#[doc = "Bits 8-15 of `I2C_PRESCALE`. Prescaler value. Set to (module clock / (5 * I2C freq) - 1). Example: if module clock is equal to sysclk; syclk is 100MHz; and I2C freq is 100kHz, then prescaler is (100MHz / (5 * 100kHz) - 1) = 199. Reset value: 0xFFFF"]
pub mod prescale1;
#[doc = "Bits 0-7 of `I2C_PRESCALE`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prescale0](prescale0) module"]
pub type PRESCALE0 = crate::Reg<u32, _PRESCALE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESCALE0;
#[doc = "`read()` method returns [prescale0::R](prescale0::R) reader structure"]
impl crate::Readable for PRESCALE0 {}
#[doc = "`write(|w| ..)` method takes [prescale0::W](prescale0::W) writer structure"]
impl crate::Writable for PRESCALE0 {}
#[doc = "Bits 0-7 of `I2C_PRESCALE`."]
pub mod prescale0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = ""]
pub mod control;
#[doc = "Next byte to transmit to slave devices. LSB indicates R/W during address phases, `1` for reading from slaves, `0` for writing to slaves\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txr](txr) module"]
pub type TXR = crate::Reg<u32, _TXR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXR;
#[doc = "`read()` method returns [txr::R](txr::R) reader structure"]
impl crate::Readable for TXR {}
#[doc = "`write(|w| ..)` method takes [txr::W](txr::W) writer structure"]
impl crate::Writable for TXR {}
#[doc = "Next byte to transmit to slave devices. LSB indicates R/W during address phases, `1` for reading from slaves, `0` for writing to slaves"]
pub mod txr;
#[doc = "Data being read from slaved devices\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxr](rxr) module"]
pub type RXR = crate::Reg<u32, _RXR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXR;
#[doc = "`read()` method returns [rxr::R](rxr::R) reader structure"]
impl crate::Readable for RXR {}
#[doc = "`write(|w| ..)` method takes [rxr::W](rxr::W) writer structure"]
impl crate::Writable for RXR {}
#[doc = "Data being read from slaved devices"]
pub mod rxr;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [command](command) module"]
pub type COMMAND = crate::Reg<u32, _COMMAND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMMAND;
#[doc = "`read()` method returns [command::R](command::R) reader structure"]
impl crate::Readable for COMMAND {}
#[doc = "`write(|w| ..)` method takes [command::W](command::W) writer structure"]
impl crate::Writable for COMMAND {}
#[doc = ""]
pub mod command;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = ""]
pub mod status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ev_status](ev_status) module"]
pub type EV_STATUS = crate::Reg<u32, _EV_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_STATUS;
#[doc = "`read()` method returns [ev_status::R](ev_status::R) reader structure"]
impl crate::Readable for EV_STATUS {}
#[doc = "`write(|w| ..)` method takes [ev_status::W](ev_status::W) writer structure"]
impl crate::Writable for EV_STATUS {}
#[doc = ""]
pub mod ev_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ev_pending](ev_pending) module"]
pub type EV_PENDING = crate::Reg<u32, _EV_PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_PENDING;
#[doc = "`read()` method returns [ev_pending::R](ev_pending::R) reader structure"]
impl crate::Readable for EV_PENDING {}
#[doc = "`write(|w| ..)` method takes [ev_pending::W](ev_pending::W) writer structure"]
impl crate::Writable for EV_PENDING {}
#[doc = ""]
pub mod ev_pending;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ev_enable](ev_enable) module"]
pub type EV_ENABLE = crate::Reg<u32, _EV_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_ENABLE;
#[doc = "`read()` method returns [ev_enable::R](ev_enable::R) reader structure"]
impl crate::Readable for EV_ENABLE {}
#[doc = "`write(|w| ..)` method takes [ev_enable::W](ev_enable::W) writer structure"]
impl crate::Writable for EV_ENABLE {}
#[doc = ""]
pub mod ev_enable;
