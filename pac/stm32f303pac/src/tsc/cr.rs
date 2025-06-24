#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `TSCE` reader - Touch sensing controller enable"]
pub type TsceR = crate::BitReader;
#[doc = "Field `TSCE` writer - Touch sensing controller enable"]
pub type TsceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start a new acquisition"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start a new acquisition"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AM` reader - Acquisition mode"]
pub type AmR = crate::BitReader;
#[doc = "Field `AM` writer - Acquisition mode"]
pub type AmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCPOL` reader - Synchronization pin polarity"]
pub type SyncpolR = crate::BitReader;
#[doc = "Field `SYNCPOL` writer - Synchronization pin polarity"]
pub type SyncpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IODEF` reader - I/O Default mode"]
pub type IodefR = crate::BitReader;
#[doc = "Field `IODEF` writer - I/O Default mode"]
pub type IodefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCV` reader - Max count value"]
pub type McvR = crate::FieldReader;
#[doc = "Field `MCV` writer - Max count value"]
pub type McvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PGPSC` reader - pulse generator prescaler"]
pub type PgpscR = crate::FieldReader;
#[doc = "Field `PGPSC` writer - pulse generator prescaler"]
pub type PgpscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SSPSC` reader - Spread spectrum prescaler"]
pub type SspscR = crate::BitReader;
#[doc = "Field `SSPSC` writer - Spread spectrum prescaler"]
pub type SspscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE` reader - Spread spectrum enable"]
pub type SseR = crate::BitReader;
#[doc = "Field `SSE` writer - Spread spectrum enable"]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSD` reader - Spread spectrum deviation"]
pub type SsdR = crate::FieldReader;
#[doc = "Field `SSD` writer - Spread spectrum deviation"]
pub type SsdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTPL` reader - Charge transfer pulse low"]
pub type CtplR = crate::FieldReader;
#[doc = "Field `CTPL` writer - Charge transfer pulse low"]
pub type CtplW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTPH` reader - Charge transfer pulse high"]
pub type CtphR = crate::FieldReader;
#[doc = "Field `CTPH` writer - Charge transfer pulse high"]
pub type CtphW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Touch sensing controller enable"]
    #[inline(always)]
    pub fn tsce(&self) -> TsceR {
        TsceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start a new acquisition"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acquisition mode"]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization pin polarity"]
    #[inline(always)]
    pub fn syncpol(&self) -> SyncpolR {
        SyncpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O Default mode"]
    #[inline(always)]
    pub fn iodef(&self) -> IodefR {
        IodefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max count value"]
    #[inline(always)]
    pub fn mcv(&self) -> McvR {
        McvR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 12:14 - pulse generator prescaler"]
    #[inline(always)]
    pub fn pgpsc(&self) -> PgpscR {
        PgpscR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler"]
    #[inline(always)]
    pub fn sspsc(&self) -> SspscR {
        SspscR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Spread spectrum enable"]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation"]
    #[inline(always)]
    pub fn ssd(&self) -> SsdR {
        SsdR::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low"]
    #[inline(always)]
    pub fn ctpl(&self) -> CtplR {
        CtplR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Charge transfer pulse high"]
    #[inline(always)]
    pub fn ctph(&self) -> CtphR {
        CtphR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Touch sensing controller enable"]
    #[inline(always)]
    pub fn tsce(&mut self) -> TsceW<'_, CrSpec> {
        TsceW::new(self, 0)
    }
    #[doc = "Bit 1 - Start a new acquisition"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CrSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bit 2 - Acquisition mode"]
    #[inline(always)]
    pub fn am(&mut self) -> AmW<'_, CrSpec> {
        AmW::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization pin polarity"]
    #[inline(always)]
    pub fn syncpol(&mut self) -> SyncpolW<'_, CrSpec> {
        SyncpolW::new(self, 3)
    }
    #[doc = "Bit 4 - I/O Default mode"]
    #[inline(always)]
    pub fn iodef(&mut self) -> IodefW<'_, CrSpec> {
        IodefW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Max count value"]
    #[inline(always)]
    pub fn mcv(&mut self) -> McvW<'_, CrSpec> {
        McvW::new(self, 5)
    }
    #[doc = "Bits 12:14 - pulse generator prescaler"]
    #[inline(always)]
    pub fn pgpsc(&mut self) -> PgpscW<'_, CrSpec> {
        PgpscW::new(self, 12)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler"]
    #[inline(always)]
    pub fn sspsc(&mut self) -> SspscW<'_, CrSpec> {
        SspscW::new(self, 15)
    }
    #[doc = "Bit 16 - Spread spectrum enable"]
    #[inline(always)]
    pub fn sse(&mut self) -> SseW<'_, CrSpec> {
        SseW::new(self, 16)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation"]
    #[inline(always)]
    pub fn ssd(&mut self) -> SsdW<'_, CrSpec> {
        SsdW::new(self, 17)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low"]
    #[inline(always)]
    pub fn ctpl(&mut self) -> CtplW<'_, CrSpec> {
        CtplW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Charge transfer pulse high"]
    #[inline(always)]
    pub fn ctph(&mut self) -> CtphW<'_, CrSpec> {
        CtphW::new(self, 28)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
