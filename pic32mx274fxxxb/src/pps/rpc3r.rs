#[doc = "Reader of register RPC3R"]
pub type R = crate::R<u32, super::RPC3R>;
#[doc = "Writer for register RPC3R"]
pub type W = crate::W<u32, super::RPC3R>;
#[doc = "Register RPC3R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC3R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC3R`"]
pub type RPC3R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC3R`"]
pub struct RPC3R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC3R_W<'a> {
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
    pub fn rpc3r(&self) -> RPC3R_R {
        RPC3R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc3r(&mut self) -> RPC3R_W {
        RPC3R_W { w: self }
    }
}
