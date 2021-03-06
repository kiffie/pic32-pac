#[doc = "Register `RPB12R` reader"]
pub struct R(crate::R<RPB12R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPB12R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RPB12R_SPEC>> for R {
    fn from(reader: crate::R<RPB12R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPB12R` writer"]
pub struct W(crate::W<RPB12R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPB12R_SPEC>;
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
impl core::convert::From<crate::W<RPB12R_SPEC>> for W {
    fn from(writer: crate::W<RPB12R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPB12R` reader - "]
pub struct RPB12R_R(crate::FieldReader<u8, u8>);
impl RPB12R_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPB12R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPB12R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPB12R` writer - "]
pub struct RPB12R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB12R_W<'a> {
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
    pub fn rpb12r(&self) -> RPB12R_R {
        RPB12R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb12r(&mut self) -> RPB12R_W {
        RPB12R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RPB12R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb12r](index.html) module"]
pub struct RPB12R_SPEC;
impl crate::RegisterSpec for RPB12R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpb12r::R](R) reader structure"]
impl crate::Readable for RPB12R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpb12r::W](W) writer structure"]
impl crate::Writable for RPB12R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPB12R to value 0"]
impl crate::Resettable for RPB12R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
