#[doc = "Reader of register CRH"]
pub type R = crate::R<u32, super::CRH>;
#[doc = "Writer for register CRH"]
pub type W = crate::W<u32, super::CRH>;
#[doc = "Register CRH `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::CRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Reader of field `MODE8`"]
pub type MODE8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE8`"]
pub struct MODE8_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CNF8`"]
pub type CNF8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF8`"]
pub struct CNF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MODE9`"]
pub type MODE9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE9`"]
pub struct MODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CNF9`"]
pub type CNF9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF9`"]
pub struct CNF9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MODE10`"]
pub type MODE10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE10`"]
pub struct MODE10_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CNF10`"]
pub type CNF10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF10`"]
pub struct CNF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MODE11`"]
pub type MODE11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE11`"]
pub struct MODE11_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CNF11`"]
pub type CNF11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF11`"]
pub struct CNF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MODE12`"]
pub type MODE12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE12`"]
pub struct MODE12_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNF12`"]
pub type CNF12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF12`"]
pub struct CNF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `MODE13`"]
pub type MODE13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE13`"]
pub struct MODE13_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CNF13`"]
pub type CNF13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF13`"]
pub struct CNF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `MODE14`"]
pub type MODE14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE14`"]
pub struct MODE14_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CNF14`"]
pub type CNF14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF14`"]
pub struct CNF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `MODE15`"]
pub type MODE15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE15`"]
pub struct MODE15_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CNF15`"]
pub type CNF15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF15`"]
pub struct CNF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&self) -> CNF8_R {
        CNF8_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&self) -> CNF9_R {
        CNF9_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&self) -> CNF10_R {
        CNF10_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&self) -> CNF11_R {
        CNF11_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&self) -> CNF12_R {
        CNF12_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&self) -> CNF13_R {
        CNF13_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&self) -> CNF14_R {
        CNF14_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&self) -> CNF15_R {
        CNF15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.8 mode bits"]
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W {
        MODE8_W { w: self }
    }
    #[doc = "Bits 2:3 - Port n.8 configuration bits"]
    #[inline(always)]
    pub fn cnf8(&mut self) -> CNF8_W {
        CNF8_W { w: self }
    }
    #[doc = "Bits 4:5 - Port n.9 mode bits"]
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W { w: self }
    }
    #[doc = "Bits 6:7 - Port n.9 configuration bits"]
    #[inline(always)]
    pub fn cnf9(&mut self) -> CNF9_W {
        CNF9_W { w: self }
    }
    #[doc = "Bits 8:9 - Port n.10 mode bits"]
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W {
        MODE10_W { w: self }
    }
    #[doc = "Bits 10:11 - Port n.10 configuration bits"]
    #[inline(always)]
    pub fn cnf10(&mut self) -> CNF10_W {
        CNF10_W { w: self }
    }
    #[doc = "Bits 12:13 - Port n.11 mode bits"]
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W {
        MODE11_W { w: self }
    }
    #[doc = "Bits 14:15 - Port n.11 configuration bits"]
    #[inline(always)]
    pub fn cnf11(&mut self) -> CNF11_W {
        CNF11_W { w: self }
    }
    #[doc = "Bits 16:17 - Port n.12 mode bits"]
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W {
        MODE12_W { w: self }
    }
    #[doc = "Bits 18:19 - Port n.12 configuration bits"]
    #[inline(always)]
    pub fn cnf12(&mut self) -> CNF12_W {
        CNF12_W { w: self }
    }
    #[doc = "Bits 20:21 - Port n.13 mode bits"]
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W {
        MODE13_W { w: self }
    }
    #[doc = "Bits 22:23 - Port n.13 configuration bits"]
    #[inline(always)]
    pub fn cnf13(&mut self) -> CNF13_W {
        CNF13_W { w: self }
    }
    #[doc = "Bits 24:25 - Port n.14 mode bits"]
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W {
        MODE14_W { w: self }
    }
    #[doc = "Bits 26:27 - Port n.14 configuration bits"]
    #[inline(always)]
    pub fn cnf14(&mut self) -> CNF14_W {
        CNF14_W { w: self }
    }
    #[doc = "Bits 28:29 - Port n.15 mode bits"]
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W {
        MODE15_W { w: self }
    }
    #[doc = "Bits 30:31 - Port n.15 configuration bits"]
    #[inline(always)]
    pub fn cnf15(&mut self) -> CNF15_W {
        CNF15_W { w: self }
    }
}
