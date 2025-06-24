#[doc = "Register `DOR2` reader"]
pub type R = crate::R<Dor2Spec>;
#[doc = "Field `DACC2DOR` reader - DAC channel2 data output"]
pub type Dacc2dorR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 data output"]
    #[inline(always)]
    pub fn dacc2dor(&self) -> Dacc2dorR {
        Dacc2dorR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "channel2 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dor2Spec;
impl crate::RegisterSpec for Dor2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor2::R`](R) reader structure"]
impl crate::Readable for Dor2Spec {}
#[doc = "`reset()` method sets DOR2 to value 0"]
impl crate::Resettable for Dor2Spec {}
