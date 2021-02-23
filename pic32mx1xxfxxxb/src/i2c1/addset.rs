#[doc = "Register `ADDSET` reader"]
pub struct R(crate::R<ADDSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDSET_SPEC>> for R {
    fn from(reader: crate::R<ADDSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDSET` writer"]
pub struct W(crate::W<ADDSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<ADDSET_SPEC>> for W {
    fn from(writer: crate::W<ADDSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CADD` reader - "]
pub struct I2CADD_R(crate::FieldReader<u16, u16>);
impl I2CADD_R {
    pub(crate) fn new(bits: u16) -> Self {
        I2CADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CADD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CADD` writer - "]
pub struct I2CADD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CADD_W<'a> {
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
    pub fn i2cadd(&self) -> I2CADD_R {
        I2CADD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2cadd(&mut self) -> I2CADD_W {
        I2CADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C1ADDSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addset](index.html) module"]
pub struct ADDSET_SPEC;
impl crate::RegisterSpec for ADDSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addset::R](R) reader structure"]
impl crate::Readable for ADDSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addset::W](W) writer structure"]
impl crate::Writable for ADDSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDSET to value 0"]
impl crate::Resettable for ADDSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
