use ares::component::processor::sm83::sm83::SM83;
use malachite_base::num::logic::traits::BitAccess;

/// See higan-rust/cpp/ares/component/processor/sm83/registers.hpp
impl SM83 {
    pub fn get_af(&self) -> u16 {
        self.r.af.get_word()
    }

    pub fn set_af(&mut self, af: u16) {
        self.r.af.set_word(af)
    }

    pub fn get_bc(&self) -> u16 {
        self.r.bc.get_word()
    }

    pub fn set_bc(&mut self, bc: u16) {
        self.r.bc.set_word(bc)
    }

    pub fn get_de(&self) -> u16 {
        self.r.de.get_word()
    }

    pub fn set_de(&mut self, de: u16) {
        self.r.de.set_word(de)
    }

    pub fn get_hl(&self) -> u16 {
        self.r.hl.get_word()
    }

    pub fn set_hl(&mut self, hl: u16) {
        self.r.hl.set_word(hl)
    }

    pub fn get_sp(&self) -> u16 {
        self.r.sp.get_word()
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.r.sp.set_word(sp)
    }

    pub fn get_pc(&self) -> u16 {
        self.r.pc.get_word()
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.r.pc.set_word(pc)
    }

    pub fn get_a(&self) -> u8 {
        self.r.af.get_hi()
    }

    pub fn set_a(&mut self, a: u8) {
        self.r.af.set_hi(a)
    }

    pub fn get_f(&self) -> u8 {
        self.r.af.get_lo()
    }

    pub fn set_f(&mut self, f: u8) {
        self.r.af.set_lo(f)
    }

    pub fn get_b(&self) -> u8 {
        self.r.bc.get_hi()
    }

    pub fn set_b(&mut self, b: u8) {
        self.r.bc.set_hi(b)
    }

    pub fn get_c(&self) -> u8 {
        self.r.bc.get_lo()
    }

    pub fn set_c(&mut self, c: u8) {
        self.r.bc.set_lo(c)
    }

    pub fn get_d(&self) -> u8 {
        self.r.de.get_hi()
    }

    pub fn set_d(&mut self, d: u8) {
        self.r.de.set_hi(d)
    }

    pub fn get_e(&self) -> u8 {
        self.r.de.get_lo()
    }

    pub fn set_e(&mut self, e: u8) {
        self.r.de.set_lo(e)
    }

    pub fn get_h(&self) -> u8 {
        self.r.hl.get_hi()
    }

    pub fn set_h(&mut self, h: u8) {
        self.r.hl.set_hi(h)
    }

    pub fn get_l(&self) -> u8 {
        self.r.hl.get_lo()
    }

    pub fn set_l(&mut self, l: u8) {
        self.r.hl.set_lo(l)
    }

    pub fn get_cf(&self) -> bool {
        self.get_f().get_bit(4)
    }

    pub fn set_cf(&mut self, cf: bool) {
        let mut f = self.get_f();
        f.assign_bit(4, cf);
        self.set_f(f);
    }

    pub fn get_hf(&self) -> bool {
        self.get_f().get_bit(5)
    }

    pub fn set_hf(&mut self, cf: bool) {
        let mut f = self.get_f();
        f.assign_bit(5, cf);
        self.set_f(f);
    }

    pub fn get_nf(&self) -> bool {
        self.get_f().get_bit(6)
    }

    pub fn set_nf(&mut self, cf: bool) {
        let mut f = self.get_f();
        f.assign_bit(6, cf);
        self.set_f(f);
    }

    pub fn get_zf(&self) -> bool {
        self.get_f().get_bit(7)
    }

    pub fn set_zf(&mut self, cf: bool) {
        let mut f = self.get_f();
        f.assign_bit(7, cf);
        self.set_f(f);
    }

    pub fn post_increment_pc(&mut self) -> u16 {
        let pc = self.get_pc();
        self.set_pc(pc.wrapping_add(1));
        pc
    }

    pub fn post_increment_sp(&mut self) -> u16 {
        let sp = self.get_sp();
        self.set_sp(sp.wrapping_add(1));
        sp
    }

    pub fn pre_decrement_sp(&mut self) -> u16 {
        let sp = self.get_sp().wrapping_sub(1);
        self.set_sp(sp);
        sp
    }
}
