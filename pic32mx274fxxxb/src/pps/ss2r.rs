#[doc = "Reader of register SS2R"]
pub type R = crate::R<u32, super::SS2R>;
#[doc = "Writer for register SS2R"]
pub type W = crate::W<u32, super::SS2R>;
#[doc = "Register SS2R `reset()`'s with value 0"]
impl crate::ResetValue for super::SS2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SS2R`"]
pub type SS2R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SS2R`"]
pub struct SS2R_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2R_W<'a> {
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
    pub fn ss2r(&self) -> SS2R_R {
        SS2R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ss2r(&mut self) -> SS2R_W {
        SS2R_W { w: self }
    }
}
