#[doc = "Reader of register ADC1BUF0"]
pub type R = crate::R<u32, super::ADC1BUF0>;
#[doc = "Writer for register ADC1BUF0"]
pub type W = crate::W<u32, super::ADC1BUF0>;
#[doc = "Register ADC1BUF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUF0`"]
pub type ADC1BUF0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUF0`"]
pub struct ADC1BUF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUF0_W<'a> {
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
    pub fn adc1buf0(&self) -> ADC1BUF0_R {
        ADC1BUF0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buf0(&mut self) -> ADC1BUF0_W {
        ADC1BUF0_W { w: self }
    }
}
