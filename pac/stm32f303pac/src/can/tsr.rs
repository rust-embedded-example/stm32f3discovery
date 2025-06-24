#[doc = "Register `TSR` reader"]
pub type R = crate::R<TsrSpec>;
#[doc = "Register `TSR` writer"]
pub type W = crate::W<TsrSpec>;
#[doc = "Field `RQCP0` reader - RQCP0"]
pub type Rqcp0R = crate::BitReader;
#[doc = "Field `RQCP0` writer - RQCP0"]
pub type Rqcp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOK0` reader - TXOK0"]
pub type Txok0R = crate::BitReader;
#[doc = "Field `TXOK0` writer - TXOK0"]
pub type Txok0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALST0` reader - ALST0"]
pub type Alst0R = crate::BitReader;
#[doc = "Field `ALST0` writer - ALST0"]
pub type Alst0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR0` reader - TERR0"]
pub type Terr0R = crate::BitReader;
#[doc = "Field `TERR0` writer - TERR0"]
pub type Terr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRQ0` reader - ABRQ0"]
pub type Abrq0R = crate::BitReader;
#[doc = "Field `ABRQ0` writer - ABRQ0"]
pub type Abrq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RQCP1` reader - RQCP1"]
pub type Rqcp1R = crate::BitReader;
#[doc = "Field `RQCP1` writer - RQCP1"]
pub type Rqcp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOK1` reader - TXOK1"]
pub type Txok1R = crate::BitReader;
#[doc = "Field `TXOK1` writer - TXOK1"]
pub type Txok1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALST1` reader - ALST1"]
pub type Alst1R = crate::BitReader;
#[doc = "Field `ALST1` writer - ALST1"]
pub type Alst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR1` reader - TERR1"]
pub type Terr1R = crate::BitReader;
#[doc = "Field `TERR1` writer - TERR1"]
pub type Terr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRQ1` reader - ABRQ1"]
pub type Abrq1R = crate::BitReader;
#[doc = "Field `ABRQ1` writer - ABRQ1"]
pub type Abrq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RQCP2` reader - RQCP2"]
pub type Rqcp2R = crate::BitReader;
#[doc = "Field `RQCP2` writer - RQCP2"]
pub type Rqcp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOK2` reader - TXOK2"]
pub type Txok2R = crate::BitReader;
#[doc = "Field `TXOK2` writer - TXOK2"]
pub type Txok2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALST2` reader - ALST2"]
pub type Alst2R = crate::BitReader;
#[doc = "Field `ALST2` writer - ALST2"]
pub type Alst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERR2` reader - TERR2"]
pub type Terr2R = crate::BitReader;
#[doc = "Field `TERR2` writer - TERR2"]
pub type Terr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRQ2` reader - ABRQ2"]
pub type Abrq2R = crate::BitReader;
#[doc = "Field `ABRQ2` writer - ABRQ2"]
pub type Abrq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODE` reader - CODE"]
pub type CodeR = crate::FieldReader;
#[doc = "Field `TME0` reader - Lowest priority flag for mailbox 0"]
pub type Tme0R = crate::BitReader;
#[doc = "Field `TME1` reader - Lowest priority flag for mailbox 1"]
pub type Tme1R = crate::BitReader;
#[doc = "Field `TME2` reader - Lowest priority flag for mailbox 2"]
pub type Tme2R = crate::BitReader;
#[doc = "Field `LOW0` reader - Lowest priority flag for mailbox 0"]
pub type Low0R = crate::BitReader;
#[doc = "Field `LOW1` reader - Lowest priority flag for mailbox 1"]
pub type Low1R = crate::BitReader;
#[doc = "Field `LOW2` reader - Lowest priority flag for mailbox 2"]
pub type Low2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RQCP0"]
    #[inline(always)]
    pub fn rqcp0(&self) -> Rqcp0R {
        Rqcp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXOK0"]
    #[inline(always)]
    pub fn txok0(&self) -> Txok0R {
        Txok0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ALST0"]
    #[inline(always)]
    pub fn alst0(&self) -> Alst0R {
        Alst0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TERR0"]
    #[inline(always)]
    pub fn terr0(&self) -> Terr0R {
        Terr0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - ABRQ0"]
    #[inline(always)]
    pub fn abrq0(&self) -> Abrq0R {
        Abrq0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RQCP1"]
    #[inline(always)]
    pub fn rqcp1(&self) -> Rqcp1R {
        Rqcp1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXOK1"]
    #[inline(always)]
    pub fn txok1(&self) -> Txok1R {
        Txok1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ALST1"]
    #[inline(always)]
    pub fn alst1(&self) -> Alst1R {
        Alst1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TERR1"]
    #[inline(always)]
    pub fn terr1(&self) -> Terr1R {
        Terr1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ABRQ1"]
    #[inline(always)]
    pub fn abrq1(&self) -> Abrq1R {
        Abrq1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RQCP2"]
    #[inline(always)]
    pub fn rqcp2(&self) -> Rqcp2R {
        Rqcp2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXOK2"]
    #[inline(always)]
    pub fn txok2(&self) -> Txok2R {
        Txok2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ALST2"]
    #[inline(always)]
    pub fn alst2(&self) -> Alst2R {
        Alst2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TERR2"]
    #[inline(always)]
    pub fn terr2(&self) -> Terr2R {
        Terr2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - ABRQ2"]
    #[inline(always)]
    pub fn abrq2(&self) -> Abrq2R {
        Abrq2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - CODE"]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Lowest priority flag for mailbox 0"]
    #[inline(always)]
    pub fn tme0(&self) -> Tme0R {
        Tme0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lowest priority flag for mailbox 1"]
    #[inline(always)]
    pub fn tme1(&self) -> Tme1R {
        Tme1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Lowest priority flag for mailbox 2"]
    #[inline(always)]
    pub fn tme2(&self) -> Tme2R {
        Tme2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lowest priority flag for mailbox 0"]
    #[inline(always)]
    pub fn low0(&self) -> Low0R {
        Low0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Lowest priority flag for mailbox 1"]
    #[inline(always)]
    pub fn low1(&self) -> Low1R {
        Low1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lowest priority flag for mailbox 2"]
    #[inline(always)]
    pub fn low2(&self) -> Low2R {
        Low2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RQCP0"]
    #[inline(always)]
    pub fn rqcp0(&mut self) -> Rqcp0W<TsrSpec> {
        Rqcp0W::new(self, 0)
    }
    #[doc = "Bit 1 - TXOK0"]
    #[inline(always)]
    pub fn txok0(&mut self) -> Txok0W<TsrSpec> {
        Txok0W::new(self, 1)
    }
    #[doc = "Bit 2 - ALST0"]
    #[inline(always)]
    pub fn alst0(&mut self) -> Alst0W<TsrSpec> {
        Alst0W::new(self, 2)
    }
    #[doc = "Bit 3 - TERR0"]
    #[inline(always)]
    pub fn terr0(&mut self) -> Terr0W<TsrSpec> {
        Terr0W::new(self, 3)
    }
    #[doc = "Bit 7 - ABRQ0"]
    #[inline(always)]
    pub fn abrq0(&mut self) -> Abrq0W<TsrSpec> {
        Abrq0W::new(self, 7)
    }
    #[doc = "Bit 8 - RQCP1"]
    #[inline(always)]
    pub fn rqcp1(&mut self) -> Rqcp1W<TsrSpec> {
        Rqcp1W::new(self, 8)
    }
    #[doc = "Bit 9 - TXOK1"]
    #[inline(always)]
    pub fn txok1(&mut self) -> Txok1W<TsrSpec> {
        Txok1W::new(self, 9)
    }
    #[doc = "Bit 10 - ALST1"]
    #[inline(always)]
    pub fn alst1(&mut self) -> Alst1W<TsrSpec> {
        Alst1W::new(self, 10)
    }
    #[doc = "Bit 11 - TERR1"]
    #[inline(always)]
    pub fn terr1(&mut self) -> Terr1W<TsrSpec> {
        Terr1W::new(self, 11)
    }
    #[doc = "Bit 15 - ABRQ1"]
    #[inline(always)]
    pub fn abrq1(&mut self) -> Abrq1W<TsrSpec> {
        Abrq1W::new(self, 15)
    }
    #[doc = "Bit 16 - RQCP2"]
    #[inline(always)]
    pub fn rqcp2(&mut self) -> Rqcp2W<TsrSpec> {
        Rqcp2W::new(self, 16)
    }
    #[doc = "Bit 17 - TXOK2"]
    #[inline(always)]
    pub fn txok2(&mut self) -> Txok2W<TsrSpec> {
        Txok2W::new(self, 17)
    }
    #[doc = "Bit 18 - ALST2"]
    #[inline(always)]
    pub fn alst2(&mut self) -> Alst2W<TsrSpec> {
        Alst2W::new(self, 18)
    }
    #[doc = "Bit 19 - TERR2"]
    #[inline(always)]
    pub fn terr2(&mut self) -> Terr2W<TsrSpec> {
        Terr2W::new(self, 19)
    }
    #[doc = "Bit 23 - ABRQ2"]
    #[inline(always)]
    pub fn abrq2(&mut self) -> Abrq2W<TsrSpec> {
        Abrq2W::new(self, 23)
    }
}
#[doc = "transmit status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsrSpec;
impl crate::RegisterSpec for TsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tsr::W`](W) writer structure"]
impl crate::Writable for TsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSR to value 0x1c00_0000"]
impl crate::Resettable for TsrSpec {
    const RESET_VALUE: u32 = 0x1c00_0000;
}
