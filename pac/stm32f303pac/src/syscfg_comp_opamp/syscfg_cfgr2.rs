#[doc = "Register `SYSCFG_CFGR2` reader"]
pub type R = crate::R<SyscfgCfgr2Spec>;
#[doc = "Register `SYSCFG_CFGR2` writer"]
pub type W = crate::W<SyscfgCfgr2Spec>;
#[doc = "Field `LOCUP_LOCK` reader - Cortex-M0 LOCKUP bit enable bit"]
pub type LocupLockR = crate::BitReader;
#[doc = "Field `LOCUP_LOCK` writer - Cortex-M0 LOCKUP bit enable bit"]
pub type LocupLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub type SramParityLockR = crate::BitReader;
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub type SramParityLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PvdLockR = crate::BitReader;
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PvdLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYP_ADD_PAR` reader - Bypass address bit 29 in parity calculation"]
pub type BypAddParR = crate::BitReader;
#[doc = "Field `BYP_ADD_PAR` writer - Bypass address bit 29 in parity calculation"]
pub type BypAddParW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_PEF` reader - SRAM parity flag"]
pub type SramPefR = crate::BitReader;
#[doc = "Field `SRAM_PEF` writer - SRAM parity flag"]
pub type SramPefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn locup_lock(&self) -> LocupLockR {
        LocupLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SramParityLockR {
        SramParityLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PvdLockR {
        PvdLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    pub fn byp_add_par(&self) -> BypAddParR {
        BypAddParR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SramPefR {
        SramPefR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn locup_lock(&mut self) -> LocupLockW<'_, SyscfgCfgr2Spec> {
        LocupLockW::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&mut self) -> SramParityLockW<'_, SyscfgCfgr2Spec> {
        SramParityLockW::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PvdLockW<'_, SyscfgCfgr2Spec> {
        PvdLockW::new(self, 2)
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    pub fn byp_add_par(&mut self) -> BypAddParW<'_, SyscfgCfgr2Spec> {
        BypAddParW::new(self, 4)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&mut self) -> SramPefW<'_, SyscfgCfgr2Spec> {
        SramPefW::new(self, 8)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgCfgr2Spec;
impl crate::RegisterSpec for SyscfgCfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr2::R`](R) reader structure"]
impl crate::Readable for SyscfgCfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr2::W`](W) writer structure"]
impl crate::Writable for SyscfgCfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG_CFGR2 to value 0"]
impl crate::Resettable for SyscfgCfgr2Spec {}
