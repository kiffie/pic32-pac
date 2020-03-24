#[doc = "Reader of register DSGPR32"]
pub type R = crate::R<u32, super::DSGPR32>;
#[doc = "Writer for register DSGPR32"]
pub type W = crate::W<u32, super::DSGPR32>;
#[doc = "Register DSGPR32 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSGPR32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSGPR`"]
pub type DSGPR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSGPR`"]
pub struct DSGPR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSGPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsgpr(&self) -> DSGPR_R {
        DSGPR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsgpr(&mut self) -> DSGPR_W {
        DSGPR_W { w: self }
    }
}
