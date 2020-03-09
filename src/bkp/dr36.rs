#[doc = "Reader of register DR36"]
pub type R = crate::R<u32, super::DR36>;
#[doc = "Writer for register DR36"]
pub type W = crate::W<u32, super::DR36>;
#[doc = "Register DR36 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR36 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D36`"]
pub type D36_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D36`"]
pub struct D36_W<'a> {
    w: &'a mut W,
}
impl<'a> D36_W<'a> {
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
    pub fn d36(&self) -> D36_R {
        D36_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d36(&mut self) -> D36_W {
        D36_W { w: self }
    }
}
