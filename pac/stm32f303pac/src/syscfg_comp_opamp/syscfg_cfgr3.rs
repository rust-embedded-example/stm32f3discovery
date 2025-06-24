#[doc = "Register `SYSCFG_CFGR3` reader"]
pub type R = crate::R<SyscfgCfgr3Spec>;
#[doc = "Register `SYSCFG_CFGR3` writer"]
pub type W = crate::W<SyscfgCfgr3Spec>;
#[doc = "Field `SPI1_RX_DMA_RMP` reader - SPI1_RX DMA remapping bit"]
pub type Spi1RxDmaRmpR = crate::FieldReader;
#[doc = "Field `SPI1_RX_DMA_RMP` writer - SPI1_RX DMA remapping bit"]
pub type Spi1RxDmaRmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI1_TX_DMA_RMP` reader - SPI1_TX DMA remapping bit"]
pub type Spi1TxDmaRmpR = crate::FieldReader;
#[doc = "Field `SPI1_TX_DMA_RMP` writer - SPI1_TX DMA remapping bit"]
pub type Spi1TxDmaRmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1_RX_DMA_RMP` reader - I2C1_RX DMA remapping bit"]
pub type I2c1RxDmaRmpR = crate::FieldReader;
#[doc = "Field `I2C1_RX_DMA_RMP` writer - I2C1_RX DMA remapping bit"]
pub type I2c1RxDmaRmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1_TX_DMA_RMP` reader - I2C1_TX DMA remapping bit"]
pub type I2c1TxDmaRmpR = crate::FieldReader;
#[doc = "Field `I2C1_TX_DMA_RMP` writer - I2C1_TX DMA remapping bit"]
pub type I2c1TxDmaRmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC2_DMA_RMP` reader - ADC2 DMA channel remapping bit"]
pub type Adc2DmaRmpR = crate::FieldReader;
#[doc = "Field `ADC2_DMA_RMP` writer - ADC2 DMA channel remapping bit"]
pub type Adc2DmaRmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&self) -> Spi1RxDmaRmpR {
        Spi1RxDmaRmpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&self) -> Spi1TxDmaRmpR {
        Spi1TxDmaRmpR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&self) -> I2c1RxDmaRmpR {
        I2c1RxDmaRmpR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&self) -> I2c1TxDmaRmpR {
        I2c1TxDmaRmpR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC2 DMA channel remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> Adc2DmaRmpR {
        Adc2DmaRmpR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&mut self) -> Spi1RxDmaRmpW<'_, SyscfgCfgr3Spec> {
        Spi1RxDmaRmpW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&mut self) -> Spi1TxDmaRmpW<'_, SyscfgCfgr3Spec> {
        Spi1TxDmaRmpW::new(self, 2)
    }
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&mut self) -> I2c1RxDmaRmpW<'_, SyscfgCfgr3Spec> {
        I2c1RxDmaRmpW::new(self, 4)
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&mut self) -> I2c1TxDmaRmpW<'_, SyscfgCfgr3Spec> {
        I2c1TxDmaRmpW::new(self, 6)
    }
    #[doc = "Bits 8:9 - ADC2 DMA channel remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&mut self) -> Adc2DmaRmpW<'_, SyscfgCfgr3Spec> {
        Adc2DmaRmpW::new(self, 8)
    }
}
#[doc = "SYSCFG configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgCfgr3Spec;
impl crate::RegisterSpec for SyscfgCfgr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr3::R`](R) reader structure"]
impl crate::Readable for SyscfgCfgr3Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr3::W`](W) writer structure"]
impl crate::Writable for SyscfgCfgr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG_CFGR3 to value 0"]
impl crate::Resettable for SyscfgCfgr3Spec {}
