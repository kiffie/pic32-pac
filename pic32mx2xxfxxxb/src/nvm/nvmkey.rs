#[doc = "Reader of register NVMKEY"]
pub type R = crate::R<u32, super::NVMKEY>;
#[doc = "Writer for register NVMKEY"]
pub type W = crate::W<u32, super::NVMKEY>;
#[doc = "Register NVMKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::NVMKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NVMKEY`"]
pub type NVMKEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NVMKEY`"]
pub struct NVMKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMKEY_W<'a> {
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
    pub fn nvmkey(&self) -> NVMKEY_R {
        NVMKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn nvmkey(&mut self) -> NVMKEY_W {
        NVMKEY_W { w: self }
    }
}
