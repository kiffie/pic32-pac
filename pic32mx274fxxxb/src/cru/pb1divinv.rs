#[doc = "Register `PB1DIVINV` reader"]
pub struct R(crate::R<PB1DIVINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB1DIVINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PB1DIVINV_SPEC>> for R {
    fn from(reader: crate::R<PB1DIVINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB1DIVINV` writer"]
pub struct W(crate::W<PB1DIVINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB1DIVINV_SPEC>;
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
impl core::convert::From<crate::W<PB1DIVINV_SPEC>> for W {
    fn from(writer: crate::W<PB1DIVINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDIV` reader - "]
pub struct PBDIV_R(crate::FieldReader<u8, u8>);
impl PBDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBDIV` writer - "]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Field `PBDIVRDY` reader - "]
pub struct PBDIVRDY_R(crate::FieldReader<bool, bool>);
impl PBDIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBDIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBDIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBDIVRDY` writer - "]
pub struct PBDIVRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIVRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ON` reader - "]
pub struct ON_R(crate::FieldReader<bool, bool>);
impl ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ON` writer - "]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pbdivrdy(&self) -> PBDIVRDY_R {
        PBDIVRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pbdivrdy(&mut self) -> PBDIVRDY_W {
        PBDIVRDY_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PB1DIVINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb1divinv](index.html) module"]
pub struct PB1DIVINV_SPEC;
impl crate::RegisterSpec for PB1DIVINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb1divinv::R](R) reader structure"]
impl crate::Readable for PB1DIVINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb1divinv::W](W) writer structure"]
impl crate::Writable for PB1DIVINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB1DIVINV to value 0"]
impl crate::Resettable for PB1DIVINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
