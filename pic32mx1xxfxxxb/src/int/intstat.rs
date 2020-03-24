#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VEC`"]
pub type VEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VEC`"]
pub struct VEC_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SRIPL`"]
pub type SRIPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRIPL`"]
pub struct SRIPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRIPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vec(&self) -> VEC_R {
        VEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sripl(&self) -> SRIPL_R {
        SRIPL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vec(&mut self) -> VEC_W {
        VEC_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sripl(&mut self) -> SRIPL_W {
        SRIPL_W { w: self }
    }
}
