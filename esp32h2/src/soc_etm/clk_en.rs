#[doc = "Register `CLK_EN` reader"]
pub struct R(crate::R<CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_EN` writer"]
pub struct W(crate::W<CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_EN_SPEC>;
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
impl From<crate::W<CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - clock enable"]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - clock enable"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - clock enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "etm clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_en](index.html) module"]
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_en::R](R) reader structure"]
impl crate::Readable for CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_en::W](W) writer structure"]
impl crate::Writable for CLK_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_EN to value 0"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}