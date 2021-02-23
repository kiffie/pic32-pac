#[doc = "Register `AD1CSSLINV` reader"]
pub struct R(crate::R<AD1CSSLINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AD1CSSLINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AD1CSSLINV_SPEC>> for R {
    fn from(reader: crate::R<AD1CSSLINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AD1CSSLINV` writer"]
pub struct W(crate::W<AD1CSSLINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AD1CSSLINV_SPEC>;
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
impl core::convert::From<crate::W<AD1CSSLINV_SPEC>> for W {
    fn from(writer: crate::W<AD1CSSLINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSSL0` reader - "]
pub struct CSSL0_R(crate::FieldReader<bool, bool>);
impl CSSL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL0` writer - "]
pub struct CSSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL0_W<'a> {
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
#[doc = "Field `CSSL1` reader - "]
pub struct CSSL1_R(crate::FieldReader<bool, bool>);
impl CSSL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL1` writer - "]
pub struct CSSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL1_W<'a> {
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
#[doc = "Field `CSSL2` reader - "]
pub struct CSSL2_R(crate::FieldReader<bool, bool>);
impl CSSL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL2` writer - "]
pub struct CSSL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL2_W<'a> {
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
#[doc = "Field `CSSL3` reader - "]
pub struct CSSL3_R(crate::FieldReader<bool, bool>);
impl CSSL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL3` writer - "]
pub struct CSSL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL3_W<'a> {
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
#[doc = "Field `CSSL4` reader - "]
pub struct CSSL4_R(crate::FieldReader<bool, bool>);
impl CSSL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL4` writer - "]
pub struct CSSL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CSSL5` reader - "]
pub struct CSSL5_R(crate::FieldReader<bool, bool>);
impl CSSL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL5` writer - "]
pub struct CSSL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CSSL9` reader - "]
pub struct CSSL9_R(crate::FieldReader<bool, bool>);
impl CSSL9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL9` writer - "]
pub struct CSSL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CSSL10` reader - "]
pub struct CSSL10_R(crate::FieldReader<bool, bool>);
impl CSSL10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL10` writer - "]
pub struct CSSL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CSSL11` reader - "]
pub struct CSSL11_R(crate::FieldReader<bool, bool>);
impl CSSL11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL11` writer - "]
pub struct CSSL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL11_W<'a> {
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
#[doc = "Field `CSSL13` reader - "]
pub struct CSSL13_R(crate::FieldReader<bool, bool>);
impl CSSL13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL13` writer - "]
pub struct CSSL13_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL13_W<'a> {
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
#[doc = "Field `CSSL14` reader - "]
pub struct CSSL14_R(crate::FieldReader<bool, bool>);
impl CSSL14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL14` writer - "]
pub struct CSSL14_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL14_W<'a> {
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
#[doc = "Field `CSSL15` reader - "]
pub struct CSSL15_R(crate::FieldReader<bool, bool>);
impl CSSL15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL15` writer - "]
pub struct CSSL15_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL15_W<'a> {
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
#[doc = "Field `CSSL17` reader - "]
pub struct CSSL17_R(crate::FieldReader<bool, bool>);
impl CSSL17_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSL17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL17` writer - "]
pub struct CSSL17_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cssl0(&self) -> CSSL0_R {
        CSSL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cssl1(&self) -> CSSL1_R {
        CSSL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cssl2(&self) -> CSSL2_R {
        CSSL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cssl3(&self) -> CSSL3_R {
        CSSL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cssl4(&self) -> CSSL4_R {
        CSSL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cssl5(&self) -> CSSL5_R {
        CSSL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cssl9(&self) -> CSSL9_R {
        CSSL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cssl10(&self) -> CSSL10_R {
        CSSL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cssl11(&self) -> CSSL11_R {
        CSSL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cssl13(&self) -> CSSL13_R {
        CSSL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cssl14(&self) -> CSSL14_R {
        CSSL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cssl15(&self) -> CSSL15_R {
        CSSL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cssl17(&self) -> CSSL17_R {
        CSSL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cssl0(&mut self) -> CSSL0_W {
        CSSL0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cssl1(&mut self) -> CSSL1_W {
        CSSL1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cssl2(&mut self) -> CSSL2_W {
        CSSL2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cssl3(&mut self) -> CSSL3_W {
        CSSL3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cssl4(&mut self) -> CSSL4_W {
        CSSL4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cssl5(&mut self) -> CSSL5_W {
        CSSL5_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cssl9(&mut self) -> CSSL9_W {
        CSSL9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cssl10(&mut self) -> CSSL10_W {
        CSSL10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cssl11(&mut self) -> CSSL11_W {
        CSSL11_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cssl13(&mut self) -> CSSL13_W {
        CSSL13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cssl14(&mut self) -> CSSL14_W {
        CSSL14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cssl15(&mut self) -> CSSL15_W {
        CSSL15_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cssl17(&mut self) -> CSSL17_W {
        CSSL17_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AD1CSSLINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1csslinv](index.html) module"]
pub struct AD1CSSLINV_SPEC;
impl crate::RegisterSpec for AD1CSSLINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ad1csslinv::R](R) reader structure"]
impl crate::Readable for AD1CSSLINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ad1csslinv::W](W) writer structure"]
impl crate::Writable for AD1CSSLINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AD1CSSLINV to value 0"]
impl crate::Resettable for AD1CSSLINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
