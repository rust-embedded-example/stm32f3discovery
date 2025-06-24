#[doc = "Register `RDL1R` reader"]
pub type R = crate::R<Rdl1rSpec>;
#[doc = "Field `DATA0` reader - DATA0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA1` reader - DATA1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA2` reader - DATA2"]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA3` reader - DATA3"]
pub type Data3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "receive FIFO mailbox data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdl1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rdl1rSpec;
impl crate::RegisterSpec for Rdl1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdl1r::R`](R) reader structure"]
impl crate::Readable for Rdl1rSpec {}
#[doc = "`reset()` method sets RDL1R to value 0"]
impl crate::Resettable for Rdl1rSpec {}
