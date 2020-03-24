#[doc = "Reader of register U1EIESET"]
pub type R = crate::R<u32, super::U1EIESET>;
#[doc = "Writer for register U1EIESET"]
pub type W = crate::W<u32, super::U1EIESET>;
#[doc = "Register U1EIESET `reset()`'s with value 0"]
impl crate::ResetValue for super::U1EIESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIDEE`"]
pub type PIDEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIDEE`"]
pub struct PIDEE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDEE_W<'a> {
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
#[doc = "Reader of field `CRC5EE_EOFEE`"]
pub type CRC5EE_EOFEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC5EE_EOFEE`"]
pub struct CRC5EE_EOFEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5EE_EOFEE_W<'a> {
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
#[doc = "Reader of field `CRC16EE`"]
pub type CRC16EE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC16EE`"]
pub struct CRC16EE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16EE_W<'a> {
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
#[doc = "Reader of field `DFN8EE`"]
pub type DFN8EE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFN8EE`"]
pub struct DFN8EE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFN8EE_W<'a> {
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
#[doc = "Reader of field `BTOEE`"]
pub type BTOEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTOEE`"]
pub struct BTOEE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTOEE_W<'a> {
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
#[doc = "Reader of field `DMAEE`"]
pub type DMAEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEE`"]
pub struct DMAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEE_W<'a> {
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
#[doc = "Reader of field `BMXEE`"]
pub type BMXEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXEE`"]
pub struct BMXEE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXEE_W<'a> {
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
#[doc = "Reader of field `BTSEE`"]
pub type BTSEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTSEE`"]
pub struct BTSEE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSEE_W<'a> {
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
    pub fn pidee(&self) -> PIDEE_R {
        PIDEE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ee_eofee(&self) -> CRC5EE_EOFEE_R {
        CRC5EE_EOFEE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ee(&self) -> CRC16EE_R {
        CRC16EE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ee(&self) -> DFN8EE_R {
        DFN8EE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoee(&self) -> BTOEE_R {
        BTOEE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaee(&self) -> DMAEE_R {
        DMAEE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxee(&self) -> BMXEE_R {
        BMXEE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsee(&self) -> BTSEE_R {
        BTSEE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pidee(&mut self) -> PIDEE_W {
        PIDEE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc5ee_eofee(&mut self) -> CRC5EE_EOFEE_W {
        CRC5EE_EOFEE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc16ee(&mut self) -> CRC16EE_W {
        CRC16EE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dfn8ee(&mut self) -> DFN8EE_W {
        DFN8EE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btoee(&mut self) -> BTOEE_W {
        BTOEE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaee(&mut self) -> DMAEE_W {
        DMAEE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxee(&mut self) -> BMXEE_W {
        BMXEE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn btsee(&mut self) -> BTSEE_W {
        BTSEE_W { w: self }
    }
}
