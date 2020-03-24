#[doc = "Reader of register U1FRMH"]
pub type R = crate::R<u32, super::U1FRMH>;
#[doc = "Writer for register U1FRMH"]
pub type W = crate::W<u32, super::U1FRMH>;
#[doc = "Register U1FRMH `reset()`'s with value 0"]
impl crate::ResetValue for super::U1FRMH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRMH`"]
pub type FRMH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRMH`"]
pub struct FRMH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn frmh(&self) -> FRMH_R {
        FRMH_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn frmh(&mut self) -> FRMH_W {
        FRMH_W { w: self }
    }
}
