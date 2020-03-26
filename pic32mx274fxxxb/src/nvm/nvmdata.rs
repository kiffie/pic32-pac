#[doc = "Reader of register NVMDATA"]
pub type R = crate::R<u32, super::NVMDATA>;
#[doc = "Writer for register NVMDATA"]
pub type W = crate::W<u32, super::NVMDATA>;
#[doc = "Register NVMDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::NVMDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NVMDATA`"]
pub type NVMDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NVMDATA`"]
pub struct NVMDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMDATA_W<'a> {
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
    pub fn nvmdata(&self) -> NVMDATA_R {
        NVMDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn nvmdata(&mut self) -> NVMDATA_W {
        NVMDATA_W { w: self }
    }
}