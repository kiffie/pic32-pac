#[doc = "Reader of register RPA4R"]
pub type R = crate::R<u32, super::RPA4R>;
#[doc = "Writer for register RPA4R"]
pub type W = crate::W<u32, super::RPA4R>;
#[doc = "Register RPA4R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPA4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPA4R`"]
pub type RPA4R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPA4R`"]
pub struct RPA4R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPA4R_W<'a> {
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
    pub fn rpa4r(&self) -> RPA4R_R {
        RPA4R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpa4r(&mut self) -> RPA4R_W {
        RPA4R_W { w: self }
    }
}
