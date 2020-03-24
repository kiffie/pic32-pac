#[doc = "Reader of register U1FRML"]
pub type R = crate::R<u32, super::U1FRML>;
#[doc = "Writer for register U1FRML"]
pub type W = crate::W<u32, super::U1FRML>;
#[doc = "Register U1FRML `reset()`'s with value 0"]
impl crate::ResetValue for super::U1FRML {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRML`"]
pub type FRML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRML`"]
pub struct FRML_W<'a> {
    w: &'a mut W,
}
impl<'a> FRML_W<'a> {
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
    pub fn frml(&self) -> FRML_R {
        FRML_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn frml(&mut self) -> FRML_W {
        FRML_W { w: self }
    }
}
