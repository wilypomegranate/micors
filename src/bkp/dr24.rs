#[doc = "Reader of register DR24"]
pub type R = crate::R<u32, super::DR24>;
#[doc = "Writer for register DR24"]
pub type W = crate::W<u32, super::DR24>;
#[doc = "Register DR24 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D24`"]
pub type D24_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D24`"]
pub struct D24_W<'a> {
    w: &'a mut W,
}
impl<'a> D24_W<'a> {
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
    pub fn d24(&self) -> D24_R {
        D24_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d24(&mut self) -> D24_W {
        D24_W { w: self }
    }
}
