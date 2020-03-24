#[doc = "Reader of register ADC1BUF9"]
pub type R = crate::R<u32, super::ADC1BUF9>;
#[doc = "Writer for register ADC1BUF9"]
pub type W = crate::W<u32, super::ADC1BUF9>;
#[doc = "Register ADC1BUF9 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUF9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUF9`"]
pub type ADC1BUF9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUF9`"]
pub struct ADC1BUF9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUF9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buf9(&self) -> ADC1BUF9_R {
        ADC1BUF9_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buf9(&mut self) -> ADC1BUF9_W {
        ADC1BUF9_W { w: self }
    }
}
