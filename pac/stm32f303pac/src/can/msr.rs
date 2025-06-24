#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Register `MSR` writer"]
pub type W = crate::W<MsrSpec>;
#[doc = "Field `INAK` reader - INAK"]
pub type InakR = crate::BitReader;
#[doc = "Field `SLAK` reader - SLAK"]
pub type SlakR = crate::BitReader;
#[doc = "Field `ERRI` reader - ERRI"]
pub type ErriR = crate::BitReader;
#[doc = "Field `ERRI` writer - ERRI"]
pub type ErriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUI` reader - WKUI"]
pub type WkuiR = crate::BitReader;
#[doc = "Field `WKUI` writer - WKUI"]
pub type WkuiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAKI` reader - SLAKI"]
pub type SlakiR = crate::BitReader;
#[doc = "Field `SLAKI` writer - SLAKI"]
pub type SlakiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXM` reader - TXM"]
pub type TxmR = crate::BitReader;
#[doc = "Field `RXM` reader - RXM"]
pub type RxmR = crate::BitReader;
#[doc = "Field `SAMP` reader - SAMP"]
pub type SampR = crate::BitReader;
#[doc = "Field `RX` reader - RX"]
pub type RxR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - INAK"]
    #[inline(always)]
    pub fn inak(&self) -> InakR {
        InakR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLAK"]
    #[inline(always)]
    pub fn slak(&self) -> SlakR {
        SlakR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&self) -> ErriR {
        ErriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&self) -> WkuiR {
        WkuiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&self) -> SlakiR {
        SlakiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TXM"]
    #[inline(always)]
    pub fn txm(&self) -> TxmR {
        TxmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXM"]
    #[inline(always)]
    pub fn rxm(&self) -> RxmR {
        RxmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SAMP"]
    #[inline(always)]
    pub fn samp(&self) -> SampR {
        SampR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&mut self) -> ErriW<MsrSpec> {
        ErriW::new(self, 2)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&mut self) -> WkuiW<MsrSpec> {
        WkuiW::new(self, 3)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&mut self) -> SlakiW<MsrSpec> {
        SlakiW::new(self, 4)
    }
}
#[doc = "master status register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSR to value 0x0c02"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0x0c02;
}
