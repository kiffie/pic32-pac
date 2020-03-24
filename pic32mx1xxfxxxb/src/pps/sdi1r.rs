#[doc = "Reader of register SDI1R"]
pub type R = crate::R<u32, super::SDI1R>;
#[doc = "Writer for register SDI1R"]
pub type W = crate::W<u32, super::SDI1R>;
#[doc = "Register SDI1R `reset()`'s with value 0"]
impl crate::ResetValue for super::SDI1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDI1R`"]
pub type SDI1R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDI1R`"]
pub struct SDI1R_W<'a> {
    w: &'a mut W,
}
impl<'a> SDI1R_W<'a> {
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
    pub fn sdi1r(&self) -> SDI1R_R {
        SDI1R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sdi1r(&mut self) -> SDI1R_W {
        SDI1R_W { w: self }
    }
}
