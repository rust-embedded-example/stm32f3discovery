#[doc = "Register `SYSCFG_CFGR1` reader"]
pub type R = crate::R<SyscfgCfgr1Spec>;
#[doc = "Register `SYSCFG_CFGR1` writer"]
pub type W = crate::W<SyscfgCfgr1Spec>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_IT_RMP` reader - USB interrupt remap"]
pub type UsbItRmpR = crate::BitReader;
#[doc = "Field `USB_IT_RMP` writer - USB interrupt remap"]
pub type UsbItRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1_ITR_RMP` reader - Timer 1 ITR3 selection"]
pub type Tim1ItrRmpR = crate::BitReader;
#[doc = "Field `TIM1_ITR_RMP` writer - Timer 1 ITR3 selection"]
pub type Tim1ItrRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_TRIG_RMP` reader - DAC trigger remap (when TSEL = 001)"]
pub type DacTrigRmpR = crate::BitReader;
#[doc = "Field `DAC_TRIG_RMP` writer - DAC trigger remap (when TSEL = 001)"]
pub type DacTrigRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC24_DMA_RMP` reader - ADC24 DMA remapping bit"]
pub type Adc24DmaRmpR = crate::BitReader;
#[doc = "Field `ADC24_DMA_RMP` writer - ADC24 DMA remapping bit"]
pub type Adc24DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit"]
pub type Tim16DmaRmpR = crate::BitReader;
#[doc = "Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit"]
pub type Tim16DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit"]
pub type Tim17DmaRmpR = crate::BitReader;
#[doc = "Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit"]
pub type Tim17DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6_DAC1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit"]
pub type Tim6Dac1DmaRmpR = crate::BitReader;
#[doc = "Field `TIM6_DAC1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit"]
pub type Tim6Dac1DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7_DAC2_DMA_RMP` reader - TIM7 and DAC2 DMA request remapping bit"]
pub type Tim7Dac2DmaRmpR = crate::BitReader;
#[doc = "Field `TIM7_DAC2_DMA_RMP` writer - TIM7 and DAC2 DMA request remapping bit"]
pub type Tim7Dac2DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB6_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb6FmR = crate::BitReader;
#[doc = "Field `I2C_PB6_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb6FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb7FmR = crate::BitReader;
#[doc = "Field `I2C_PB7_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb7FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb8FmR = crate::BitReader;
#[doc = "Field `I2C_PB8_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb8FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB9_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb9FmR = crate::BitReader;
#[doc = "Field `I2C_PB9_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2cPb9FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_FM` reader - I2C1 Fast Mode Plus"]
pub type I2c1FmR = crate::BitReader;
#[doc = "Field `I2C1_FM` writer - I2C1 Fast Mode Plus"]
pub type I2c1FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_FM` reader - I2C2 Fast Mode Plus"]
pub type I2c2FmR = crate::BitReader;
#[doc = "Field `I2C2_FM` writer - I2C2 Fast Mode Plus"]
pub type I2c2FmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCODER_MODE` reader - Encoder mode"]
pub type EncoderModeR = crate::FieldReader;
#[doc = "Field `ENCODER_MODE` writer - Encoder mode"]
pub type EncoderModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FPU_IT` reader - Interrupt enable bits from FPU"]
pub type FpuItR = crate::FieldReader;
#[doc = "Field `FPU_IT` writer - Interrupt enable bits from FPU"]
pub type FpuItW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&self) -> UsbItRmpR {
        UsbItRmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr_rmp(&self) -> Tim1ItrRmpR {
        Tim1ItrRmpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac_trig_rmp(&self) -> DacTrigRmpR {
        DacTrigRmpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc24_dma_rmp(&self) -> Adc24DmaRmpR {
        Adc24DmaRmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> Tim16DmaRmpR {
        Tim16DmaRmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> Tim17DmaRmpR {
        Tim17DmaRmpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_dma_rmp(&self) -> Tim6Dac1DmaRmpR {
        Tim6Dac1DmaRmpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac2_dma_rmp(&self) -> Tim7Dac2DmaRmpR {
        Tim7Dac2DmaRmpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fm(&self) -> I2cPb6FmR {
        I2cPb6FmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fm(&self) -> I2cPb7FmR {
        I2cPb7FmR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fm(&self) -> I2cPb8FmR {
        I2cPb8FmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fm(&self) -> I2cPb9FmR {
        I2cPb9FmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fm(&self) -> I2c1FmR {
        I2c1FmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fm(&self) -> I2c2FmR {
        I2c2FmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&self) -> EncoderModeR {
        EncoderModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:31 - Interrupt enable bits from FPU"]
    #[inline(always)]
    pub fn fpu_it(&self) -> FpuItR {
        FpuItR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MemModeW<'_, SyscfgCfgr1Spec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&mut self) -> UsbItRmpW<'_, SyscfgCfgr1Spec> {
        UsbItRmpW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr_rmp(&mut self) -> Tim1ItrRmpW<'_, SyscfgCfgr1Spec> {
        Tim1ItrRmpW::new(self, 6)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac_trig_rmp(&mut self) -> DacTrigRmpW<'_, SyscfgCfgr1Spec> {
        DacTrigRmpW::new(self, 7)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc24_dma_rmp(&mut self) -> Adc24DmaRmpW<'_, SyscfgCfgr1Spec> {
        Adc24DmaRmpW::new(self, 8)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&mut self) -> Tim16DmaRmpW<'_, SyscfgCfgr1Spec> {
        Tim16DmaRmpW::new(self, 11)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&mut self) -> Tim17DmaRmpW<'_, SyscfgCfgr1Spec> {
        Tim17DmaRmpW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_dma_rmp(&mut self) -> Tim6Dac1DmaRmpW<'_, SyscfgCfgr1Spec> {
        Tim6Dac1DmaRmpW::new(self, 13)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac2_dma_rmp(&mut self) -> Tim7Dac2DmaRmpW<'_, SyscfgCfgr1Spec> {
        Tim7Dac2DmaRmpW::new(self, 14)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fm(&mut self) -> I2cPb6FmW<'_, SyscfgCfgr1Spec> {
        I2cPb6FmW::new(self, 16)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fm(&mut self) -> I2cPb7FmW<'_, SyscfgCfgr1Spec> {
        I2cPb7FmW::new(self, 17)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fm(&mut self) -> I2cPb8FmW<'_, SyscfgCfgr1Spec> {
        I2cPb8FmW::new(self, 18)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fm(&mut self) -> I2cPb9FmW<'_, SyscfgCfgr1Spec> {
        I2cPb9FmW::new(self, 19)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fm(&mut self) -> I2c1FmW<'_, SyscfgCfgr1Spec> {
        I2c1FmW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fm(&mut self) -> I2c2FmW<'_, SyscfgCfgr1Spec> {
        I2c2FmW::new(self, 21)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&mut self) -> EncoderModeW<'_, SyscfgCfgr1Spec> {
        EncoderModeW::new(self, 22)
    }
    #[doc = "Bits 26:31 - Interrupt enable bits from FPU"]
    #[inline(always)]
    pub fn fpu_it(&mut self) -> FpuItW<'_, SyscfgCfgr1Spec> {
        FpuItW::new(self, 26)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgCfgr1Spec;
impl crate::RegisterSpec for SyscfgCfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr1::R`](R) reader structure"]
impl crate::Readable for SyscfgCfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr1::W`](W) writer structure"]
impl crate::Writable for SyscfgCfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG_CFGR1 to value 0"]
impl crate::Resettable for SyscfgCfgr1Spec {}
