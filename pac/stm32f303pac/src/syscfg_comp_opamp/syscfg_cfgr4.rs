#[doc = "Register `SYSCFG_CFGR4` reader"]
pub type R = crate::R<SyscfgCfgr4Spec>;
#[doc = "Register `SYSCFG_CFGR4` writer"]
pub type W = crate::W<SyscfgCfgr4Spec>;
#[doc = "Field `ADC12_EXT2_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT2"]
pub type Adc12Ext2RmpR = crate::BitReader;
#[doc = "Field `ADC12_EXT2_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT2"]
pub type Adc12Ext2RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12_EXT3_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT3"]
pub type Adc12Ext3RmpR = crate::BitReader;
#[doc = "Field `ADC12_EXT3_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT3"]
pub type Adc12Ext3RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12_EXT5_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT5"]
pub type Adc12Ext5RmpR = crate::BitReader;
#[doc = "Field `ADC12_EXT5_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT5"]
pub type Adc12Ext5RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12_EXT13_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT13"]
pub type Adc12Ext13RmpR = crate::BitReader;
#[doc = "Field `ADC12_EXT13_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT13"]
pub type Adc12Ext13RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12_EXT15_RMP` reader - Controls the Input trigger of ADC12 regular channel EXT15"]
pub type Adc12Ext15RmpR = crate::BitReader;
#[doc = "Field `ADC12_EXT15_RMP` writer - Controls the Input trigger of ADC12 regular channel EXT15"]
pub type Adc12Ext15RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12_JEXT3_RMP` reader - Controls the Input trigger of ADC12 injected channel EXTI3"]
pub type Adc12Jext3RmpR = crate::BitReader;
#[doc = "Field `ADC12_JEXT3_RMP` writer - Controls the Input trigger of ADC12 injected channel EXTI3"]
pub type Adc12Jext3RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12_JEXT6_RMP` reader - Controls the Input trigger of ADC12 injected channel EXTI6"]
pub type Adc12Jext6RmpR = crate::BitReader;
#[doc = "Field `ADC12_JEXT6_RMP` writer - Controls the Input trigger of ADC12 injected channel EXTI6"]
pub type Adc12Jext6RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12_JEXT13_RMP` reader - Controls the Input trigger of ADC12 injected channel EXTI13"]
pub type Adc12Jext13RmpR = crate::BitReader;
#[doc = "Field `ADC12_JEXT13_RMP` writer - Controls the Input trigger of ADC12 injected channel EXTI13"]
pub type Adc12Jext13RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34_EXT5_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT5"]
pub type Adc34Ext5RmpR = crate::BitReader;
#[doc = "Field `ADC34_EXT5_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT5"]
pub type Adc34Ext5RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34_EXT6_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT6"]
pub type Adc34Ext6RmpR = crate::BitReader;
#[doc = "Field `ADC34_EXT6_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT6"]
pub type Adc34Ext6RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34_EXT15_RMP` reader - Controls the Input trigger of ADC34 regular channel EXT15"]
pub type Adc34Ext15RmpR = crate::BitReader;
#[doc = "Field `ADC34_EXT15_RMP` writer - Controls the Input trigger of ADC34 regular channel EXT15"]
pub type Adc34Ext15RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34_JEXT5_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT5"]
pub type Adc34Jext5RmpR = crate::BitReader;
#[doc = "Field `ADC34_JEXT5_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT5"]
pub type Adc34Jext5RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34_JEXT11_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT11"]
pub type Adc34Jext11RmpR = crate::BitReader;
#[doc = "Field `ADC34_JEXT11_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT11"]
pub type Adc34Jext11RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34_JEXT14_RMP` reader - Controls the Input trigger of ADC34 injected channel JEXT14"]
pub type Adc34Jext14RmpR = crate::BitReader;
#[doc = "Field `ADC34_JEXT14_RMP` writer - Controls the Input trigger of ADC34 injected channel JEXT14"]
pub type Adc34Jext14RmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2"]
    #[inline(always)]
    pub fn adc12_ext2_rmp(&self) -> Adc12Ext2RmpR {
        Adc12Ext2RmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3"]
    #[inline(always)]
    pub fn adc12_ext3_rmp(&self) -> Adc12Ext3RmpR {
        Adc12Ext3RmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5"]
    #[inline(always)]
    pub fn adc12_ext5_rmp(&self) -> Adc12Ext5RmpR {
        Adc12Ext5RmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13"]
    #[inline(always)]
    pub fn adc12_ext13_rmp(&self) -> Adc12Ext13RmpR {
        Adc12Ext13RmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15"]
    #[inline(always)]
    pub fn adc12_ext15_rmp(&self) -> Adc12Ext15RmpR {
        Adc12Ext15RmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Controls the Input trigger of ADC12 injected channel EXTI3"]
    #[inline(always)]
    pub fn adc12_jext3_rmp(&self) -> Adc12Jext3RmpR {
        Adc12Jext3RmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Controls the Input trigger of ADC12 injected channel EXTI6"]
    #[inline(always)]
    pub fn adc12_jext6_rmp(&self) -> Adc12Jext6RmpR {
        Adc12Jext6RmpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Controls the Input trigger of ADC12 injected channel EXTI13"]
    #[inline(always)]
    pub fn adc12_jext13_rmp(&self) -> Adc12Jext13RmpR {
        Adc12Jext13RmpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5"]
    #[inline(always)]
    pub fn adc34_ext5_rmp(&self) -> Adc34Ext5RmpR {
        Adc34Ext5RmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6"]
    #[inline(always)]
    pub fn adc34_ext6_rmp(&self) -> Adc34Ext6RmpR {
        Adc34Ext6RmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15"]
    #[inline(always)]
    pub fn adc34_ext15_rmp(&self) -> Adc34Ext15RmpR {
        Adc34Ext15RmpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5"]
    #[inline(always)]
    pub fn adc34_jext5_rmp(&self) -> Adc34Jext5RmpR {
        Adc34Jext5RmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11"]
    #[inline(always)]
    pub fn adc34_jext11_rmp(&self) -> Adc34Jext11RmpR {
        Adc34Jext11RmpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14"]
    #[inline(always)]
    pub fn adc34_jext14_rmp(&self) -> Adc34Jext14RmpR {
        Adc34Jext14RmpR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the Input trigger of ADC12 regular channel EXT2"]
    #[inline(always)]
    pub fn adc12_ext2_rmp(&mut self) -> Adc12Ext2RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Ext2RmpW::new(self, 0)
    }
    #[doc = "Bit 1 - Controls the Input trigger of ADC12 regular channel EXT3"]
    #[inline(always)]
    pub fn adc12_ext3_rmp(&mut self) -> Adc12Ext3RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Ext3RmpW::new(self, 1)
    }
    #[doc = "Bit 2 - Controls the Input trigger of ADC12 regular channel EXT5"]
    #[inline(always)]
    pub fn adc12_ext5_rmp(&mut self) -> Adc12Ext5RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Ext5RmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Controls the Input trigger of ADC12 regular channel EXT13"]
    #[inline(always)]
    pub fn adc12_ext13_rmp(&mut self) -> Adc12Ext13RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Ext13RmpW::new(self, 3)
    }
    #[doc = "Bit 4 - Controls the Input trigger of ADC12 regular channel EXT15"]
    #[inline(always)]
    pub fn adc12_ext15_rmp(&mut self) -> Adc12Ext15RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Ext15RmpW::new(self, 4)
    }
    #[doc = "Bit 5 - Controls the Input trigger of ADC12 injected channel EXTI3"]
    #[inline(always)]
    pub fn adc12_jext3_rmp(&mut self) -> Adc12Jext3RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Jext3RmpW::new(self, 5)
    }
    #[doc = "Bit 6 - Controls the Input trigger of ADC12 injected channel EXTI6"]
    #[inline(always)]
    pub fn adc12_jext6_rmp(&mut self) -> Adc12Jext6RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Jext6RmpW::new(self, 6)
    }
    #[doc = "Bit 7 - Controls the Input trigger of ADC12 injected channel EXTI13"]
    #[inline(always)]
    pub fn adc12_jext13_rmp(&mut self) -> Adc12Jext13RmpW<'_, SyscfgCfgr4Spec> {
        Adc12Jext13RmpW::new(self, 7)
    }
    #[doc = "Bit 8 - Controls the Input trigger of ADC34 regular channel EXT5"]
    #[inline(always)]
    pub fn adc34_ext5_rmp(&mut self) -> Adc34Ext5RmpW<'_, SyscfgCfgr4Spec> {
        Adc34Ext5RmpW::new(self, 8)
    }
    #[doc = "Bit 9 - Controls the Input trigger of ADC34 regular channel EXT6"]
    #[inline(always)]
    pub fn adc34_ext6_rmp(&mut self) -> Adc34Ext6RmpW<'_, SyscfgCfgr4Spec> {
        Adc34Ext6RmpW::new(self, 9)
    }
    #[doc = "Bit 10 - Controls the Input trigger of ADC34 regular channel EXT15"]
    #[inline(always)]
    pub fn adc34_ext15_rmp(&mut self) -> Adc34Ext15RmpW<'_, SyscfgCfgr4Spec> {
        Adc34Ext15RmpW::new(self, 10)
    }
    #[doc = "Bit 11 - Controls the Input trigger of ADC34 injected channel JEXT5"]
    #[inline(always)]
    pub fn adc34_jext5_rmp(&mut self) -> Adc34Jext5RmpW<'_, SyscfgCfgr4Spec> {
        Adc34Jext5RmpW::new(self, 11)
    }
    #[doc = "Bit 12 - Controls the Input trigger of ADC34 injected channel JEXT11"]
    #[inline(always)]
    pub fn adc34_jext11_rmp(&mut self) -> Adc34Jext11RmpW<'_, SyscfgCfgr4Spec> {
        Adc34Jext11RmpW::new(self, 12)
    }
    #[doc = "Bit 13 - Controls the Input trigger of ADC34 injected channel JEXT14"]
    #[inline(always)]
    pub fn adc34_jext14_rmp(&mut self) -> Adc34Jext14RmpW<'_, SyscfgCfgr4Spec> {
        Adc34Jext14RmpW::new(self, 13)
    }
}
#[doc = "SYSCFG configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgCfgr4Spec;
impl crate::RegisterSpec for SyscfgCfgr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr4::R`](R) reader structure"]
impl crate::Readable for SyscfgCfgr4Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr4::W`](W) writer structure"]
impl crate::Writable for SyscfgCfgr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG_CFGR4 to value 0"]
impl crate::Resettable for SyscfgCfgr4Spec {}
