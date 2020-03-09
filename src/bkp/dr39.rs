#[doc = "Reader of register DR39"]
pub type R = crate::R<u32, super::DR39>;
#[doc = "Writer for register DR39"]
pub type W = crate::W<u32, super::DR39>;
#[doc = "Register DR39 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR39 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D39`"]
pub type D39_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D39`"]
pub struct D39_W<'a> {
    w: &'a mut W,
}
impl<'a> D39_W<'a> {
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
    pub fn d39(&self) -> D39_R {
        D39_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d39(&mut self) -> D39_W {
        D39_W { w: self }
    }
}
