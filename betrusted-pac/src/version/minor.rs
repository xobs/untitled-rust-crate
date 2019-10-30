#[doc = "Reader of register MINOR"]
pub type R = crate::R<u32, super::MINOR>;
#[doc = "Writer for register MINOR"]
pub type W = crate::W<u32, super::MINOR>;
#[doc = "Register MINOR `reset()`'s with value 0"]
impl crate::ResetValue for super::MINOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `minor`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `minor`"]
pub struct MINOR_W<'a> {
    w: &'a mut W,
}
impl<'a> MINOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn minor(&mut self) -> MINOR_W {
        MINOR_W { w: self }
    }
}
