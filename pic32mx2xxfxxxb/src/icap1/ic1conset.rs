#[doc = "Register `IC1CONSET` reader"]
pub struct R(crate::R<IC1CONSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC1CONSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IC1CONSET_SPEC>> for R {
    fn from(reader: crate::R<IC1CONSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC1CONSET` writer"]
pub struct W(crate::W<IC1CONSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC1CONSET_SPEC>;
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
impl core::convert::From<crate::W<IC1CONSET_SPEC>> for W {
    fn from(writer: crate::W<IC1CONSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICM` reader - "]
pub struct ICM_R(crate::FieldReader<u8, u8>);
impl ICM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ICM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICM` writer - "]
pub struct ICM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Field `ICBNE` reader - "]
pub struct ICBNE_R(crate::FieldReader<bool, bool>);
impl ICBNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICBNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICBNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICBNE` writer - "]
pub struct ICBNE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICBNE_W<'a> {
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
#[doc = "Field `ICOV` reader - "]
pub struct ICOV_R(crate::FieldReader<bool, bool>);
impl ICOV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICOV` writer - "]
pub struct ICOV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICOV_W<'a> {
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
#[doc = "Field `ICI` reader - "]
pub struct ICI_R(crate::FieldReader<u8, u8>);
impl ICI_R {
    pub(crate) fn new(bits: u8) -> Self {
        ICI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICI` writer - "]
pub struct ICI_W<'a> {
    w: &'a mut W,
}
impl<'a> ICI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `ICTMR` reader - "]
pub struct ICTMR_R(crate::FieldReader<bool, bool>);
impl ICTMR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICTMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICTMR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICTMR` writer - "]
pub struct ICTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICTMR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `C32` reader - "]
pub struct C32_R(crate::FieldReader<bool, bool>);
impl C32_R {
    pub(crate) fn new(bits: bool) -> Self {
        C32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C32` writer - "]
pub struct C32_W<'a> {
    w: &'a mut W,
}
impl<'a> C32_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FEDGE` reader - "]
pub struct FEDGE_R(crate::FieldReader<bool, bool>);
impl FEDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEDGE` writer - "]
pub struct FEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEDGE_W<'a> {
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
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn icm(&self) -> ICM_R {
        ICM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn icbne(&self) -> ICBNE_R {
        ICBNE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn icov(&self) -> ICOV_R {
        ICOV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn ici(&self) -> ICI_R {
        ICI_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ictmr(&self) -> ICTMR_R {
        ICTMR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn c32(&self) -> C32_R {
        C32_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fedge(&self) -> FEDGE_R {
        FEDGE_R::new(((self.bits >> 9) & 0x01) != 0)
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn icm(&mut self) -> ICM_W {
        ICM_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn icbne(&mut self) -> ICBNE_W {
        ICBNE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn icov(&mut self) -> ICOV_W {
        ICOV_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn ici(&mut self) -> ICI_W {
        ICI_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ictmr(&mut self) -> ICTMR_W {
        ICTMR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn c32(&mut self) -> C32_W {
        C32_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fedge(&mut self) -> FEDGE_W {
        FEDGE_W { w: self }
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
#[doc = "IC1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1conset](index.html) module"]
pub struct IC1CONSET_SPEC;
impl crate::RegisterSpec for IC1CONSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic1conset::R](R) reader structure"]
impl crate::Readable for IC1CONSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic1conset::W](W) writer structure"]
impl crate::Writable for IC1CONSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC1CONSET to value 0"]
impl crate::Resettable for IC1CONSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
