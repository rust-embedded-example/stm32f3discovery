#[doc = "Register `OBR` reader"]
pub type R = crate::R<ObrSpec>;
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OpterrR = crate::BitReader;
#[doc = "Field `LEVEL1_PROT` reader - Level 1 protection status"]
pub type Level1ProtR = crate::BitReader;
#[doc = "Field `LEVEL2_PROT` reader - Level 2 protection status"]
pub type Level2ProtR = crate::BitReader;
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WdgSwR = crate::BitReader;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type NRstStopR = crate::BitReader;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type NRstStdbyR = crate::BitReader;
#[doc = "Field `BOOT1` reader - BOOT1"]
pub type Boot1R = crate::BitReader;
#[doc = "Field `VDDA_MONITOR` reader - VDDA_MONITOR"]
pub type VddaMonitorR = crate::BitReader;
#[doc = "Field `SRAM_PARITY_CHECK` reader - SRAM_PARITY_CHECK"]
pub type SramParityCheckR = crate::BitReader;
#[doc = "Field `Data0` reader - Data0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `Data1` reader - Data1"]
pub type Data1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OpterrR {
        OpterrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1 protection status"]
    #[inline(always)]
    pub fn level1_prot(&self) -> Level1ProtR {
        Level1ProtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 2 protection status"]
    #[inline(always)]
    pub fn level2_prot(&self) -> Level2ProtR {
        Level2ProtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WdgSwR {
        WdgSwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRstStopR {
        NRstStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRstStdbyR {
        NRstStdbyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn boot1(&self) -> Boot1R {
        Boot1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VddaMonitorR {
        VddaMonitorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM_PARITY_CHECK"]
    #[inline(always)]
    pub fn sram_parity_check(&self) -> SramParityCheckR {
        SramParityCheckR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Option byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`obr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObrSpec;
impl crate::RegisterSpec for ObrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obr::R`](R) reader structure"]
impl crate::Readable for ObrSpec {}
#[doc = "`reset()` method sets OBR to value 0xffff_ff02"]
impl crate::Resettable for ObrSpec {
    const RESET_VALUE: u32 = 0xffff_ff02;
}
