#[doc = "Reader of register AHBENR"]
pub type R = crate::R<u32, super::AHBENR>;
#[doc = "Writer for register AHBENR"]
pub type W = crate::W<u32, super::AHBENR>;
#[doc = "Register AHBENR `reset()`'s with value 0x14"]
impl crate::ResetValue for super::AHBENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x14
    }
}
#[doc = "Reader of field `DMA1EN`"]
pub type DMA1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1EN`"]
pub struct DMA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1EN_W<'a> {
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
#[doc = "Reader of field `DMA2EN`"]
pub type DMA2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2EN`"]
pub struct DMA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2EN_W<'a> {
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
#[doc = "Reader of field `SRAMEN`"]
pub type SRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMEN`"]
pub struct SRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMEN_W<'a> {
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
#[doc = "Reader of field `FLITFEN`"]
pub type FLITFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLITFEN`"]
pub struct FLITFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITFEN_W<'a> {
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
#[doc = "Reader of field `CRCEN`"]
pub type CRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEN`"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
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
#[doc = "Reader of field `FSMCEN`"]
pub type FSMCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSMCEN`"]
pub struct FSMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCEN_W<'a> {
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
#[doc = "Reader of field `SDIOEN`"]
pub type SDIOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIOEN`"]
pub struct SDIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W { w: self }
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W { w: self }
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W {
        FLITFEN_W { w: self }
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W {
        FSMCEN_W { w: self }
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W {
        SDIOEN_W { w: self }
    }
}
