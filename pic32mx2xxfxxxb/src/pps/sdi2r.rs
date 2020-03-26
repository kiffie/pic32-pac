#[doc = "Reader of register SDI2R"]
pub type R = crate::R<u32, super::SDI2R>;
#[doc = "Writer for register SDI2R"]
pub type W = crate::W<u32, super::SDI2R>;
#[doc = "Register SDI2R `reset()`'s with value 0"]
impl crate::ResetValue for super::SDI2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDI2R`"]
pub type SDI2R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDI2R`"]
pub struct SDI2R_W<'a> {
    w: &'a mut W,
}
impl<'a> SDI2R_W<'a> {
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
    pub fn sdi2r(&self) -> SDI2R_R {
        SDI2R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sdi2r(&mut self) -> SDI2R_W {
        SDI2R_W { w: self }
    }
}
