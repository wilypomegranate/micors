#[doc = "Reader of register DR17"]
pub type R = crate::R<u32, super::DR17>;
#[doc = "Writer for register DR17"]
pub type W = crate::W<u32, super::DR17>;
#[doc = "Register DR17 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D17`"]
pub type D17_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D17`"]
pub struct D17_W<'a> {
    w: &'a mut W,
}
impl<'a> D17_W<'a> {
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
    pub fn d17(&self) -> D17_R {
        D17_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d17(&mut self) -> D17_W {
        D17_W { w: self }
    }
}
