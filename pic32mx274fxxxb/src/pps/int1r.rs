#[doc = "Reader of register INT1R"]
pub type R = crate::R<u32, super::INT1R>;
#[doc = "Writer for register INT1R"]
pub type W = crate::W<u32, super::INT1R>;
#[doc = "Register INT1R `reset()`'s with value 0"]
impl crate::ResetValue for super::INT1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT1R`"]
pub type INT1R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT1R`"]
pub struct INT1R_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1R_W<'a> {
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
    pub fn int1r(&self) -> INT1R_R {
        INT1R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn int1r(&mut self) -> INT1R_W {
        INT1R_W { w: self }
    }
}
