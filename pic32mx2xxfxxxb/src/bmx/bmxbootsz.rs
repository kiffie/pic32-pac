#[doc = "Reader of register BMXBOOTSZ"]
pub type R = crate::R<u32, super::BMXBOOTSZ>;
#[doc = "Writer for register BMXBOOTSZ"]
pub type W = crate::W<u32, super::BMXBOOTSZ>;
#[doc = "Register BMXBOOTSZ `reset()`'s with value 0x3000"]
impl crate::ResetValue for super::BMXBOOTSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000
    }
}
#[doc = "Reader of field `BMXBOOTSZ`"]
pub type BMXBOOTSZ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BMXBOOTSZ`"]
pub struct BMXBOOTSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXBOOTSZ_W<'a> {
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
    pub fn bmxbootsz(&self) -> BMXBOOTSZ_R {
        BMXBOOTSZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxbootsz(&mut self) -> BMXBOOTSZ_W {
        BMXBOOTSZ_W { w: self }
    }
}
