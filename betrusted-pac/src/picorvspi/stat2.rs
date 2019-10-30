#[doc = "Reader of register STAT2"]
pub type R = crate::R<u32, super::STAT2>;
#[doc = "Writer for register STAT2"]
pub type W = crate::W<u32, super::STAT2>;
#[doc = "Register STAT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `stat2`"]
pub type STAT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `stat2`"]
pub struct STAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT2_W<'a> {
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
    pub fn stat2(&self) -> STAT2_R {
        STAT2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn stat2(&mut self) -> STAT2_W {
        STAT2_W { w: self }
    }
}
