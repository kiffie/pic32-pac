#[doc = "Reader of register CTMUCON"]
pub type R = crate::R<u32, super::CTMUCON>;
#[doc = "Writer for register CTMUCON"]
pub type W = crate::W<u32, super::CTMUCON>;
#[doc = "Register CTMUCON `reset()`'s with value 0"]
impl crate::ResetValue for super::CTMUCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRNG`"]
pub type IRNG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRNG`"]
pub struct IRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> IRNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ITRIM`"]
pub type ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITRIM`"]
pub struct ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `CTTRIG`"]
pub type CTTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTTRIG`"]
pub struct CTTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTRIG_W<'a> {
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
#[doc = "Reader of field `IDISSEN`"]
pub type IDISSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDISSEN`"]
pub struct IDISSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDISSEN_W<'a> {
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
#[doc = "Reader of field `EDGSEQEN`"]
pub type EDGSEQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGSEQEN`"]
pub struct EDGSEQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGSEQEN_W<'a> {
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
#[doc = "Reader of field `EDGEN`"]
pub type EDGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGEN`"]
pub struct EDGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGEN_W<'a> {
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
#[doc = "Reader of field `TGEN`"]
pub type TGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TGEN`"]
pub struct TGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TGEN_W<'a> {
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
#[doc = "Reader of field `CTMUSIDL`"]
pub type CTMUSIDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMUSIDL`"]
pub struct CTMUSIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUSIDL_W<'a> {
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
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRZ`"]
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
#[doc = "Reader of field `ON`"]
pub type ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ON`"]
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
#[doc = "Reader of field `EDG2SEL`"]
pub type EDG2SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDG2SEL`"]
pub struct EDG2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `EDG2POL`"]
pub type EDG2POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDG2POL`"]
pub struct EDG2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2POL_W<'a> {
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
#[doc = "Reader of field `EDG2MOD`"]
pub type EDG2MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDG2MOD`"]
pub struct EDG2MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2MOD_W<'a> {
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
#[doc = "Reader of field `EDG1STAT`"]
pub type EDG1STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDG1STAT`"]
pub struct EDG1STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1STAT_W<'a> {
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
#[doc = "Reader of field `EDG2STAT`"]
pub type EDG2STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDG2STAT`"]
pub struct EDG2STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2STAT_W<'a> {
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
#[doc = "Reader of field `EDG1SEL`"]
pub type EDG1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDG1SEL`"]
pub struct EDG1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
#[doc = "Reader of field `EDG1POL`"]
pub type EDG1POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDG1POL`"]
pub struct EDG1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1POL_W<'a> {
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
#[doc = "Reader of field `EDG1MOD`"]
pub type EDG1MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDG1MOD`"]
pub struct EDG1MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1MOD_W<'a> {
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
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn irng(&self) -> IRNG_R {
        IRNG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cttrig(&self) -> CTTRIG_R {
        CTTRIG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idissen(&self) -> IDISSEN_R {
        IDISSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn edgseqen(&self) -> EDGSEQEN_R {
        EDGSEQEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn edgen(&self) -> EDGEN_R {
        EDGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tgen(&self) -> TGEN_R {
        TGEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ctmusidl(&self) -> CTMUSIDL_R {
        CTMUSIDL_R::new(((self.bits >> 13) & 0x01) != 0)
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
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn edg2sel(&self) -> EDG2SEL_R {
        EDG2SEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn edg2pol(&self) -> EDG2POL_R {
        EDG2POL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn edg2mod(&self) -> EDG2MOD_R {
        EDG2MOD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn edg1stat(&self) -> EDG1STAT_R {
        EDG1STAT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn edg2stat(&self) -> EDG2STAT_R {
        EDG2STAT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn edg1sel(&self) -> EDG1SEL_R {
        EDG1SEL_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn edg1pol(&self) -> EDG1POL_R {
        EDG1POL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn edg1mod(&self) -> EDG1MOD_R {
        EDG1MOD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn irng(&mut self) -> IRNG_W {
        IRNG_W { w: self }
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn itrim(&mut self) -> ITRIM_W {
        ITRIM_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cttrig(&mut self) -> CTTRIG_W {
        CTTRIG_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idissen(&mut self) -> IDISSEN_W {
        IDISSEN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn edgseqen(&mut self) -> EDGSEQEN_W {
        EDGSEQEN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn edgen(&mut self) -> EDGEN_W {
        EDGEN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tgen(&mut self) -> TGEN_W {
        TGEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ctmusidl(&mut self) -> CTMUSIDL_W {
        CTMUSIDL_W { w: self }
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
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn edg2sel(&mut self) -> EDG2SEL_W {
        EDG2SEL_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn edg2pol(&mut self) -> EDG2POL_W {
        EDG2POL_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn edg2mod(&mut self) -> EDG2MOD_W {
        EDG2MOD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn edg1stat(&mut self) -> EDG1STAT_W {
        EDG1STAT_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn edg2stat(&mut self) -> EDG2STAT_W {
        EDG2STAT_W { w: self }
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn edg1sel(&mut self) -> EDG1SEL_W {
        EDG1SEL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn edg1pol(&mut self) -> EDG1POL_W {
        EDG1POL_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn edg1mod(&mut self) -> EDG1MOD_W {
        EDG1MOD_W { w: self }
    }
}
