#[doc = "Reader of register TXR"]
pub type R = crate::R<u32, super::TXR>;
#[doc = "Writer for register TXR"]
pub type W = crate::W<u32, super::TXR>;
#[doc = "Register TXR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `txr`"]
pub type TXR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `txr`"]
pub struct TXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXR_W<'a> {
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
    pub fn txr(&self) -> TXR_R {
        TXR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txr(&mut self) -> TXR_W {
        TXR_W { w: self }
    }
}
