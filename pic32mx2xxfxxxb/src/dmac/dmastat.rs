#[doc = "Register `DMASTAT` reader"]
pub struct R(crate::R<DMASTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMASTAT_SPEC>> for R {
    fn from(reader: crate::R<DMASTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASTAT` writer"]
pub struct W(crate::W<DMASTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASTAT_SPEC>;
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
impl core::convert::From<crate::W<DMASTAT_SPEC>> for W {
    fn from(writer: crate::W<DMASTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMACH` reader - "]
pub struct DMACH_R(crate::FieldReader<u8, u8>);
impl DMACH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMACH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMACH` writer - "]
pub struct DMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Field `RDWR` reader - "]
pub struct RDWR_R(crate::FieldReader<bool, bool>);
impl RDWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDWR` writer - "]
pub struct RDWR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dmach(&self) -> DMACH_R {
        DMACH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rdwr(&self) -> RDWR_R {
        RDWR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dmach(&mut self) -> DMACH_W {
        DMACH_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rdwr(&mut self) -> RDWR_W {
        RDWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMASTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](index.html) module"]
pub struct DMASTAT_SPEC;
impl crate::RegisterSpec for DMASTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmastat::R](R) reader structure"]
impl crate::Readable for DMASTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmastat::W](W) writer structure"]
impl crate::Writable for DMASTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASTAT to value 0"]
impl crate::Resettable for DMASTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
