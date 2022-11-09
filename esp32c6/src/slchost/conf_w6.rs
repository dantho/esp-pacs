#[doc = "Register `CONF_W6` reader"]
pub struct R(crate::R<CONF_W6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W6` writer"]
pub struct W(crate::W<CONF_W6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W6_SPEC>;
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
impl From<crate::W<CONF_W6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF24` reader - *******Description***********"]
pub type SLCHOST_CONF24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLCHOST_CONF24` writer - *******Description***********"]
pub type SLCHOST_CONF24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF_W6_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLCHOST_CONF25` reader - *******Description***********"]
pub type SLCHOST_CONF25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLCHOST_CONF25` writer - *******Description***********"]
pub type SLCHOST_CONF25_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF_W6_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLCHOST_CONF26` reader - *******Description***********"]
pub type SLCHOST_CONF26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLCHOST_CONF26` writer - *******Description***********"]
pub type SLCHOST_CONF26_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF_W6_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLCHOST_CONF27` reader - *******Description***********"]
pub type SLCHOST_CONF27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLCHOST_CONF27` writer - *******Description***********"]
pub type SLCHOST_CONF27_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF_W6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf24(&self) -> SLCHOST_CONF24_R {
        SLCHOST_CONF24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf25(&self) -> SLCHOST_CONF25_R {
        SLCHOST_CONF25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf26(&self) -> SLCHOST_CONF26_R {
        SLCHOST_CONF26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf27(&self) -> SLCHOST_CONF27_R {
        SLCHOST_CONF27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf24(&mut self) -> SLCHOST_CONF24_W<0> {
        SLCHOST_CONF24_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf25(&mut self) -> SLCHOST_CONF25_W<8> {
        SLCHOST_CONF25_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf26(&mut self) -> SLCHOST_CONF26_W<16> {
        SLCHOST_CONF26_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf27(&mut self) -> SLCHOST_CONF27_W<24> {
        SLCHOST_CONF27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w6](index.html) module"]
pub struct CONF_W6_SPEC;
impl crate::RegisterSpec for CONF_W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w6::R](R) reader structure"]
impl crate::Readable for CONF_W6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w6::W](W) writer structure"]
impl crate::Writable for CONF_W6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W6 to value 0"]
impl crate::Resettable for CONF_W6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}