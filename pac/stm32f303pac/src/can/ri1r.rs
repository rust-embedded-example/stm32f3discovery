#[doc = "Register `RI1R` reader"]
pub type R = crate::R<Ri1rSpec>;
#[doc = "Field `RTR` reader - RTR"]
pub type RtrR = crate::BitReader;
#[doc = "Field `IDE` reader - IDE"]
pub type IdeR = crate::BitReader;
#[doc = "Field `EXID` reader - EXID"]
pub type ExidR = crate::FieldReader<u32>;
#[doc = "Field `STID` reader - STID"]
pub type StidR = crate::FieldReader<u16>;
impl R {
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
#[doc = "receive FIFO mailbox identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`ri1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ri1rSpec;
impl crate::RegisterSpec for Ri1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ri1r::R`](R) reader structure"]
impl crate::Readable for Ri1rSpec {}
#[doc = "`reset()` method sets RI1R to value 0"]
impl crate::Resettable for Ri1rSpec {}
