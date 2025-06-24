#[doc = "Register `OPAMP2_CSR` reader"]
pub type R = crate::R<Opamp2CsrSpec>;
#[doc = "Register `OPAMP2_CSR` writer"]
pub type W = crate::W<Opamp2CsrSpec>;
#[doc = "Field `OPAMP2EN` reader - OPAMP2 enable"]
pub type Opamp2enR = crate::BitReader;
#[doc = "Field `OPAMP2EN` writer - OPAMP2 enable"]
pub type Opamp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
pub type ForceVpR = crate::BitReader;
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub type ForceVpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VP_SEL` reader - OPAMP2 Non inverting input selection"]
pub type VpSelR = crate::FieldReader;
#[doc = "Field `VP_SEL` writer - OPAMP2 Non inverting input selection"]
pub type VpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VM_SEL` reader - OPAMP2 inverting input selection"]
pub type VmSelR = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - OPAMP2 inverting input selection"]
pub type VmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCM_EN` reader - Timer controlled Mux mode enable"]
pub type TcmEnR = crate::BitReader;
#[doc = "Field `TCM_EN` writer - Timer controlled Mux mode enable"]
pub type TcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMS_SEL` reader - OPAMP2 inverting input secondary selection"]
pub type VmsSelR = crate::BitReader;
#[doc = "Field `VMS_SEL` writer - OPAMP2 inverting input secondary selection"]
pub type VmsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPS_SEL` reader - OPAMP2 Non inverting input secondary selection"]
pub type VpsSelR = crate::FieldReader;
#[doc = "Field `VPS_SEL` writer - OPAMP2 Non inverting input secondary selection"]
pub type VpsSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CALON` reader - Calibration mode enable"]
pub type CalonR = crate::BitReader;
#[doc = "Field `CALON` writer - Calibration mode enable"]
pub type CalonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CalselR = crate::FieldReader;
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CalselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGA_GAIN` reader - Gain in PGA mode"]
pub type PgaGainR = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - Gain in PGA mode"]
pub type PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USER_TRIM` reader - User trimming enable"]
pub type UserTrimR = crate::BitReader;
#[doc = "Field `USER_TRIM` writer - User trimming enable"]
pub type UserTrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)"]
pub type TrimoffsetpR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)"]
pub type TrimoffsetpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)"]
pub type TrimoffsetnR = crate::FieldReader;
#[doc = "Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)"]
pub type TrimoffsetnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSTREF` reader - TSTREF"]
pub type TstrefR = crate::BitReader;
#[doc = "Field `TSTREF` writer - TSTREF"]
pub type TstrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTCAL` reader - OPAMP 2 ouput status flag"]
pub type OutcalR = crate::BitReader;
#[doc = "Field `LOCK` reader - OPAMP 2 lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - OPAMP 2 lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OPAMP2 enable"]
    #[inline(always)]
    pub fn opamp2en(&self) -> Opamp2enR {
        Opamp2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> ForceVpR {
        ForceVpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - OPAMP2 Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VpSelR {
        VpSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - OPAMP2 inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VmSelR {
        VmSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&self) -> TcmEnR {
        TcmEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAMP2 inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VmsSelR {
        VmsSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - OPAMP2 Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VpsSelR {
        VpsSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CalselR {
        CalselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PgaGainR {
        PgaGainR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&self) -> UserTrimR {
        UserTrimR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TrimoffsetpR {
        TrimoffsetpR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TrimoffsetnR {
        TrimoffsetnR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&self) -> TstrefR {
        TstrefR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP 2 ouput status flag"]
    #[inline(always)]
    pub fn outcal(&self) -> OutcalR {
        OutcalR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMP 2 lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP2 enable"]
    #[inline(always)]
    pub fn opamp2en(&mut self) -> Opamp2enW<'_, Opamp2CsrSpec> {
        Opamp2enW::new(self, 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> ForceVpW<'_, Opamp2CsrSpec> {
        ForceVpW::new(self, 1)
    }
    #[doc = "Bits 2:3 - OPAMP2 Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VpSelW<'_, Opamp2CsrSpec> {
        VpSelW::new(self, 2)
    }
    #[doc = "Bits 5:6 - OPAMP2 inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VmSelW<'_, Opamp2CsrSpec> {
        VmSelW::new(self, 5)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&mut self) -> TcmEnW<'_, Opamp2CsrSpec> {
        TcmEnW::new(self, 7)
    }
    #[doc = "Bit 8 - OPAMP2 inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VmsSelW<'_, Opamp2CsrSpec> {
        VmsSelW::new(self, 8)
    }
    #[doc = "Bits 9:10 - OPAMP2 Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VpsSelW<'_, Opamp2CsrSpec> {
        VpsSelW::new(self, 9)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&mut self) -> CalonW<'_, Opamp2CsrSpec> {
        CalonW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CalselW<'_, Opamp2CsrSpec> {
        CalselW::new(self, 12)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PgaGainW<'_, Opamp2CsrSpec> {
        PgaGainW::new(self, 14)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&mut self) -> UserTrimW<'_, Opamp2CsrSpec> {
        UserTrimW::new(self, 18)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TrimoffsetpW<'_, Opamp2CsrSpec> {
        TrimoffsetpW::new(self, 19)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TrimoffsetnW<'_, Opamp2CsrSpec> {
        TrimoffsetnW::new(self, 24)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&mut self) -> TstrefW<'_, Opamp2CsrSpec> {
        TstrefW::new(self, 29)
    }
    #[doc = "Bit 31 - OPAMP 2 lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, Opamp2CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp2CsrSpec;
impl crate::RegisterSpec for Opamp2CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_csr::R`](R) reader structure"]
impl crate::Readable for Opamp2CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp2_csr::W`](W) writer structure"]
impl crate::Writable for Opamp2CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for Opamp2CsrSpec {}
