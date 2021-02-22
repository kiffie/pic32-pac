#[doc = "Register `CMSTATINV` reader"]
pub struct R(crate::R<CMSTATINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMSTATINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CMSTATINV_SPEC>> for R {
    fn from(reader: crate::R<CMSTATINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMSTATINV` writer"]
pub struct W(crate::W<CMSTATINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMSTATINV_SPEC>;
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
impl core::convert::From<crate::W<CMSTATINV_SPEC>> for W {
    fn from(writer: crate::W<CMSTATINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C1OUT` reader - "]
pub struct C1OUT_R(crate::FieldReader<bool, bool>);
impl C1OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1OUT` writer - "]
pub struct C1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> C1OUT_W<'a> {
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
#[doc = "Field `C2OUT` reader - "]
pub struct C2OUT_R(crate::FieldReader<bool, bool>);
impl C2OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2OUT` writer - "]
pub struct C2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> C2OUT_W<'a> {
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
#[doc = "Field `C3OUT` reader - "]
pub struct C3OUT_R(crate::FieldReader<bool, bool>);
impl C3OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        C3OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C3OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C3OUT` writer - "]
pub struct C3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> C3OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SIDL` reader - "]
pub struct SIDL_R(crate::FieldReader<bool, bool>);
impl SIDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDL` writer - "]
pub struct SIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FRZ` reader - "]
pub struct FRZ_R(crate::FieldReader<bool, bool>);
impl FRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - "]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c1out(&self) -> C1OUT_R {
        C1OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c2out(&self) -> C2OUT_R {
        C2OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c3out(&self) -> C3OUT_R {
        C3OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c1out(&mut self) -> C1OUT_W {
        C1OUT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c2out(&mut self) -> C2OUT_W {
        C2OUT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c3out(&mut self) -> C3OUT_W {
        C3OUT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMSTATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmstatinv](index.html) module"]
pub struct CMSTATINV_SPEC;
impl crate::RegisterSpec for CMSTATINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmstatinv::R](R) reader structure"]
impl crate::Readable for CMSTATINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmstatinv::W](W) writer structure"]
impl crate::Writable for CMSTATINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMSTATINV to value 0"]
impl crate::Resettable for CMSTATINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
