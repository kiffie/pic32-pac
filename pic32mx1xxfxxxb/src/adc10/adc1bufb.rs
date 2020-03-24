#[doc = "Reader of register ADC1BUFB"]
pub type R = crate::R<u32, super::ADC1BUFB>;
#[doc = "Writer for register ADC1BUFB"]
pub type W = crate::W<u32, super::ADC1BUFB>;
#[doc = "Register ADC1BUFB `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUFB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUFB`"]
pub type ADC1BUFB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUFB`"]
pub struct ADC1BUFB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUFB_W<'a> {
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
    pub fn adc1bufb(&self) -> ADC1BUFB_R {
        ADC1BUFB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1bufb(&mut self) -> ADC1BUFB_W {
        ADC1BUFB_W { w: self }
    }
}
