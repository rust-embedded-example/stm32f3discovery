#[doc = "Register `PECR` reader"]
pub type R = crate::R<PecrSpec>;
#[doc = "Field `PEC` reader - Packet error checking register"]
pub type PecR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PEC register\n\nYou can [`read`](crate::Reg::read) this register and get [`pecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PecrSpec;
impl crate::RegisterSpec for PecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecr::R`](R) reader structure"]
impl crate::Readable for PecrSpec {}
#[doc = "`reset()` method sets PECR to value 0"]
impl crate::Resettable for PecrSpec {}
