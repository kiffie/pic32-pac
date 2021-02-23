#[doc = "Register `RPB10R` reader"]
pub struct R(crate::R<RPB10R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPB10R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RPB10R_SPEC>> for R {
    fn from(reader: crate::R<RPB10R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPB10R` writer"]
pub struct W(crate::W<RPB10R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPB10R_SPEC>;
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
impl core::convert::From<crate::W<RPB10R_SPEC>> for W {
    fn from(writer: crate::W<RPB10R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPB10R` reader - "]
pub struct RPB10R_R(crate::FieldReader<u8, u8>);
impl RPB10R_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPB10R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPB10R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPB10R` writer - "]
pub struct RPB10R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB10R_W<'a> {
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
    pub fn rpb10r(&self) -> RPB10R_R {
        RPB10R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb10r(&mut self) -> RPB10R_W {
        RPB10R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RPB10R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb10r](index.html) module"]
pub struct RPB10R_SPEC;
impl crate::RegisterSpec for RPB10R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpb10r::R](R) reader structure"]
impl crate::Readable for RPB10R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpb10r::W](W) writer structure"]
impl crate::Writable for RPB10R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPB10R to value 0"]
impl crate::Resettable for RPB10R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
