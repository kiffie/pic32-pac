#[doc = "Reader of register RPC0R"]
pub type R = crate::R<u32, super::RPC0R>;
#[doc = "Writer for register RPC0R"]
pub type W = crate::W<u32, super::RPC0R>;
#[doc = "Register RPC0R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC0R`"]
pub type RPC0R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC0R`"]
pub struct RPC0R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC0R_W<'a> {
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
    pub fn rpc0r(&self) -> RPC0R_R {
        RPC0R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc0r(&mut self) -> RPC0R_W {
        RPC0R_W { w: self }
    }
}
