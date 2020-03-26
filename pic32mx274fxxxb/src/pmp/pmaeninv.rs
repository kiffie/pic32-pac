#[doc = "Reader of register PMAENINV"]
pub type R = crate::R<u32, super::PMAENINV>;
#[doc = "Writer for register PMAENINV"]
pub type W = crate::W<u32, super::PMAENINV>;
#[doc = "Register PMAENINV `reset()`'s with value 0"]
impl crate::ResetValue for super::PMAENINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PTEN`"]
pub type PTEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PTEN`"]
pub struct PTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn pten(&self) -> PTEN_R {
        PTEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn pten(&mut self) -> PTEN_W {
        PTEN_W { w: self }
    }
}