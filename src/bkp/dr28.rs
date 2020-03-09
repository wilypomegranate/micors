#[doc = "Reader of register DR28"]
pub type R = crate::R<u32, super::DR28>;
#[doc = "Writer for register DR28"]
pub type W = crate::W<u32, super::DR28>;
#[doc = "Register DR28 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR28 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D28`"]
pub type D28_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D28`"]
pub struct D28_W<'a> {
    w: &'a mut W,
}
impl<'a> D28_W<'a> {
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
    pub fn d28(&self) -> D28_R {
        D28_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d28(&mut self) -> D28_W {
        D28_W { w: self }
    }
}
