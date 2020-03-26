#[doc = "Reader of register MSK"]
pub type R = crate::R<u32, super::MSK>;
#[doc = "Writer for register MSK"]
pub type W = crate::W<u32, super::MSK>;
#[doc = "Register MSK `reset()`'s with value 0"]
impl crate::ResetValue for super::MSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2CMSK`"]
pub type I2CMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2CMSK`"]
pub struct I2CMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2cmsk(&self) -> I2CMSK_R {
        I2CMSK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2cmsk(&mut self) -> I2CMSK_W {
        I2CMSK_W { w: self }
    }
}
