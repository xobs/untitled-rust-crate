#[doc = "Reader of register BUS_ERRORS2"]
pub type R = crate::R<u32, super::BUS_ERRORS2>;
#[doc = "Writer for register BUS_ERRORS2"]
pub type W = crate::W<u32, super::BUS_ERRORS2>;
#[doc = "Register BUS_ERRORS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BUS_ERRORS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `bus_errors`"]
pub type BUS_ERRORS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `bus_errors`"]
pub struct BUS_ERRORS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ERRORS_W<'a> {
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
    pub fn bus_errors(&self) -> BUS_ERRORS_R {
        BUS_ERRORS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bus_errors(&mut self) -> BUS_ERRORS_W {
        BUS_ERRORS_W { w: self }
    }
}
