#[doc = "Reader of register PMRDINSET"]
pub type R = crate::R<u32, super::PMRDINSET>;
#[doc = "Writer for register PMRDINSET"]
pub type W = crate::W<u32, super::PMRDINSET>;
#[doc = "Register PMRDINSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PMRDINSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDATAIN`"]
pub type RDATAIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDATAIN`"]
pub struct RDATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATAIN_W<'a> {
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
    pub fn rdatain(&self) -> RDATAIN_R {
        RDATAIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdatain(&mut self) -> RDATAIN_W {
        RDATAIN_W { w: self }
    }
}
