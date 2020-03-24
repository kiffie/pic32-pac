#[doc = "Reader of register RPA9R"]
pub type R = crate::R<u32, super::RPA9R>;
#[doc = "Writer for register RPA9R"]
pub type W = crate::W<u32, super::RPA9R>;
#[doc = "Register RPA9R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPA9R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPA9R`"]
pub type RPA9R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPA9R`"]
pub struct RPA9R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPA9R_W<'a> {
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
    pub fn rpa9r(&self) -> RPA9R_R {
        RPA9R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpa9r(&mut self) -> RPA9R_W {
        RPA9R_W { w: self }
    }
}
