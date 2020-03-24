#[doc = "Reader of register CHEMSKINV"]
pub type R = crate::R<u32, super::CHEMSKINV>;
#[doc = "Writer for register CHEMSKINV"]
pub type W = crate::W<u32, super::CHEMSKINV>;
#[doc = "Register CHEMSKINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CHEMSKINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LMASK`"]
pub type LMASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LMASK`"]
pub struct LMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn lmask(&self) -> LMASK_R {
        LMASK_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn lmask(&mut self) -> LMASK_W {
        LMASK_W { w: self }
    }
}
