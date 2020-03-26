#[doc = "Reader of register U1BDTP1INV"]
pub type R = crate::R<u32, super::U1BDTP1INV>;
#[doc = "Writer for register U1BDTP1INV"]
pub type W = crate::W<u32, super::U1BDTP1INV>;
#[doc = "Register U1BDTP1INV `reset()`'s with value 0"]
impl crate::ResetValue for super::U1BDTP1INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BDTPTRL`"]
pub type BDTPTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDTPTRL`"]
pub struct BDTPTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BDTPTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn bdtptrl(&self) -> BDTPTRL_R {
        BDTPTRL_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn bdtptrl(&mut self) -> BDTPTRL_W {
        BDTPTRL_W { w: self }
    }
}
