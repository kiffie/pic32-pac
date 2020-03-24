#[doc = "Reader of register DCH0SSIZ"]
pub type R = crate::R<u32, super::DCH0SSIZ>;
#[doc = "Writer for register DCH0SSIZ"]
pub type W = crate::W<u32, super::DCH0SSIZ>;
#[doc = "Register DCH0SSIZ `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH0SSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHSSIZ`"]
pub type CHSSIZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHSSIZ`"]
pub struct CHSSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSSIZ_W<'a> {
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
    pub fn chssiz(&self) -> CHSSIZ_R {
        CHSSIZ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chssiz(&mut self) -> CHSSIZ_W {
        CHSSIZ_W { w: self }
    }
}
