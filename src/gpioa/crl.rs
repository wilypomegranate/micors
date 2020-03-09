#[doc = "Reader of register CRL"]
pub type R = crate::R<u32, super::CRL>;
#[doc = "Writer for register CRL"]
pub type W = crate::W<u32, super::CRL>;
#[doc = "Register CRL `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::CRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Reader of field `MODE0`"]
pub type MODE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE0`"]
pub struct MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CNF0`"]
pub type CNF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF0`"]
pub struct CNF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MODE1`"]
pub type MODE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE1`"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CNF1`"]
pub type CNF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF1`"]
pub struct CNF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MODE2`"]
pub type MODE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE2`"]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CNF2`"]
pub type CNF2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF2`"]
pub struct CNF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MODE3`"]
pub type MODE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE3`"]
pub struct MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CNF3`"]
pub type CNF3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF3`"]
pub struct CNF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MODE4`"]
pub type MODE4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE4`"]
pub struct MODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNF4`"]
pub type CNF4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF4`"]
pub struct CNF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `MODE5`"]
pub type MODE5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE5`"]
pub struct MODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CNF5`"]
pub type CNF5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF5`"]
pub struct CNF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `MODE6`"]
pub type MODE6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE6`"]
pub struct MODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CNF6`"]
pub type CNF6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF6`"]
pub struct CNF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `MODE7`"]
pub type MODE7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE7`"]
pub struct MODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CNF7`"]
pub type CNF7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNF7`"]
pub struct CNF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNF7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&self) -> CNF0_R {
        CNF0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&self) -> CNF1_R {
        CNF1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&self) -> CNF2_R {
        CNF2_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&self) -> CNF3_R {
        CNF3_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&self) -> CNF4_R {
        CNF4_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&self) -> CNF5_R {
        CNF5_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&self) -> CNF6_R {
        CNF6_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&self) -> CNF7_R {
        CNF7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.0 mode bits"]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W { w: self }
    }
    #[doc = "Bits 2:3 - Port n.0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&mut self) -> CNF0_W {
        CNF0_W { w: self }
    }
    #[doc = "Bits 4:5 - Port n.1 mode bits"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bits 6:7 - Port n.1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&mut self) -> CNF1_W {
        CNF1_W { w: self }
    }
    #[doc = "Bits 8:9 - Port n.2 mode bits"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bits 10:11 - Port n.2 configuration bits"]
    #[inline(always)]
    pub fn cnf2(&mut self) -> CNF2_W {
        CNF2_W { w: self }
    }
    #[doc = "Bits 12:13 - Port n.3 mode bits"]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    #[doc = "Bits 14:15 - Port n.3 configuration bits"]
    #[inline(always)]
    pub fn cnf3(&mut self) -> CNF3_W {
        CNF3_W { w: self }
    }
    #[doc = "Bits 16:17 - Port n.4 mode bits"]
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W {
        MODE4_W { w: self }
    }
    #[doc = "Bits 18:19 - Port n.4 configuration bits"]
    #[inline(always)]
    pub fn cnf4(&mut self) -> CNF4_W {
        CNF4_W { w: self }
    }
    #[doc = "Bits 20:21 - Port n.5 mode bits"]
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W {
        MODE5_W { w: self }
    }
    #[doc = "Bits 22:23 - Port n.5 configuration bits"]
    #[inline(always)]
    pub fn cnf5(&mut self) -> CNF5_W {
        CNF5_W { w: self }
    }
    #[doc = "Bits 24:25 - Port n.6 mode bits"]
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W {
        MODE6_W { w: self }
    }
    #[doc = "Bits 26:27 - Port n.6 configuration bits"]
    #[inline(always)]
    pub fn cnf6(&mut self) -> CNF6_W {
        CNF6_W { w: self }
    }
    #[doc = "Bits 28:29 - Port n.7 mode bits"]
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W {
        MODE7_W { w: self }
    }
    #[doc = "Bits 30:31 - Port n.7 configuration bits"]
    #[inline(always)]
    pub fn cnf7(&mut self) -> CNF7_W {
        CNF7_W { w: self }
    }
}
