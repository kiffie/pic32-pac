#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTSTAT_SPEC>> for R {
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTAT` writer"]
pub struct W(crate::W<INTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTAT_SPEC>;
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
impl core::convert::From<crate::W<INTSTAT_SPEC>> for W {
    fn from(writer: crate::W<INTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VEC` reader - "]
pub struct VEC_R(crate::FieldReader<u8, u8>);
impl VEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        VEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VEC` writer - "]
pub struct VEC_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Field `SRIPL` reader - "]
pub struct SRIPL_R(crate::FieldReader<u8, u8>);
impl SRIPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRIPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRIPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRIPL` writer - "]
pub struct SRIPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRIPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vec(&self) -> VEC_R {
        VEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sripl(&self) -> SRIPL_R {
        SRIPL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vec(&mut self) -> VEC_W {
        VEC_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sripl(&mut self) -> SRIPL_W {
        SRIPL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstat::W](W) writer structure"]
impl crate::Writable for INTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
