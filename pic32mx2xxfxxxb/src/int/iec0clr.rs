#[doc = "Register `IEC0CLR` reader"]
pub struct R(crate::R<IEC0CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEC0CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IEC0CLR_SPEC>> for R {
    fn from(reader: crate::R<IEC0CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEC0CLR` writer"]
pub struct W(crate::W<IEC0CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEC0CLR_SPEC>;
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
impl core::convert::From<crate::W<IEC0CLR_SPEC>> for W {
    fn from(writer: crate::W<IEC0CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIE` reader - "]
pub struct CTIE_R(crate::FieldReader<bool, bool>);
impl CTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIE` writer - "]
pub struct CTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIE_W<'a> {
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
#[doc = "Field `CS0IE` reader - "]
pub struct CS0IE_R(crate::FieldReader<bool, bool>);
impl CS0IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS0IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS0IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS0IE` writer - "]
pub struct CS0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0IE_W<'a> {
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
#[doc = "Field `CS1IE` reader - "]
pub struct CS1IE_R(crate::FieldReader<bool, bool>);
impl CS1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1IE` writer - "]
pub struct CS1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1IE_W<'a> {
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
#[doc = "Field `INT0IE` reader - "]
pub struct INT0IE_R(crate::FieldReader<bool, bool>);
impl INT0IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT0IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT0IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT0IE` writer - "]
pub struct INT0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0IE_W<'a> {
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
#[doc = "Field `T1IE` reader - "]
pub struct T1IE_R(crate::FieldReader<bool, bool>);
impl T1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        T1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1IE` writer - "]
pub struct T1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T1IE_W<'a> {
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
#[doc = "Field `IC1EIE` reader - "]
pub struct IC1EIE_R(crate::FieldReader<bool, bool>);
impl IC1EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC1EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1EIE` writer - "]
pub struct IC1EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1EIE_W<'a> {
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
#[doc = "Field `IC1IE` reader - "]
pub struct IC1IE_R(crate::FieldReader<bool, bool>);
impl IC1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1IE` writer - "]
pub struct IC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IE_W<'a> {
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
#[doc = "Field `OC1IE` reader - "]
pub struct OC1IE_R(crate::FieldReader<bool, bool>);
impl OC1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1IE` writer - "]
pub struct OC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1IE_W<'a> {
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
#[doc = "Field `INT1IE` reader - "]
pub struct INT1IE_R(crate::FieldReader<bool, bool>);
impl INT1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1IE` writer - "]
pub struct INT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1IE_W<'a> {
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
#[doc = "Field `T2IE` reader - "]
pub struct T2IE_R(crate::FieldReader<bool, bool>);
impl T2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        T2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2IE` writer - "]
pub struct T2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T2IE_W<'a> {
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
#[doc = "Field `IC2EIE` reader - "]
pub struct IC2EIE_R(crate::FieldReader<bool, bool>);
impl IC2EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC2EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2EIE` writer - "]
pub struct IC2EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2EIE_W<'a> {
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
#[doc = "Field `IC2IE` reader - "]
pub struct IC2IE_R(crate::FieldReader<bool, bool>);
impl IC2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2IE` writer - "]
pub struct IC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IE_W<'a> {
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
#[doc = "Field `OC2IE` reader - "]
pub struct OC2IE_R(crate::FieldReader<bool, bool>);
impl OC2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2IE` writer - "]
pub struct OC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `INT2IE` reader - "]
pub struct INT2IE_R(crate::FieldReader<bool, bool>);
impl INT2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2IE` writer - "]
pub struct INT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2IE_W<'a> {
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
#[doc = "Field `T3IE` reader - "]
pub struct T3IE_R(crate::FieldReader<bool, bool>);
impl T3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        T3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T3IE` writer - "]
pub struct T3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T3IE_W<'a> {
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
#[doc = "Field `IC3EIE` reader - "]
pub struct IC3EIE_R(crate::FieldReader<bool, bool>);
impl IC3EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC3EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3EIE` writer - "]
pub struct IC3EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3EIE_W<'a> {
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
#[doc = "Field `IC3IE` reader - "]
pub struct IC3IE_R(crate::FieldReader<bool, bool>);
impl IC3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3IE` writer - "]
pub struct IC3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IE_W<'a> {
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
#[doc = "Field `OC3IE` reader - "]
pub struct OC3IE_R(crate::FieldReader<bool, bool>);
impl OC3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3IE` writer - "]
pub struct OC3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3IE_W<'a> {
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
#[doc = "Field `INT3IE` reader - "]
pub struct INT3IE_R(crate::FieldReader<bool, bool>);
impl INT3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3IE` writer - "]
pub struct INT3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3IE_W<'a> {
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
#[doc = "Field `T4IE` reader - "]
pub struct T4IE_R(crate::FieldReader<bool, bool>);
impl T4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        T4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T4IE` writer - "]
pub struct T4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T4IE_W<'a> {
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
#[doc = "Field `IC4EIE` reader - "]
pub struct IC4EIE_R(crate::FieldReader<bool, bool>);
impl IC4EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC4EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4EIE` writer - "]
pub struct IC4EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4EIE_W<'a> {
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
#[doc = "Field `IC4IE` reader - "]
pub struct IC4IE_R(crate::FieldReader<bool, bool>);
impl IC4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4IE` writer - "]
pub struct IC4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IE_W<'a> {
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
#[doc = "Field `OC4IE` reader - "]
pub struct OC4IE_R(crate::FieldReader<bool, bool>);
impl OC4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4IE` writer - "]
pub struct OC4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4IE_W<'a> {
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
#[doc = "Field `INT4IE` reader - "]
pub struct INT4IE_R(crate::FieldReader<bool, bool>);
impl INT4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4IE` writer - "]
pub struct INT4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4IE_W<'a> {
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
#[doc = "Field `T5IE` reader - "]
pub struct T5IE_R(crate::FieldReader<bool, bool>);
impl T5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        T5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T5IE` writer - "]
pub struct T5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T5IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `IC5EIE` reader - "]
pub struct IC5EIE_R(crate::FieldReader<bool, bool>);
impl IC5EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC5EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5EIE` writer - "]
pub struct IC5EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5EIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `IC5IE` reader - "]
pub struct IC5IE_R(crate::FieldReader<bool, bool>);
impl IC5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5IE` writer - "]
pub struct IC5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `OC5IE` reader - "]
pub struct OC5IE_R(crate::FieldReader<bool, bool>);
impl OC5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5IE` writer - "]
pub struct OC5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `AD1IE` reader - "]
pub struct AD1IE_R(crate::FieldReader<bool, bool>);
impl AD1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1IE` writer - "]
pub struct AD1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `FSCMIE` reader - "]
pub struct FSCMIE_R(crate::FieldReader<bool, bool>);
impl FSCMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSCMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSCMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSCMIE` writer - "]
pub struct FSCMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCMIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `RTCCIE` reader - "]
pub struct RTCCIE_R(crate::FieldReader<bool, bool>);
impl RTCCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCIE` writer - "]
pub struct RTCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `FCEIE` reader - "]
pub struct FCEIE_R(crate::FieldReader<bool, bool>);
impl FCEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCEIE` writer - "]
pub struct FCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctie(&self) -> CTIE_R {
        CTIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs0ie(&self) -> CS0IE_R {
        CS0IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cs1ie(&self) -> CS1IE_R {
        CS1IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int0ie(&self) -> INT0IE_R {
        INT0IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t1ie(&self) -> T1IE_R {
        T1IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ic1eie(&self) -> IC1EIE_R {
        IC1EIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ic1ie(&self) -> IC1IE_R {
        IC1IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1ie(&self) -> OC1IE_R {
        OC1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn int1ie(&self) -> INT1IE_R {
        INT1IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t2ie(&self) -> T2IE_R {
        T2IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ic2eie(&self) -> IC2EIE_R {
        IC2EIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ic2ie(&self) -> IC2IE_R {
        IC2IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oc2ie(&self) -> OC2IE_R {
        OC2IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn int2ie(&self) -> INT2IE_R {
        INT2IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn t3ie(&self) -> T3IE_R {
        T3IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ic3eie(&self) -> IC3EIE_R {
        IC3EIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic3ie(&self) -> IC3IE_R {
        IC3IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc3ie(&self) -> OC3IE_R {
        OC3IE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn int3ie(&self) -> INT3IE_R {
        INT3IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn t4ie(&self) -> T4IE_R {
        T4IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic4eie(&self) -> IC4EIE_R {
        IC4EIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic4ie(&self) -> IC4IE_R {
        IC4IE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn oc4ie(&self) -> OC4IE_R {
        OC4IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn int4ie(&self) -> INT4IE_R {
        INT4IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn t5ie(&self) -> T5IE_R {
        T5IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic5eie(&self) -> IC5EIE_R {
        IC5EIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic5ie(&self) -> IC5IE_R {
        IC5IE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn oc5ie(&self) -> OC5IE_R {
        OC5IE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ad1ie(&self) -> AD1IE_R {
        AD1IE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fscmie(&self) -> FSCMIE_R {
        FSCMIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtccie(&self) -> RTCCIE_R {
        RTCCIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn fceie(&self) -> FCEIE_R {
        FCEIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctie(&mut self) -> CTIE_W {
        CTIE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs0ie(&mut self) -> CS0IE_W {
        CS0IE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cs1ie(&mut self) -> CS1IE_W {
        CS1IE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int0ie(&mut self) -> INT0IE_W {
        INT0IE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t1ie(&mut self) -> T1IE_W {
        T1IE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ic1eie(&mut self) -> IC1EIE_W {
        IC1EIE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ic1ie(&mut self) -> IC1IE_W {
        IC1IE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1ie(&mut self) -> OC1IE_W {
        OC1IE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn int1ie(&mut self) -> INT1IE_W {
        INT1IE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t2ie(&mut self) -> T2IE_W {
        T2IE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ic2eie(&mut self) -> IC2EIE_W {
        IC2EIE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ic2ie(&mut self) -> IC2IE_W {
        IC2IE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oc2ie(&mut self) -> OC2IE_W {
        OC2IE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn int2ie(&mut self) -> INT2IE_W {
        INT2IE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn t3ie(&mut self) -> T3IE_W {
        T3IE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ic3eie(&mut self) -> IC3EIE_W {
        IC3EIE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic3ie(&mut self) -> IC3IE_W {
        IC3IE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc3ie(&mut self) -> OC3IE_W {
        OC3IE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn int3ie(&mut self) -> INT3IE_W {
        INT3IE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn t4ie(&mut self) -> T4IE_W {
        T4IE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic4eie(&mut self) -> IC4EIE_W {
        IC4EIE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic4ie(&mut self) -> IC4IE_W {
        IC4IE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn oc4ie(&mut self) -> OC4IE_W {
        OC4IE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn int4ie(&mut self) -> INT4IE_W {
        INT4IE_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn t5ie(&mut self) -> T5IE_W {
        T5IE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic5eie(&mut self) -> IC5EIE_W {
        IC5EIE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic5ie(&mut self) -> IC5IE_W {
        IC5IE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn oc5ie(&mut self) -> OC5IE_W {
        OC5IE_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ad1ie(&mut self) -> AD1IE_W {
        AD1IE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fscmie(&mut self) -> FSCMIE_W {
        FSCMIE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtccie(&mut self) -> RTCCIE_W {
        RTCCIE_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn fceie(&mut self) -> FCEIE_W {
        FCEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEC0CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec0clr](index.html) module"]
pub struct IEC0CLR_SPEC;
impl crate::RegisterSpec for IEC0CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iec0clr::R](R) reader structure"]
impl crate::Readable for IEC0CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iec0clr::W](W) writer structure"]
impl crate::Writable for IEC0CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEC0CLR to value 0"]
impl crate::Resettable for IEC0CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
