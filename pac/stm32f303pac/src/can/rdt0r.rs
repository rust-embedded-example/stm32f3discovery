#[doc = "Register `RDT0R` reader"]
pub type R = crate::R<Rdt0rSpec>;
#[doc = "Field `DLC` reader - DLC"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `FMI` reader - FMI"]
pub type FmiR = crate::FieldReader;
#[doc = "Field `TIME` reader - TIME"]
pub type TimeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FmiR {
        FmiR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "receive FIFO mailbox data length control and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdt0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rdt0rSpec;
impl crate::RegisterSpec for Rdt0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdt0r::R`](R) reader structure"]
impl crate::Readable for Rdt0rSpec {}
#[doc = "`reset()` method sets RDT0R to value 0"]
impl crate::Resettable for Rdt0rSpec {}
