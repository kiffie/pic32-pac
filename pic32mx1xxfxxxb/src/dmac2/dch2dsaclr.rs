#[doc = "Reader of register DCH2DSACLR"]
pub type R = crate::R<u32, super::DCH2DSACLR>;
#[doc = "Writer for register DCH2DSACLR"]
pub type W = crate::W<u32, super::DCH2DSACLR>;
#[doc = "Register DCH2DSACLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH2DSACLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCH2DSA`"]
pub type DCH2DSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCH2DSA`"]
pub struct DCH2DSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCH2DSA_W<'a> {
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
    pub fn dch2dsa(&self) -> DCH2DSA_R {
        DCH2DSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dch2dsa(&mut self) -> DCH2DSA_W {
        DCH2DSA_W { w: self }
    }
}
