#[doc = "Reader of register RAW"]
pub type R = crate::R<u32, super::RAW>;
#[doc = "Writer for register RAW"]
pub type W = crate::W<u32, super::RAW>;
#[doc = "Register RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `raw`"]
pub type RAW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `raw`"]
pub struct RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn raw(&self) -> RAW_R {
        RAW_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn raw(&mut self) -> RAW_W {
        RAW_W { w: self }
    }
}
