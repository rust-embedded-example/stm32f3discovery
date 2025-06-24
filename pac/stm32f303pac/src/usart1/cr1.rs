#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `UE` reader - USART enable"]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - USART enable"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UESM` reader - USART enable in Stop mode"]
pub type UesmR = crate::BitReader;
#[doc = "Field `UESM` writer - USART enable in Stop mode"]
pub type UesmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IdleieR = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - interrupt enable"]
pub type TxeieR = crate::BitReader;
#[doc = "Field `TXEIE` writer - interrupt enable"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PeieR = crate::BitReader;
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Parity selection"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Parity selection"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - Receiver wakeup method"]
pub type WakeR = crate::BitReader;
#[doc = "Field `WAKE` writer - Receiver wakeup method"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - Word length"]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - Word length"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MME` reader - Mute mode enable"]
pub type MmeR = crate::BitReader;
#[doc = "Field `MME` writer - Mute mode enable"]
pub type MmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMIE` reader - Character match interrupt enable"]
pub type CmieR = crate::BitReader;
#[doc = "Field `CMIE` writer - Character match interrupt enable"]
pub type CmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER8` reader - Oversampling mode"]
pub type Over8R = crate::BitReader;
#[doc = "Field `OVER8` writer - Oversampling mode"]
pub type Over8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT` reader - Driver Enable deassertion time"]
pub type DedtR = crate::FieldReader;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time"]
pub type DedtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time"]
pub type DeatR = crate::FieldReader;
#[doc = "Field `DEAT` writer - Driver Enable assertion time"]
pub type DeatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTOIE` reader - Receiver timeout interrupt enable"]
pub type RtoieR = crate::BitReader;
#[doc = "Field `RTOIE` writer - Receiver timeout interrupt enable"]
pub type RtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBIE` reader - End of Block interrupt enable"]
pub type EobieR = crate::BitReader;
#[doc = "Field `EOBIE` writer - End of Block interrupt enable"]
pub type EobieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UesmR {
        UesmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&self) -> CmieR {
        CmieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn over8(&self) -> Over8R {
        Over8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn dedt(&self) -> DedtR {
        DedtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&self) -> DeatR {
        DeatR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtoie(&self) -> RtoieR {
        RtoieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn eobie(&self) -> EobieR {
        EobieR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<Cr1Spec> {
        UeW::new(self, 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&mut self) -> UesmW<Cr1Spec> {
        UesmW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<Cr1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<Cr1Spec> {
        RxneieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<Cr1Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PeieW<Cr1Spec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<Cr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&mut self) -> WakeW<Cr1Spec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<Cr1Spec> {
        MW::new(self, 12)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&mut self) -> MmeW<Cr1Spec> {
        MmeW::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&mut self) -> CmieW<Cr1Spec> {
        CmieW::new(self, 14)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn over8(&mut self) -> Over8W<Cr1Spec> {
        Over8W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn dedt(&mut self) -> DedtW<Cr1Spec> {
        DedtW::new(self, 16)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&mut self) -> DeatW<Cr1Spec> {
        DeatW::new(self, 21)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtoie(&mut self) -> RtoieW<Cr1Spec> {
        RtoieW::new(self, 26)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn eobie(&mut self) -> EobieW<Cr1Spec> {
        EobieW::new(self, 27)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
