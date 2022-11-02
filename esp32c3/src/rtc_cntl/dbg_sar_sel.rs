#[doc = "Register `DBG_SAR_SEL` reader"]
pub struct R(crate::R<DBG_SAR_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_SAR_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_SAR_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_SAR_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_SAR_SEL` writer"]
pub struct W(crate::W<DBG_SAR_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_SAR_SEL_SPEC>;
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
impl From<crate::W<DBG_SAR_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_SAR_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_DEBUG_SEL` reader - use for debug"]
pub type SAR_DEBUG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_DEBUG_SEL` writer - use for debug"]
pub type SAR_DEBUG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SAR_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 27:31 - use for debug"]
    #[inline(always)]
    pub fn sar_debug_sel(&self) -> SAR_DEBUG_SEL_R {
        SAR_DEBUG_SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - use for debug"]
    #[inline(always)]
    pub fn sar_debug_sel(&mut self) -> SAR_DEBUG_SEL_W<27> {
        SAR_DEBUG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sar_sel](index.html) module"]
pub struct DBG_SAR_SEL_SPEC;
impl crate::RegisterSpec for DBG_SAR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_sar_sel::R](R) reader structure"]
impl crate::Readable for DBG_SAR_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_sar_sel::W](W) writer structure"]
impl crate::Writable for DBG_SAR_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_SAR_SEL to value 0"]
impl crate::Resettable for DBG_SAR_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}