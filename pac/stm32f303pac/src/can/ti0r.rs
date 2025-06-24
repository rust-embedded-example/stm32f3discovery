#[doc = "Register `TI0R` reader"]
pub type R = crate::R<Ti0rSpec>;
#[doc = "Register `TI0R` writer"]
pub type W = crate::W<Ti0rSpec>;
#[doc = "Field `TXRQ` reader - TXRQ"]
pub type TxrqR = crate::BitReader;
#[doc = "Field `TXRQ` writer - TXRQ"]
pub type TxrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTR` reader - RTR"]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - RTR"]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDE` reader - IDE"]
pub type IdeR = crate::BitReader;
#[doc = "Field `IDE` writer - IDE"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXID` reader - EXID"]
pub type ExidR = crate::FieldReader<u32>;
#[doc = "Field `EXID` writer - EXID"]
pub type ExidW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `STID` reader - STID"]
pub type StidR = crate::FieldReader<u16>;
#[doc = "Field `STID` writer - STID"]
pub type StidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TxrqR {
        TxrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> ExidR {
        ExidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> StidR {
        StidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&mut self) -> TxrqW<Ti0rSpec> {
        TxrqW::new(self, 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RtrW<Ti0rSpec> {
        RtrW::new(self, 1)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&mut self) -> IdeW<Ti0rSpec> {
        IdeW::new(self, 2)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&mut self) -> ExidW<Ti0rSpec> {
        ExidW::new(self, 3)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&mut self) -> StidW<Ti0rSpec> {
        StidW::new(self, 21)
    }
}
#[doc = "TX mailbox identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ti0rSpec;
impl crate::RegisterSpec for Ti0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti0r::R`](R) reader structure"]
impl crate::Readable for Ti0rSpec {}
#[doc = "`write(|w| ..)` method takes [`ti0r::W`](W) writer structure"]
impl crate::Writable for Ti0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TI0R to value 0"]
impl crate::Resettable for Ti0rSpec {}
