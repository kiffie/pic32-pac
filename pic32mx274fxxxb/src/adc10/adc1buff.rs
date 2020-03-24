#[doc = "Reader of register ADC1BUFF"]
pub type R = crate::R<u32, super::ADC1BUFF>;
#[doc = "Writer for register ADC1BUFF"]
pub type W = crate::W<u32, super::ADC1BUFF>;
#[doc = "Register ADC1BUFF `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUFF`"]
pub type ADC1BUFF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUFF`"]
pub struct ADC1BUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUFF_W<'a> {
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
    pub fn adc1buff(&self) -> ADC1BUFF_R {
        ADC1BUFF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buff(&mut self) -> ADC1BUFF_W {
        ADC1BUFF_W { w: self }
    }
}
