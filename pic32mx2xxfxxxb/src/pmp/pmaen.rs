#[doc = "Reader of register PMAEN"]
pub type R = crate::R<u32, super::PMAEN>;
#[doc = "Writer for register PMAEN"]
pub type W = crate::W<u32, super::PMAEN>;
#[doc = "Register PMAEN `reset()`'s with value 0"]
impl crate::ResetValue for super::PMAEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PTEN`"]
pub type PTEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTEN`"]
pub struct PTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEN_W<'a> {
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
    pub fn pten(&self) -> PTEN_R {
        PTEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pten(&mut self) -> PTEN_W {
        PTEN_W { w: self }
    }
}
