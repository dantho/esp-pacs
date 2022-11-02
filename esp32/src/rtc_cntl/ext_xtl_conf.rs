#[doc = "Register `EXT_XTL_CONF` reader"]
pub struct R(crate::R<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_XTL_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_XTL_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_XTL_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_XTL_CONF` writer"]
pub struct W(crate::W<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_XTL_CONF_SPEC>;
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
impl From<crate::W<EXT_XTL_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_XTL_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTL_EXT_CTR_LV` reader - 0: power down XTAL at high level 1: power down XTAL at low level"]
pub type XTL_EXT_CTR_LV_R = crate::BitReader<bool>;
#[doc = "Field `XTL_EXT_CTR_LV` writer - 0: power down XTAL at high level 1: power down XTAL at low level"]
pub type XTL_EXT_CTR_LV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
#[doc = "Field `XTL_EXT_CTR_EN` reader - enable control XTAL by external pads"]
pub type XTL_EXT_CTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `XTL_EXT_CTR_EN` writer - enable control XTAL by external pads"]
pub type XTL_EXT_CTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_XTL_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - 0: power down XTAL at high level 1: power down XTAL at low level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable control XTAL by external pads"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - 0: power down XTAL at high level 1: power down XTAL at low level"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W<30> {
        XTL_EXT_CTR_LV_W::new(self)
    }
    #[doc = "Bit 31 - enable control XTAL by external pads"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W<31> {
        XTL_EXT_CTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_xtl_conf](index.html) module"]
pub struct EXT_XTL_CONF_SPEC;
impl crate::RegisterSpec for EXT_XTL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_xtl_conf::R](R) reader structure"]
impl crate::Readable for EXT_XTL_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_xtl_conf::W](W) writer structure"]
impl crate::Writable for EXT_XTL_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_XTL_CONF to value 0"]
impl crate::Resettable for EXT_XTL_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}