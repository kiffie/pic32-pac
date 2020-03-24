#[doc = "Reader of register NVMADDRSET"]
pub type R = crate::R<u32, super::NVMADDRSET>;
#[doc = "Writer for register NVMADDRSET"]
pub type W = crate::W<u32, super::NVMADDRSET>;
#[doc = "Register NVMADDRSET `reset()`'s with value 0"]
impl crate::ResetValue for super::NVMADDRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NVMADDR`"]
pub type NVMADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NVMADDR`"]
pub struct NVMADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMADDR_W<'a> {
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
    pub fn nvmaddr(&self) -> NVMADDR_R {
        NVMADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn nvmaddr(&mut self) -> NVMADDR_W {
        NVMADDR_W { w: self }
    }
}
