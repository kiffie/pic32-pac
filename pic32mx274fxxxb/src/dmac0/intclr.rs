#[doc = "Register `INTCLR` reader"]
pub struct R(crate::R<INTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTCLR_SPEC>> for R {
    fn from(reader: crate::R<INTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCLR` writer"]
pub struct W(crate::W<INTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCLR_SPEC>;
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
impl core::convert::From<crate::W<INTCLR_SPEC>> for W {
    fn from(writer: crate::W<INTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHERIF` reader - "]
pub struct CHERIF_R(crate::FieldReader<bool, bool>);
impl CHERIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHERIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHERIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHERIF` writer - "]
pub struct CHERIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHERIF_W<'a> {
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
#[doc = "Field `CHTAIF` reader - "]
pub struct CHTAIF_R(crate::FieldReader<bool, bool>);
impl CHTAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTAIF` writer - "]
pub struct CHTAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTAIF_W<'a> {
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
#[doc = "Field `CHCCIF` reader - "]
pub struct CHCCIF_R(crate::FieldReader<bool, bool>);
impl CHCCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHCCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHCCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHCCIF` writer - "]
pub struct CHCCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCCIF_W<'a> {
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
#[doc = "Field `CHBCIF` reader - "]
pub struct CHBCIF_R(crate::FieldReader<bool, bool>);
impl CHBCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHBCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBCIF` writer - "]
pub struct CHBCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHBCIF_W<'a> {
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
#[doc = "Field `CHDHIF` reader - "]
pub struct CHDHIF_R(crate::FieldReader<bool, bool>);
impl CHDHIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHDHIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDHIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDHIF` writer - "]
pub struct CHDHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDHIF_W<'a> {
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
#[doc = "Field `CHDDIF` reader - "]
pub struct CHDDIF_R(crate::FieldReader<bool, bool>);
impl CHDDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHDDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDDIF` writer - "]
pub struct CHDDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDDIF_W<'a> {
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
#[doc = "Field `CHSHIF` reader - "]
pub struct CHSHIF_R(crate::FieldReader<bool, bool>);
impl CHSHIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSHIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSHIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSHIF` writer - "]
pub struct CHSHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSHIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CHSDIF` reader - "]
pub struct CHSDIF_R(crate::FieldReader<bool, bool>);
impl CHSDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSDIF` writer - "]
pub struct CHSDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSDIF_W<'a> {
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
#[doc = "Field `CHERIE` reader - "]
pub struct CHERIE_R(crate::FieldReader<bool, bool>);
impl CHERIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHERIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHERIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHERIE` writer - "]
pub struct CHERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHERIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CHTAIE` reader - "]
pub struct CHTAIE_R(crate::FieldReader<bool, bool>);
impl CHTAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTAIE` writer - "]
pub struct CHTAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTAIE_W<'a> {
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
#[doc = "Field `CHCCIE` reader - "]
pub struct CHCCIE_R(crate::FieldReader<bool, bool>);
impl CHCCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHCCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHCCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHCCIE` writer - "]
pub struct CHCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCCIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CHBCIE` reader - "]
pub struct CHBCIE_R(crate::FieldReader<bool, bool>);
impl CHBCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHBCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBCIE` writer - "]
pub struct CHBCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHBCIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CHDHIE` reader - "]
pub struct CHDHIE_R(crate::FieldReader<bool, bool>);
impl CHDHIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHDHIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDHIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDHIE` writer - "]
pub struct CHDHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDHIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CHDDIE` reader - "]
pub struct CHDDIE_R(crate::FieldReader<bool, bool>);
impl CHDDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHDDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDDIE` writer - "]
pub struct CHDDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CHSHIE` reader - "]
pub struct CHSHIE_R(crate::FieldReader<bool, bool>);
impl CHSHIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSHIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSHIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSHIE` writer - "]
pub struct CHSHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSHIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CHSDIE` reader - "]
pub struct CHSDIE_R(crate::FieldReader<bool, bool>);
impl CHSDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSDIE` writer - "]
pub struct CHSDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cherif(&self) -> CHERIF_R {
        CHERIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chtaif(&self) -> CHTAIF_R {
        CHTAIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chccif(&self) -> CHCCIF_R {
        CHCCIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn chbcif(&self) -> CHBCIF_R {
        CHBCIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chdhif(&self) -> CHDHIF_R {
        CHDHIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chddif(&self) -> CHDDIF_R {
        CHDDIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chshif(&self) -> CHSHIF_R {
        CHSHIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chsdif(&self) -> CHSDIF_R {
        CHSDIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cherie(&self) -> CHERIE_R {
        CHERIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn chtaie(&self) -> CHTAIE_R {
        CHTAIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn chccie(&self) -> CHCCIE_R {
        CHCCIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn chbcie(&self) -> CHBCIE_R {
        CHBCIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn chdhie(&self) -> CHDHIE_R {
        CHDHIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn chddie(&self) -> CHDDIE_R {
        CHDDIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn chshie(&self) -> CHSHIE_R {
        CHSHIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn chsdie(&self) -> CHSDIE_R {
        CHSDIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cherif(&mut self) -> CHERIF_W {
        CHERIF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chtaif(&mut self) -> CHTAIF_W {
        CHTAIF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chccif(&mut self) -> CHCCIF_W {
        CHCCIF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn chbcif(&mut self) -> CHBCIF_W {
        CHBCIF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chdhif(&mut self) -> CHDHIF_W {
        CHDHIF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chddif(&mut self) -> CHDDIF_W {
        CHDDIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chshif(&mut self) -> CHSHIF_W {
        CHSHIF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chsdif(&mut self) -> CHSDIF_W {
        CHSDIF_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cherie(&mut self) -> CHERIE_W {
        CHERIE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn chtaie(&mut self) -> CHTAIE_W {
        CHTAIE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn chccie(&mut self) -> CHCCIE_W {
        CHCCIE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn chbcie(&mut self) -> CHBCIE_W {
        CHBCIE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn chdhie(&mut self) -> CHDHIE_W {
        CHDHIE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn chddie(&mut self) -> CHDDIE_W {
        CHDDIE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn chshie(&mut self) -> CHSHIE_W {
        CHSHIE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn chsdie(&mut self) -> CHSDIE_W {
        CHSDIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0INTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](index.html) module"]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intclr::R](R) reader structure"]
impl crate::Readable for INTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intclr::W](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
