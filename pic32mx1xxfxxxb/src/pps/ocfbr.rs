#[doc = "Reader of register OCFBR"]
pub type R = crate::R<u32, super::OCFBR>;
#[doc = "Writer for register OCFBR"]
pub type W = crate::W<u32, super::OCFBR>;
#[doc = "Register OCFBR `reset()`'s with value 0"]
impl crate::ResetValue for super::OCFBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OCFBR`"]
pub type OCFBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCFBR`"]
pub struct OCFBR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCFBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ocfbr(&self) -> OCFBR_R {
        OCFBR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ocfbr(&mut self) -> OCFBR_W {
        OCFBR_W { w: self }
    }
}
