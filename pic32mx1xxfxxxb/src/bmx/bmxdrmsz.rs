#[doc = "Reader of register BMXDRMSZ"]
pub type R = crate::R<u32, super::BMXDRMSZ>;
#[doc = "Writer for register BMXDRMSZ"]
pub type W = crate::W<u32, super::BMXDRMSZ>;
#[doc = "Register BMXDRMSZ `reset()`'s with value 0x0002_0000"]
impl crate::ResetValue for super::BMXDRMSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0000
    }
}
#[doc = "Reader of field `BMXDRMSZ`"]
pub type BMXDRMSZ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BMXDRMSZ`"]
pub struct BMXDRMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXDRMSZ_W<'a> {
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
    pub fn bmxdrmsz(&self) -> BMXDRMSZ_R {
        BMXDRMSZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdrmsz(&mut self) -> BMXDRMSZ_W {
        BMXDRMSZ_W { w: self }
    }
}
