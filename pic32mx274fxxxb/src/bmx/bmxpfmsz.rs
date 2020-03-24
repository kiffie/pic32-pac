#[doc = "Reader of register BMXPFMSZ"]
pub type R = crate::R<u32, super::BMXPFMSZ>;
#[doc = "Writer for register BMXPFMSZ"]
pub type W = crate::W<u32, super::BMXPFMSZ>;
#[doc = "Register BMXPFMSZ `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::BMXPFMSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Reader of field `BMXPFMSZ`"]
pub type BMXPFMSZ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BMXPFMSZ`"]
pub struct BMXPFMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXPFMSZ_W<'a> {
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
    pub fn bmxpfmsz(&self) -> BMXPFMSZ_R {
        BMXPFMSZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxpfmsz(&mut self) -> BMXPFMSZ_W {
        BMXPFMSZ_W { w: self }
    }
}
