#[doc = "Register `RQR` reader"]
pub type R = crate::R<RqrSpec>;
#[doc = "Register `RQR` writer"]
pub type W = crate::W<RqrSpec>;
#[doc = "Field `ABRRQ` reader - Auto baud rate request"]
pub type AbrrqR = crate::BitReader;
#[doc = "Field `ABRRQ` writer - Auto baud rate request"]
pub type AbrrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBKRQ` reader - Send break request"]
pub type SbkrqR = crate::BitReader;
#[doc = "Field `SBKRQ` writer - Send break request"]
pub type SbkrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` reader - Mute mode request"]
pub type MmrqR = crate::BitReader;
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub type MmrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` reader - Receive data flush request"]
pub type RxfrqR = crate::BitReader;
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub type RxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` reader - Transmit data flush request"]
pub type TxfrqR = crate::BitReader;
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub type TxfrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&self) -> AbrrqR {
        AbrrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&self) -> SbkrqR {
        SbkrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&self) -> MmrqR {
        MmrqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&self) -> RxfrqR {
        RxfrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&self) -> TxfrqR {
        TxfrqR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&mut self) -> AbrrqW<'_, RqrSpec> {
        AbrrqW::new(self, 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SbkrqW<'_, RqrSpec> {
        SbkrqW::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&mut self) -> MmrqW<'_, RqrSpec> {
        MmrqW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RxfrqW<'_, RqrSpec> {
        RxfrqW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&mut self) -> TxfrqW<'_, RqrSpec> {
        TxfrqW::new(self, 4)
    }
}
#[doc = "Request register\n\nYou can [`read`](crate::Reg::read) this register and get [`rqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RqrSpec;
impl crate::RegisterSpec for RqrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rqr::R`](R) reader structure"]
impl crate::Readable for RqrSpec {}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RqrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RqrSpec {}
