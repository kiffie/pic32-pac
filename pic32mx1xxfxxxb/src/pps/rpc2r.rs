#[doc = "Reader of register RPC2R"]
pub type R = crate::R<u32, super::RPC2R>;
#[doc = "Writer for register RPC2R"]
pub type W = crate::W<u32, super::RPC2R>;
#[doc = "Register RPC2R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC2R`"]
pub type RPC2R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC2R`"]
pub struct RPC2R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC2R_W<'a> {
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
    pub fn rpc2r(&self) -> RPC2R_R {
        RPC2R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc2r(&mut self) -> RPC2R_W {
        RPC2R_W { w: self }
    }
}
