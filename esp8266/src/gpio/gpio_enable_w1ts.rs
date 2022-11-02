#[doc = "Register `GPIO_ENABLE_W1TS` writer"]
pub struct W(crate::W<GPIO_ENABLE_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_ENABLE_W1TS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO_ENABLE_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_ENABLE_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_ENABLE_DATA_W1TS` writer - Writing 1 into a bit in this register will set the related bit in GPIO_ENABLE_DATA"]
pub type GPIO_ENABLE_DATA_W1TS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_ENABLE_W1TS_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Writing 1 into a bit in this register will set the related bit in GPIO_ENABLE_DATA"]
    #[inline(always)]
    pub fn gpio_enable_data_w1ts(&mut self) -> GPIO_ENABLE_DATA_W1TS_W<0> {
        GPIO_ENABLE_DATA_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_ENABLE_W1TS\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_enable_w1ts](index.html) module"]
pub struct GPIO_ENABLE_W1TS_SPEC;
impl crate::RegisterSpec for GPIO_ENABLE_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_enable_w1ts::W](W) writer structure"]
impl crate::Writable for GPIO_ENABLE_W1TS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_ENABLE_W1TS to value 0"]
impl crate::Resettable for GPIO_ENABLE_W1TS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}