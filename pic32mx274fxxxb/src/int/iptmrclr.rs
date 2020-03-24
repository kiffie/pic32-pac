#[doc = "Reader of register IPTMRCLR"]
pub type R = crate::R<u32, super::IPTMRCLR>;
#[doc = "Writer for register IPTMRCLR"]
pub type W = crate::W<u32, super::IPTMRCLR>;
#[doc = "Register IPTMRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPTMRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPTMR`"]
pub type IPTMR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IPTMR`"]
pub struct IPTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTMR_W<'a> {
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
    pub fn iptmr(&self) -> IPTMR_R {
        IPTMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iptmr(&mut self) -> IPTMR_W {
        IPTMR_W { w: self }
    }
}
