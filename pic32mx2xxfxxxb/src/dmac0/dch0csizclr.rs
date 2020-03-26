#[doc = "Reader of register DCH0CSIZCLR"]
pub type R = crate::R<u32, super::DCH0CSIZCLR>;
#[doc = "Writer for register DCH0CSIZCLR"]
pub type W = crate::W<u32, super::DCH0CSIZCLR>;
#[doc = "Register DCH0CSIZCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH0CSIZCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHCSIZ`"]
pub type CHCSIZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHCSIZ`"]
pub struct CHCSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chcsiz(&self) -> CHCSIZ_R {
        CHCSIZ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chcsiz(&mut self) -> CHCSIZ_W {
        CHCSIZ_W { w: self }
    }
}
