#[doc = "Reader of register DR5"]
pub type R = crate::R<u32, super::DR5>;
#[doc = "Writer for register DR5"]
pub type W = crate::W<u32, super::DR5>;
#[doc = "Register DR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D5`"]
pub type D5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D5`"]
pub struct D5_W<'a> {
    w: &'a mut W,
}
impl<'a> D5_W<'a> {
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
    pub fn d5(&self) -> D5_R {
        D5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d5(&mut self) -> D5_W {
        D5_W { w: self }
    }
}
