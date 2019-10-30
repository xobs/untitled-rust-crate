#[doc = "Reader of register OUT"]
pub type R = crate::R<u32, super::OUT>;
#[doc = "Writer for register OUT"]
pub type W = crate::W<u32, super::OUT>;
#[doc = "Register OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `out`"]
pub type OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `out`"]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
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
    pub fn out(&self) -> OUT_R {
        OUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
}
