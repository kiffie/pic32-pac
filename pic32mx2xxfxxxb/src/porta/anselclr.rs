#[doc = "Register `ANSELCLR` reader"]
pub struct R(crate::R<ANSELCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANSELCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ANSELCLR_SPEC>> for R {
    fn from(reader: crate::R<ANSELCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANSELCLR` writer"]
pub struct W(crate::W<ANSELCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANSELCLR_SPEC>;
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
impl core::convert::From<crate::W<ANSELCLR_SPEC>> for W {
    fn from(writer: crate::W<ANSELCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSA0` reader - "]
pub struct ANSA0_R(crate::FieldReader<bool, bool>);
impl ANSA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSA0` writer - "]
pub struct ANSA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSA0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Field `ANSA1` reader - "]
pub struct ANSA1_R(crate::FieldReader<bool, bool>);
impl ANSA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSA1` writer - "]
pub struct ANSA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSA1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ansa0(&self) -> ANSA0_R {
        ANSA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansa1(&self) -> ANSA1_R {
        ANSA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ansa0(&mut self) -> ANSA0_W {
        ANSA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansa1(&mut self) -> ANSA1_W {
        ANSA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ANSELACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anselclr](index.html) module"]
pub struct ANSELCLR_SPEC;
impl crate::RegisterSpec for ANSELCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anselclr::R](R) reader structure"]
impl crate::Readable for ANSELCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anselclr::W](W) writer structure"]
impl crate::Writable for ANSELCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANSELCLR to value 0"]
impl crate::Resettable for ANSELCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
