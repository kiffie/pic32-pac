#[doc = "Reader of register RPC6R"]
pub type R = crate::R<u32, super::RPC6R>;
#[doc = "Writer for register RPC6R"]
pub type W = crate::W<u32, super::RPC6R>;
#[doc = "Register RPC6R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC6R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC6R`"]
pub type RPC6R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC6R`"]
pub struct RPC6R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC6R_W<'a> {
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
    pub fn rpc6r(&self) -> RPC6R_R {
        RPC6R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc6r(&mut self) -> RPC6R_W {
        RPC6R_W { w: self }
    }
}
