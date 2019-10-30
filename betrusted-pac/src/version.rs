#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub major: MAJOR,
    #[doc = "0x04 - "]
    pub minor: MINOR,
    #[doc = "0x08 - "]
    pub revision: REVISION,
    #[doc = "0x0c - Bits 24-31 of `VERSION_GITREV`."]
    pub gitrev3: GITREV3,
    #[doc = "0x10 - Bits 16-23 of `VERSION_GITREV`."]
    pub gitrev2: GITREV2,
    #[doc = "0x14 - Bits 8-15 of `VERSION_GITREV`."]
    pub gitrev1: GITREV1,
    #[doc = "0x18 - Bits 0-7 of `VERSION_GITREV`."]
    pub gitrev0: GITREV0,
    #[doc = "0x1c - Bits 8-9 of `VERSION_GITEXTRA`."]
    pub gitextra1: GITEXTRA1,
    #[doc = "0x20 - Bits 0-7 of `VERSION_GITEXTRA`."]
    pub gitextra0: GITEXTRA0,
    #[doc = "0x24 - "]
    pub dirty: DIRTY,
    #[doc = "0x28 - "]
    pub model: MODEL,
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [major](major) module"]
pub type MAJOR = crate::Reg<u32, _MAJOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAJOR;
#[doc = "`read()` method returns [major::R](major::R) reader structure"]
impl crate::Readable for MAJOR {}
#[doc = "`write(|w| ..)` method takes [major::W](major::W) writer structure"]
impl crate::Writable for MAJOR {}
#[doc = ""]
pub mod major;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [minor](minor) module"]
pub type MINOR = crate::Reg<u32, _MINOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MINOR;
#[doc = "`read()` method returns [minor::R](minor::R) reader structure"]
impl crate::Readable for MINOR {}
#[doc = "`write(|w| ..)` method takes [minor::W](minor::W) writer structure"]
impl crate::Writable for MINOR {}
#[doc = ""]
pub mod minor;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [revision](revision) module"]
pub type REVISION = crate::Reg<u32, _REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVISION;
#[doc = "`read()` method returns [revision::R](revision::R) reader structure"]
impl crate::Readable for REVISION {}
#[doc = "`write(|w| ..)` method takes [revision::W](revision::W) writer structure"]
impl crate::Writable for REVISION {}
#[doc = ""]
pub mod revision;
#[doc = "Bits 24-31 of `VERSION_GITREV`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev3](gitrev3) module"]
pub type GITREV3 = crate::Reg<u32, _GITREV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV3;
#[doc = "`read()` method returns [gitrev3::R](gitrev3::R) reader structure"]
impl crate::Readable for GITREV3 {}
#[doc = "`write(|w| ..)` method takes [gitrev3::W](gitrev3::W) writer structure"]
impl crate::Writable for GITREV3 {}
#[doc = "Bits 24-31 of `VERSION_GITREV`."]
pub mod gitrev3;
#[doc = "Bits 16-23 of `VERSION_GITREV`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev2](gitrev2) module"]
pub type GITREV2 = crate::Reg<u32, _GITREV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV2;
#[doc = "`read()` method returns [gitrev2::R](gitrev2::R) reader structure"]
impl crate::Readable for GITREV2 {}
#[doc = "`write(|w| ..)` method takes [gitrev2::W](gitrev2::W) writer structure"]
impl crate::Writable for GITREV2 {}
#[doc = "Bits 16-23 of `VERSION_GITREV`."]
pub mod gitrev2;
#[doc = "Bits 8-15 of `VERSION_GITREV`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev1](gitrev1) module"]
pub type GITREV1 = crate::Reg<u32, _GITREV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV1;
#[doc = "`read()` method returns [gitrev1::R](gitrev1::R) reader structure"]
impl crate::Readable for GITREV1 {}
#[doc = "`write(|w| ..)` method takes [gitrev1::W](gitrev1::W) writer structure"]
impl crate::Writable for GITREV1 {}
#[doc = "Bits 8-15 of `VERSION_GITREV`."]
pub mod gitrev1;
#[doc = "Bits 0-7 of `VERSION_GITREV`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitrev0](gitrev0) module"]
pub type GITREV0 = crate::Reg<u32, _GITREV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITREV0;
#[doc = "`read()` method returns [gitrev0::R](gitrev0::R) reader structure"]
impl crate::Readable for GITREV0 {}
#[doc = "`write(|w| ..)` method takes [gitrev0::W](gitrev0::W) writer structure"]
impl crate::Writable for GITREV0 {}
#[doc = "Bits 0-7 of `VERSION_GITREV`."]
pub mod gitrev0;
#[doc = "Bits 8-9 of `VERSION_GITEXTRA`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitextra1](gitextra1) module"]
pub type GITEXTRA1 = crate::Reg<u32, _GITEXTRA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITEXTRA1;
#[doc = "`read()` method returns [gitextra1::R](gitextra1::R) reader structure"]
impl crate::Readable for GITEXTRA1 {}
#[doc = "`write(|w| ..)` method takes [gitextra1::W](gitextra1::W) writer structure"]
impl crate::Writable for GITEXTRA1 {}
#[doc = "Bits 8-9 of `VERSION_GITEXTRA`."]
pub mod gitextra1;
#[doc = "Bits 0-7 of `VERSION_GITEXTRA`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gitextra0](gitextra0) module"]
pub type GITEXTRA0 = crate::Reg<u32, _GITEXTRA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GITEXTRA0;
#[doc = "`read()` method returns [gitextra0::R](gitextra0::R) reader structure"]
impl crate::Readable for GITEXTRA0 {}
#[doc = "`write(|w| ..)` method takes [gitextra0::W](gitextra0::W) writer structure"]
impl crate::Writable for GITEXTRA0 {}
#[doc = "Bits 0-7 of `VERSION_GITEXTRA`."]
pub mod gitextra0;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dirty](dirty) module"]
pub type DIRTY = crate::Reg<u32, _DIRTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRTY;
#[doc = "`read()` method returns [dirty::R](dirty::R) reader structure"]
impl crate::Readable for DIRTY {}
#[doc = "`write(|w| ..)` method takes [dirty::W](dirty::W) writer structure"]
impl crate::Writable for DIRTY {}
#[doc = ""]
pub mod dirty;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [model](model) module"]
pub type MODEL = crate::Reg<u32, _MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEL;
#[doc = "`read()` method returns [model::R](model::R) reader structure"]
impl crate::Readable for MODEL {}
#[doc = "`write(|w| ..)` method takes [model::W](model::W) writer structure"]
impl crate::Writable for MODEL {}
#[doc = ""]
pub mod model;
