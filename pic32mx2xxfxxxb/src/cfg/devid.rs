#[doc = "Reader of register DEVID"]
pub type R = crate::R<u32, super::DEVID>;
#[doc = "Writer for register DEVID"]
pub type W = crate::W<u32, super::DEVID>;
#[doc = "Register DEVID `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVID`"]
pub type DEVID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DEVID`"]
pub struct DEVID_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VER`"]
pub struct VER_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn devid(&self) -> DEVID_R {
        DEVID_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn devid(&mut self) -> DEVID_W {
        DEVID_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ver(&mut self) -> VER_W {
        VER_W { w: self }
    }
}
