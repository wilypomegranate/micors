#[doc = "Reader of register DR35"]
pub type R = crate::R<u32, super::DR35>;
#[doc = "Writer for register DR35"]
pub type W = crate::W<u32, super::DR35>;
#[doc = "Register DR35 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR35 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D35`"]
pub type D35_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D35`"]
pub struct D35_W<'a> {
    w: &'a mut W,
}
impl<'a> D35_W<'a> {
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
    pub fn d35(&self) -> D35_R {
        D35_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d35(&mut self) -> D35_W {
        D35_W { w: self }
    }
}
