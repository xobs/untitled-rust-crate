#[doc = "Reader of register STAT3"]
pub type R = crate::R<u32, super::STAT3>;
#[doc = "Writer for register STAT3"]
pub type W = crate::W<u32, super::STAT3>;
#[doc = "Register STAT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `stat3`"]
pub type STAT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `stat3`"]
pub struct STAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT3_W<'a> {
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
    pub fn stat3(&self) -> STAT3_R {
        STAT3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn stat3(&mut self) -> STAT3_W {
        STAT3_W { w: self }
    }
}
