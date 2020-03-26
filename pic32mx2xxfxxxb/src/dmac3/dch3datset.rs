#[doc = "Reader of register DCH3DATSET"]
pub type R = crate::R<u32, super::DCH3DATSET>;
#[doc = "Writer for register DCH3DATSET"]
pub type W = crate::W<u32, super::DCH3DATSET>;
#[doc = "Register DCH3DATSET `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH3DATSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCHPDAT`"]
pub type DCHPDAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCHPDAT`"]
pub struct DCHPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DCHPDAT_W<'a> {
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
    pub fn dchpdat(&self) -> DCHPDAT_R {
        DCHPDAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dchpdat(&mut self) -> DCHPDAT_W {
        DCHPDAT_W { w: self }
    }
}
