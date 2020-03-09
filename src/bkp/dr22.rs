#[doc = "Reader of register DR22"]
pub type R = crate::R<u32, super::DR22>;
#[doc = "Writer for register DR22"]
pub type W = crate::W<u32, super::DR22>;
#[doc = "Register DR22 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D22`"]
pub type D22_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D22`"]
pub struct D22_W<'a> {
    w: &'a mut W,
}
impl<'a> D22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d22(&self) -> D22_R {
        D22_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d22(&mut self) -> D22_W {
        D22_W { w: self }
    }
}
