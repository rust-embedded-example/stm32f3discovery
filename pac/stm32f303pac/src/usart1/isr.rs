#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `PE` reader - Parity error"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - Framing error"]
pub type FeR = crate::BitReader;
#[doc = "Field `NF` reader - Noise detected flag"]
pub type NfR = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error"]
pub type OreR = crate::BitReader;
#[doc = "Field `IDLE` reader - Idle line detected"]
pub type IdleR = crate::BitReader;
#[doc = "Field `RXNE` reader - Read data register not empty"]
pub type RxneR = crate::BitReader;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TcR = crate::BitReader;
#[doc = "Field `TXE` reader - Transmit data register empty"]
pub type TxeR = crate::BitReader;
#[doc = "Field `LBDF` reader - LIN break detection flag"]
pub type LbdfR = crate::BitReader;
#[doc = "Field `CTSIF` reader - CTS interrupt flag"]
pub type CtsifR = crate::BitReader;
#[doc = "Field `CTS` reader - CTS flag"]
pub type CtsR = crate::BitReader;
#[doc = "Field `RTOF` reader - Receiver timeout"]
pub type RtofR = crate::BitReader;
#[doc = "Field `EOBF` reader - End of block flag"]
pub type EobfR = crate::BitReader;
#[doc = "Field `ABRE` reader - Auto baud rate error"]
pub type AbreR = crate::BitReader;
#[doc = "Field `ABRF` reader - Auto baud rate flag"]
pub type AbrfR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag"]
pub type BusyR = crate::BitReader;
#[doc = "Field `CMF` reader - character match flag"]
pub type CmfR = crate::BitReader;
#[doc = "Field `SBKF` reader - Send break flag"]
pub type SbkfR = crate::BitReader;
#[doc = "Field `RWU` reader - Receiver wakeup from Mute mode"]
pub type RwuR = crate::BitReader;
#[doc = "Field `WUF` reader - Wakeup from Stop mode flag"]
pub type WufR = crate::BitReader;
#[doc = "Field `TEACK` reader - Transmit enable acknowledge flag"]
pub type TeackR = crate::BitReader;
#[doc = "Field `REACK` reader - Receive enable acknowledge flag"]
pub type ReackR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detected flag"]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbdf(&self) -> LbdfR {
        LbdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag"]
    #[inline(always)]
    pub fn ctsif(&self) -> CtsifR {
        CtsifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout"]
    #[inline(always)]
    pub fn rtof(&self) -> RtofR {
        RtofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of block flag"]
    #[inline(always)]
    pub fn eobf(&self) -> EobfR {
        EobfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto baud rate error"]
    #[inline(always)]
    pub fn abre(&self) -> AbreR {
        AbreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto baud rate flag"]
    #[inline(always)]
    pub fn abrf(&self) -> AbrfR {
        AbrfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - character match flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CmfR {
        CmfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag"]
    #[inline(always)]
    pub fn sbkf(&self) -> SbkfR {
        SbkfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from Stop mode flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WufR {
        WufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag"]
    #[inline(always)]
    pub fn teack(&self) -> TeackR {
        TeackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag"]
    #[inline(always)]
    pub fn reack(&self) -> ReackR {
        ReackR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Interrupt & status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0xc0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0xc0;
}
