#[doc = "Reader of register TRNSET"]
pub type R = crate::R<u32, super::TRNSET>;
#[doc = "Writer for register TRNSET"]
pub type W = crate::W<u32, super::TRNSET>;
#[doc = "Register TRNSET `reset()`'s with value 0"]
impl crate::ResetValue for super::TRNSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRN`"]
pub type TRN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRN`"]
pub struct TRN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn trn(&self) -> TRN_R {
        TRN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn trn(&mut self) -> TRN_W {
        TRN_W { w: self }
    }
}
