#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
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
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PCM_CONF` reader - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
pub type TX_PCM_CONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PCM_CONF` writer - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
pub type TX_PCM_CONF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF1_SPEC, u8, u8, 3, O>;
#[doc = "Field `TX_PCM_BYPASS` reader - Set this bit to bypass Compress/Decompress module for transmitted data."]
pub type TX_PCM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `TX_PCM_BYPASS` writer - Set this bit to bypass Compress/Decompress module for transmitted data."]
pub type TX_PCM_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `RX_PCM_CONF` reader - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
pub type RX_PCM_CONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_PCM_CONF` writer - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
pub type RX_PCM_CONF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF1_SPEC, u8, u8, 3, O>;
#[doc = "Field `RX_PCM_BYPASS` reader - Set this bit to bypass Compress/Decompress module for received data."]
pub type RX_PCM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `RX_PCM_BYPASS` writer - Set this bit to bypass Compress/Decompress module for received data."]
pub type RX_PCM_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `TX_STOP_EN` reader - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
pub type TX_STOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_STOP_EN` writer - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
pub type TX_STOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `TX_ZEROS_RM_EN` reader - Reserved."]
pub type TX_ZEROS_RM_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_ZEROS_RM_EN` writer - Reserved."]
pub type TX_ZEROS_RM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
    #[inline(always)]
    pub fn tx_pcm_conf(&self) -> TX_PCM_CONF_R {
        TX_PCM_CONF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Set this bit to bypass Compress/Decompress module for transmitted data."]
    #[inline(always)]
    pub fn tx_pcm_bypass(&self) -> TX_PCM_BYPASS_R {
        TX_PCM_BYPASS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> RX_PCM_CONF_R {
        RX_PCM_CONF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> RX_PCM_BYPASS_R {
        RX_PCM_BYPASS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
    #[inline(always)]
    pub fn tx_stop_en(&self) -> TX_STOP_EN_R {
        TX_STOP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved."]
    #[inline(always)]
    pub fn tx_zeros_rm_en(&self) -> TX_ZEROS_RM_EN_R {
        TX_ZEROS_RM_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compress/Decompress module configuration bits. 0: decompress transmitted data 1:compress transmitted data"]
    #[inline(always)]
    pub fn tx_pcm_conf(&mut self) -> TX_PCM_CONF_W<0> {
        TX_PCM_CONF_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to bypass Compress/Decompress module for transmitted data."]
    #[inline(always)]
    pub fn tx_pcm_bypass(&mut self) -> TX_PCM_BYPASS_W<3> {
        TX_PCM_BYPASS_W::new(self)
    }
    #[doc = "Bits 4:6 - Compress/Decompress module configuration bits. 0: decompress received data 1:compress received data"]
    #[inline(always)]
    pub fn rx_pcm_conf(&mut self) -> RX_PCM_CONF_W<4> {
        RX_PCM_CONF_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    pub fn rx_pcm_bypass(&mut self) -> RX_PCM_BYPASS_W<7> {
        RX_PCM_BYPASS_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to stop the output of BCK signal and WS signal when TX FIFO is empty."]
    #[inline(always)]
    pub fn tx_stop_en(&mut self) -> TX_STOP_EN_W<8> {
        TX_STOP_EN_W::new(self)
    }
    #[doc = "Bit 9 - Reserved."]
    #[inline(always)]
    pub fn tx_zeros_rm_en(&mut self) -> TX_ZEROS_RM_EN_W<9> {
        TX_ZEROS_RM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R](R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF1 to value 0x89"]
impl crate::Resettable for CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x89
    }
}