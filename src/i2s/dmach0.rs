#[doc = "Register `DMACH0` reader"]
pub type R = crate::R<DMACH0_SPEC>;
#[doc = "Register `DMACH0` writer"]
pub type W = crate::W<DMACH0_SPEC>;
#[doc = "Field `DMA_TX_THD_VAL` reader - TX FIFO Level DMA Trigger."]
pub type DMA_TX_THD_VAL_R = crate::FieldReader;
#[doc = "Field `DMA_TX_THD_VAL` writer - TX FIFO Level DMA Trigger."]
pub type DMA_TX_THD_VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DMA_TX_EN` reader - TX DMA channel enable."]
pub type DMA_TX_EN_R = crate::BitReader;
#[doc = "Field `DMA_TX_EN` writer - TX DMA channel enable."]
pub type DMA_TX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA_RX_THD_VAL` reader - RX FIFO Level DMA Trigger."]
pub type DMA_RX_THD_VAL_R = crate::FieldReader;
#[doc = "Field `DMA_RX_THD_VAL` writer - RX FIFO Level DMA Trigger."]
pub type DMA_RX_THD_VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DMA_RX_EN` reader - RX DMA channel enable."]
pub type DMA_RX_EN_R = crate::BitReader;
#[doc = "Field `DMA_RX_EN` writer - RX DMA channel enable."]
pub type DMA_RX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_LVL` reader - Number of data word in the TX FIFO."]
pub type TX_LVL_R = crate::FieldReader;
#[doc = "Field `TX_LVL` writer - Number of data word in the TX FIFO."]
pub type TX_LVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RX_LVL` reader - Number of data word in the RX FIFO."]
pub type RX_LVL_R = crate::FieldReader;
#[doc = "Field `RX_LVL` writer - Number of data word in the RX FIFO."]
pub type RX_LVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:6 - TX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_tx_thd_val(&self) -> DMA_TX_THD_VAL_R {
        DMA_TX_THD_VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TX DMA channel enable."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RX FIFO Level DMA Trigger."]
    #[inline(always)]
    pub fn dma_rx_thd_val(&self) -> DMA_RX_THD_VAL_R {
        DMA_RX_THD_VAL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - RX DMA channel enable."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of data word in the TX FIFO."]
    #[inline(always)]
    pub fn tx_lvl(&self) -> TX_LVL_R {
        TX_LVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of data word in the RX FIFO."]
    #[inline(always)]
    pub fn rx_lvl(&self) -> RX_LVL_R {
        RX_LVL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TX FIFO Level DMA Trigger."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_thd_val(&mut self) -> DMA_TX_THD_VAL_W<DMACH0_SPEC, 0> {
        DMA_TX_THD_VAL_W::new(self)
    }
    #[doc = "Bit 7 - TX DMA channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W<DMACH0_SPEC, 7> {
        DMA_TX_EN_W::new(self)
    }
    #[doc = "Bits 8:14 - RX FIFO Level DMA Trigger."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_thd_val(&mut self) -> DMA_RX_THD_VAL_W<DMACH0_SPEC, 8> {
        DMA_RX_THD_VAL_W::new(self)
    }
    #[doc = "Bit 15 - RX DMA channel enable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W<DMACH0_SPEC, 15> {
        DMA_RX_EN_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of data word in the TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lvl(&mut self) -> TX_LVL_W<DMACH0_SPEC, 16> {
        TX_LVL_W::new(self)
    }
    #[doc = "Bits 24:31 - Number of data word in the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_lvl(&mut self) -> RX_LVL_W<DMACH0_SPEC, 24> {
        RX_LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACH0_SPEC;
impl crate::RegisterSpec for DMACH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach0::R`](R) reader structure"]
impl crate::Readable for DMACH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmach0::W`](W) writer structure"]
impl crate::Writable for DMACH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACH0 to value 0"]
impl crate::Resettable for DMACH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
