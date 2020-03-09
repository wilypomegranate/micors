#[doc = "Writer for register BRR"]
pub type W = crate::W<u32, super::BRR>;
#[doc = "Register BRR `reset()`'s with value 0"]
impl crate::ResetValue for super::BRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `BR0`"]
pub struct BR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BR0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `BR1`"]
pub struct BR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `BR2`"]
pub struct BR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BR2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `BR3`"]
pub struct BR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BR3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `BR4`"]
pub struct BR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BR4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `BR5`"]
pub struct BR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BR5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `BR6`"]
pub struct BR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BR6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `BR7`"]
pub struct BR7_W<'a> {
    w: &'a mut W,
}
impl<'a> BR7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `BR8`"]
pub struct BR8_W<'a> {
    w: &'a mut W,
}
impl<'a> BR8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `BR9`"]
pub struct BR9_W<'a> {
    w: &'a mut W,
}
impl<'a> BR9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `BR10`"]
pub struct BR10_W<'a> {
    w: &'a mut W,
}
impl<'a> BR10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `BR11`"]
pub struct BR11_W<'a> {
    w: &'a mut W,
}
impl<'a> BR11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `BR12`"]
pub struct BR12_W<'a> {
    w: &'a mut W,
}
impl<'a> BR12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `BR13`"]
pub struct BR13_W<'a> {
    w: &'a mut W,
}
impl<'a> BR13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `BR14`"]
pub struct BR14_W<'a> {
    w: &'a mut W,
}
impl<'a> BR14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `BR15`"]
pub struct BR15_W<'a> {
    w: &'a mut W,
}
impl<'a> BR15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Reset bit 0"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W {
        BR0_W { w: self }
    }
    #[doc = "Bit 1 - Reset bit 1"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W {
        BR1_W { w: self }
    }
    #[doc = "Bit 2 - Reset bit 1"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W {
        BR2_W { w: self }
    }
    #[doc = "Bit 3 - Reset bit 3"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W {
        BR3_W { w: self }
    }
    #[doc = "Bit 4 - Reset bit 4"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W {
        BR4_W { w: self }
    }
    #[doc = "Bit 5 - Reset bit 5"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W {
        BR5_W { w: self }
    }
    #[doc = "Bit 6 - Reset bit 6"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W {
        BR6_W { w: self }
    }
    #[doc = "Bit 7 - Reset bit 7"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W {
        BR7_W { w: self }
    }
    #[doc = "Bit 8 - Reset bit 8"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W {
        BR8_W { w: self }
    }
    #[doc = "Bit 9 - Reset bit 9"]
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W {
        BR9_W { w: self }
    }
    #[doc = "Bit 10 - Reset bit 10"]
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W {
        BR10_W { w: self }
    }
    #[doc = "Bit 11 - Reset bit 11"]
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W {
        BR11_W { w: self }
    }
    #[doc = "Bit 12 - Reset bit 12"]
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W {
        BR12_W { w: self }
    }
    #[doc = "Bit 13 - Reset bit 13"]
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W {
        BR13_W { w: self }
    }
    #[doc = "Bit 14 - Reset bit 14"]
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W {
        BR14_W { w: self }
    }
    #[doc = "Bit 15 - Reset bit 15"]
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W {
        BR15_W { w: self }
    }
}
