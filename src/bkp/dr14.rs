#[doc = "Reader of register DR14"]
pub type R = crate::R<u32, super::DR14>;
#[doc = "Writer for register DR14"]
pub type W = crate::W<u32, super::DR14>;
#[doc = "Register DR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D14`"]
pub type D14_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D14`"]
pub struct D14_W<'a> {
    w: &'a mut W,
}
impl<'a> D14_W<'a> {
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
    pub fn d14(&self) -> D14_R {
        D14_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d14(&mut self) -> D14_W {
        D14_W { w: self }
    }
}
