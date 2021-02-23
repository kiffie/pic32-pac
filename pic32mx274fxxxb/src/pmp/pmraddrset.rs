#[doc = "Register `PMRADDRSET` reader"]
pub struct R(crate::R<PMRADDRSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMRADDRSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMRADDRSET_SPEC>> for R {
    fn from(reader: crate::R<PMRADDRSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMRADDRSET` writer"]
pub struct W(crate::W<PMRADDRSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMRADDRSET_SPEC>;
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
impl core::convert::From<crate::W<PMRADDRSET_SPEC>> for W {
    fn from(writer: crate::W<PMRADDRSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RADDR` reader - "]
pub struct RADDR_R(crate::FieldReader<u16, u16>);
impl RADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADDR` writer - "]
pub struct RADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Field `RADDR14` reader - "]
pub struct RADDR14_R(crate::FieldReader<bool, bool>);
impl RADDR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RADDR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADDR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADDR14` writer - "]
pub struct RADDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> RADDR14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RADDR15` reader - "]
pub struct RADDR15_R(crate::FieldReader<bool, bool>);
impl RADDR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RADDR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADDR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADDR15` writer - "]
pub struct RADDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> RADDR15_W<'a> {
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
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn raddr14(&self) -> RADDR14_R {
        RADDR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn raddr15(&self) -> RADDR15_R {
        RADDR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn raddr(&mut self) -> RADDR_W {
        RADDR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn raddr14(&mut self) -> RADDR14_W {
        RADDR14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn raddr15(&mut self) -> RADDR15_W {
        RADDR15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMRADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmraddrset](index.html) module"]
pub struct PMRADDRSET_SPEC;
impl crate::RegisterSpec for PMRADDRSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmraddrset::R](R) reader structure"]
impl crate::Readable for PMRADDRSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmraddrset::W](W) writer structure"]
impl crate::Writable for PMRADDRSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMRADDRSET to value 0"]
impl crate::Resettable for PMRADDRSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
