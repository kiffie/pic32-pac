#[doc = "Reader of register U1EIR"]
pub type R = crate::R<u32, super::U1EIR>;
#[doc = "Writer for register U1EIR"]
pub type W = crate::W<u32, super::U1EIR>;
#[doc = "Register U1EIR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1EIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIDEF`"]
pub type PIDEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIDEF`"]
pub struct PIDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDEF_W<'a> {
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
#[doc = "Reader of field `CRC5EF_EOFEF`"]
pub type CRC5EF_EOFEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC5EF_EOFEF`"]
pub struct CRC5EF_EOFEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5EF_EOFEF_W<'a> {
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
#[doc = "Reader of field `CRC16EF`"]
pub type CRC16EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC16EF`"]
pub struct CRC16EF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16EF_W<'a> {
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
#[doc = "Reader of field `DFN8EF`"]
pub type DFN8EF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFN8EF`"]
pub struct DFN8EF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFN8EF_W<'a> {
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
#[doc = "Reader of field `BTOEF`"]
pub type BTOEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTOEF`"]
pub struct BTOEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BTOEF_W<'a> {
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
#[doc = "Reader of field `DMAEF`"]
pub type DMAEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEF`"]
pub struct DMAEF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEF_W<'a> {
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
#[doc = "Reader of field `BMXEF`"]
pub type BMXEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXEF`"]
pub struct BMXEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXEF_W<'a> {
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
#[doc = "Reader of field `BTSEF`"]
pub type BTSEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTSEF`"]
pub struct BTSEF_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSEF_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pidef(&self) -> PIDEF_R {
        PIDEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ef_eofef(&self) -> CRC5EF_EOFEF_R {
        CRC5EF_EOFEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ef(&self) -> CRC16EF_R {
        CRC16EF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ef(&self) -> DFN8EF_R {
        DFN8EF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoef(&self) -> BTOEF_R {
        BTOEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaef(&self) -> DMAEF_R {
        DMAEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxef(&self) -> BMXEF_R {
        BMXEF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsef(&self) -> BTSEF_R {
        BTSEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pidef(&mut self) -> PIDEF_W {
        PIDEF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ef_eofef(&mut self) -> CRC5EF_EOFEF_W {
        CRC5EF_EOFEF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ef(&mut self) -> CRC16EF_W {
        CRC16EF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ef(&mut self) -> DFN8EF_W {
        DFN8EF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoef(&mut self) -> BTOEF_W {
        BTOEF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaef(&mut self) -> DMAEF_W {
        DMAEF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxef(&mut self) -> BMXEF_W {
        BMXEF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsef(&mut self) -> BTSEF_W {
        BTSEF_W { w: self }
    }
}
