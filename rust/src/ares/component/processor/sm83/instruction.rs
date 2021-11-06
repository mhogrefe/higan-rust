use ares::emulator::types::U3;
use ares::gb::system::{System, ThreadState};
use ares::platform::Platform;
use malachite_base::num::logic::traits::BitBlockAccess;

impl<P: Platform> System<P> {
    pub fn s_instruction(&mut self) {
        match self.s_cpu_operand() {
            0x00 => System::<P>::instruction_nop(),
            0x01 => {
                let mut bc = self.cpu.r.get_bc();
                self.s_instruction_ld_direct_data_16(&mut bc);
                self.cpu.r.set_bc(bc);
            }
            0x02 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_bc(), self.cpu.r.get_a()),
            0x03 => {
                let mut bc = self.cpu.r.get_bc();
                self.s_instruction_inc_direct_16(&mut bc);
                self.cpu.r.set_bc(bc);
            }
            0x04 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_inc_direct_8(&mut b);
                self.cpu.r.set_b(b);
            }
            0x05 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_dec_direct_8(&mut b);
                self.cpu.r.set_b(b);
            }
            0x06 => {
                let mut b = self.cpu.r.get_b();
                self.s_instruction_ld_direct_data_8(&mut b);
                self.cpu.r.set_b(b);
            }
            0x07 => self.instruction_rlca(),
            0x08 => self.s_instruction_ld_address_direct_16(self.cpu.r.get_sp()),
            0x09 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_add_direct_direct_16(&mut hl, self.cpu.r.get_bc());
                self.cpu.r.set_hl(hl);
            }
            0x0a => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_ld_direct_indirect(&mut a, self.cpu.r.get_bc());
                self.cpu.r.set_a(a);
            }
            0x0b => {
                let mut bc = self.cpu.r.get_bc();
                self.s_instruction_dec_direct_16(&mut bc);
                self.cpu.r.set_bc(bc);
            }
            0x0c => {
                let mut c = self.cpu.r.get_c();
                self.instruction_inc_direct_8(&mut c);
                self.cpu.r.set_c(c);
            }
            0x0d => {
                let mut c = self.cpu.r.get_c();
                self.instruction_dec_direct_8(&mut c);
                self.cpu.r.set_c(c);
            }
            0x0e => {
                let mut c = self.cpu.r.get_c();
                self.s_instruction_ld_direct_data_8(&mut c);
                self.cpu.r.set_c(c);
            }
            0x0f => self.instruction_rrca(),
            0x10 => self.instruction_stop(),
            0x11 => {
                let mut de = self.cpu.r.get_de();
                self.s_instruction_ld_direct_data_16(&mut de);
                self.cpu.r.set_de(de);
            }
            0x12 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_de(), self.cpu.r.get_a()),
            0x13 => {
                let mut de = self.cpu.r.get_de();
                self.s_instruction_inc_direct_16(&mut de);
                self.cpu.r.set_de(de);
            }
            0x14 => {
                let mut d = self.cpu.r.get_d();
                self.instruction_inc_direct_8(&mut d);
                self.cpu.r.set_d(d);
            }
            0x15 => {
                let mut d = self.cpu.r.get_d();
                self.instruction_dec_direct_8(&mut d);
                self.cpu.r.set_d(d);
            }
            0x16 => {
                let mut d = self.cpu.r.get_d();
                self.s_instruction_ld_direct_data_8(&mut d);
                self.cpu.r.set_d(d);
            }
            0x17 => self.instruction_rla(),
            0x18 => self.s_instruction_jr_condition_relative(true),
            0x19 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_add_direct_direct_16(&mut hl, self.cpu.r.get_de());
                self.cpu.r.set_hl(hl);
            }
            0x1a => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_ld_direct_indirect(&mut a, self.cpu.r.get_de());
                self.cpu.r.set_a(a);
            }
            0x1b => {
                let mut de = self.cpu.r.get_de();
                self.s_instruction_dec_direct_16(&mut de);
                self.cpu.r.set_de(de);
            }
            0x1c => {
                let mut e = self.cpu.r.get_e();
                self.instruction_inc_direct_8(&mut e);
                self.cpu.r.set_e(e);
            }
            0x1d => {
                let mut e = self.cpu.r.get_e();
                self.instruction_dec_direct_8(&mut e);
                self.cpu.r.set_e(e);
            }
            0x1e => {
                let mut e = self.cpu.r.get_e();
                self.s_instruction_ld_direct_data_8(&mut e);
                self.cpu.r.set_e(e);
            }
            0x1f => self.instruction_rra(),
            0x20 => self.s_instruction_jr_condition_relative(!self.cpu.r.get_zf()),
            0x21 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_ld_direct_data_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x22 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_ld_indirect_increment_direct(&mut hl, self.cpu.r.get_a());
                self.cpu.r.set_hl(hl);
            }
            0x23 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_inc_direct_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x24 => {
                let mut h = self.cpu.r.get_h();
                self.instruction_inc_direct_8(&mut h);
                self.cpu.r.set_h(h);
            }
            0x25 => {
                let mut h = self.cpu.r.get_h();
                self.instruction_dec_direct_8(&mut h);
                self.cpu.r.set_h(h);
            }
            0x26 => {
                let mut h = self.cpu.r.get_h();
                self.s_instruction_ld_direct_data_8(&mut h);
                self.cpu.r.set_h(h);
            }
            0x27 => self.instruction_daa(),
            0x28 => self.s_instruction_jr_condition_relative(self.cpu.r.get_zf()),
            0x29 => {
                let mut hl = self.cpu.r.get_hl();
                let hl_copy = hl;
                self.s_instruction_add_direct_direct_16(&mut hl, hl_copy);
                self.cpu.r.set_hl(hl);
            }
            0x2a => {
                let mut a = self.cpu.r.get_a();
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_ld_direct_indirect_increment(&mut a, &mut hl);
                self.cpu.r.set_a(a);
                self.cpu.r.set_hl(hl);
            }
            0x2b => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_dec_direct_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x2c => {
                let mut l = self.cpu.r.get_l();
                self.instruction_inc_direct_8(&mut l);
                self.cpu.r.set_l(l);
            }
            0x2d => {
                let mut l = self.cpu.r.get_l();
                self.instruction_dec_direct_8(&mut l);
                self.cpu.r.set_l(l);
            }
            0x2e => {
                let mut l = self.cpu.r.get_l();
                self.s_instruction_ld_direct_data_8(&mut l);
                self.cpu.r.set_l(l);
            }
            0x2f => self.instruction_cpl(),
            0x30 => self.s_instruction_jr_condition_relative(!self.cpu.r.get_cf()),
            0x31 => {
                let mut sp = self.cpu.r.get_sp();
                self.s_instruction_ld_direct_data_16(&mut sp);
                self.cpu.r.set_sp(sp);
            }
            0x32 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_ld_indirect_decrement_direct(&mut hl, self.cpu.r.get_a());
                self.cpu.r.set_hl(hl);
            }
            0x33 => {
                let mut sp = self.cpu.r.get_sp();
                self.s_instruction_inc_direct_16(&mut sp);
                self.cpu.r.set_sp(sp);
            }
            0x34 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_inc_direct_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x35 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_dec_direct_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x36 => self.s_instruction_ld_indirect_data(self.cpu.r.get_hl()),
            0x37 => self.instruction_scf(),
            0x38 => self.s_instruction_jr_condition_relative(self.cpu.r.get_cf()),
            0x39 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_add_direct_direct_16(&mut hl, self.cpu.r.get_sp());
                self.cpu.r.set_hl(hl);
            }
            0x3a => {
                let mut a = self.cpu.r.get_a();
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_ld_direct_indirect_decrement(&mut a, &mut hl);
                self.cpu.r.set_a(a);
                self.cpu.r.set_hl(hl);
            }
            0x3b => {
                let mut sp = self.cpu.r.get_sp();
                self.s_instruction_dec_direct_16(&mut sp);
                self.cpu.r.set_sp(sp);
            }
            0x3c => {
                let mut a = self.cpu.r.get_a();
                self.instruction_inc_direct_8(&mut a);
                self.cpu.r.set_a(a);
            }
            0x3d => {
                let mut a = self.cpu.r.get_a();
                self.instruction_dec_direct_8(&mut a);
                self.cpu.r.set_a(a);
            }
            0x3e => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_ld_direct_data_8(&mut a);
                self.cpu.r.set_a(a);
            }
            0x3f => self.instruction_ccf(),
            0x40 => {
                let mut b = self.cpu.r.get_b();
                let b_copy = b;
                System::<P>::instruction_ld_direct_direct_8(&mut b, b_copy);
                self.cpu.r.set_b(b);
            }
            0x41 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_c());
                self.cpu.r.set_b(b);
            }
            0x42 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_d());
                self.cpu.r.set_b(b);
            }
            0x43 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_e());
                self.cpu.r.set_b(b);
            }
            0x44 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_h());
                self.cpu.r.set_b(b);
            }
            0x45 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_l());
                self.cpu.r.set_b(b);
            }
            0x46 => {
                let mut b = self.cpu.r.get_b();
                self.s_instruction_ld_direct_indirect(&mut b, self.cpu.r.get_hl());
                self.cpu.r.set_b(b);
            }
            0x47 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_a());
                self.cpu.r.set_b(b);
            }
            0x48 => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_b());
                self.cpu.r.set_c(c);
            }
            0x49 => {
                let mut c = self.cpu.r.get_c();
                let c_copy = c;
                System::<P>::instruction_ld_direct_direct_8(&mut c, c_copy);
                self.cpu.r.set_c(c);
            }
            0x4a => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_d());
                self.cpu.r.set_c(c);
            }
            0x4b => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_e());
                self.cpu.r.set_c(c);
            }
            0x4c => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_h());
                self.cpu.r.set_c(c);
            }
            0x4d => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_l());
                self.cpu.r.set_c(c);
            }
            0x4e => {
                let mut c = self.cpu.r.get_c();
                self.s_instruction_ld_direct_indirect(&mut c, self.cpu.r.get_hl());
                self.cpu.r.set_c(c);
            }
            0x4f => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_a());
                self.cpu.r.set_c(c);
            }
            0x50 => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_b());
                self.cpu.r.set_d(d);
            }
            0x51 => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_c());
                self.cpu.r.set_d(d);
            }
            0x52 => {
                let mut d = self.cpu.r.get_d();
                let d_copy = d;
                System::<P>::instruction_ld_direct_direct_8(&mut d, d_copy);
                self.cpu.r.set_d(d);
            }
            0x53 => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_e());
                self.cpu.r.set_d(d);
            }
            0x54 => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_h());
                self.cpu.r.set_d(d);
            }
            0x55 => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_l());
                self.cpu.r.set_d(d);
            }
            0x56 => {
                let mut d = self.cpu.r.get_d();
                self.s_instruction_ld_direct_indirect(&mut d, self.cpu.r.get_hl());
                self.cpu.r.set_d(d);
            }
            0x57 => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_a());
                self.cpu.r.set_d(d);
            }
            0x58 => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_b());
                self.cpu.r.set_e(e);
            }
            0x59 => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_c());
                self.cpu.r.set_e(e);
            }
            0x5a => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_d());
                self.cpu.r.set_e(e);
            }
            0x5b => {
                let mut e = self.cpu.r.get_e();
                let e_copy = e;
                System::<P>::instruction_ld_direct_direct_8(&mut e, e_copy);
                self.cpu.r.set_e(e);
            }
            0x5c => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_h());
                self.cpu.r.set_e(e);
            }
            0x5d => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_l());
                self.cpu.r.set_e(e);
            }
            0x5e => {
                let mut e = self.cpu.r.get_e();
                self.s_instruction_ld_direct_indirect(&mut e, self.cpu.r.get_hl());
                self.cpu.r.set_e(e);
            }
            0x5f => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_a());
                self.cpu.r.set_e(e);
            }
            0x60 => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_b());
                self.cpu.r.set_h(h);
            }
            0x61 => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_c());
                self.cpu.r.set_h(h);
            }
            0x62 => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_d());
                self.cpu.r.set_h(h);
            }
            0x63 => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_e());
                self.cpu.r.set_h(h);
            }
            0x64 => {
                let mut h = self.cpu.r.get_h();
                let h_copy = h;
                System::<P>::instruction_ld_direct_direct_8(&mut h, h_copy);
                self.cpu.r.set_h(h);
            }
            0x65 => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_l());
                self.cpu.r.set_h(h);
            }
            0x66 => {
                let mut h = self.cpu.r.get_h();
                self.s_instruction_ld_direct_indirect(&mut h, self.cpu.r.get_hl());
                self.cpu.r.set_h(h);
            }
            0x67 => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_a());
                self.cpu.r.set_h(h);
            }
            0x68 => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_b());
                self.cpu.r.set_l(l);
            }
            0x69 => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_c());
                self.cpu.r.set_l(l);
            }
            0x6a => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_d());
                self.cpu.r.set_l(l);
            }
            0x6b => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_e());
                self.cpu.r.set_l(l);
            }
            0x6c => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_e());
                self.cpu.r.set_l(l);
            }
            0x6d => {
                let mut l = self.cpu.r.get_l();
                let l_copy = l;
                System::<P>::instruction_ld_direct_direct_8(&mut l, l_copy);
                self.cpu.r.set_l(l);
            }
            0x6e => {
                let mut l = self.cpu.r.get_l();
                self.s_instruction_ld_direct_indirect(&mut l, self.cpu.r.get_hl());
                self.cpu.r.set_l(l);
            }
            0x6f => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_a());
                self.cpu.r.set_l(l);
            }
            0x70 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_hl(), self.cpu.r.get_b()),
            0x71 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_hl(), self.cpu.r.get_c()),
            0x72 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_hl(), self.cpu.r.get_d()),
            0x73 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_hl(), self.cpu.r.get_e()),
            0x74 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_hl(), self.cpu.r.get_h()),
            0x75 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_hl(), self.cpu.r.get_l()),
            0x76 => self.s_instruction_halt(),
            0x77 => self.s_instruction_ld_indirect_direct(self.cpu.r.get_hl(), self.cpu.r.get_a()),
            0x78 => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_ld_direct_direct_8(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0x79 => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_ld_direct_direct_8(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0x7a => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_ld_direct_direct_8(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0x7b => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_ld_direct_direct_8(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0x7c => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_ld_direct_direct_8(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0x7d => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_ld_direct_direct_8(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0x7e => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_ld_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0x7f => {
                // This is a no-op
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                System::<P>::instruction_ld_direct_direct_8(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0x80 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_add_direct_direct_8(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0x81 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_add_direct_direct_8(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0x82 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_add_direct_direct_8(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0x83 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_add_direct_direct_8(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0x84 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_add_direct_direct_8(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0x85 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_add_direct_direct_8(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0x86 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_add_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0x87 => {
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                self.instruction_add_direct_direct_8(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0x88 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_adc_direct_direct(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0x89 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_adc_direct_direct(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0x8a => {
                let mut a = self.cpu.r.get_a();
                self.instruction_adc_direct_direct(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0x8b => {
                let mut a = self.cpu.r.get_a();
                self.instruction_adc_direct_direct(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0x8c => {
                let mut a = self.cpu.r.get_a();
                self.instruction_adc_direct_direct(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0x8d => {
                let mut a = self.cpu.r.get_a();
                self.instruction_adc_direct_direct(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0x8e => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_adc_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0x8f => {
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                self.instruction_adc_direct_direct(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0x90 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sub_direct_direct(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0x91 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sub_direct_direct(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0x92 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sub_direct_direct(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0x93 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sub_direct_direct(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0x94 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sub_direct_direct(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0x95 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sub_direct_direct(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0x96 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_sub_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0x97 => {
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                self.instruction_sub_direct_direct(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0x98 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sbc_direct_direct(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0x99 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sbc_direct_direct(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0x9a => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sbc_direct_direct(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0x9b => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sbc_direct_direct(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0x9c => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sbc_direct_direct(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0x9d => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sbc_direct_direct(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0x9e => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_sbc_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0x9f => {
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                self.instruction_sbc_direct_direct(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0xa0 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_and_direct_direct(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0xa1 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_and_direct_direct(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0xa2 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_and_direct_direct(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0xa3 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_and_direct_direct(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0xa4 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_and_direct_direct(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0xa5 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_and_direct_direct(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0xa6 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_and_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0xa7 => {
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                self.instruction_and_direct_direct(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0xa8 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_xor_direct_direct(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0xa9 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_xor_direct_direct(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0xaa => {
                let mut a = self.cpu.r.get_a();
                self.instruction_xor_direct_direct(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0xab => {
                let mut a = self.cpu.r.get_a();
                self.instruction_xor_direct_direct(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0xac => {
                let mut a = self.cpu.r.get_a();
                self.instruction_xor_direct_direct(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0xad => {
                let mut a = self.cpu.r.get_a();
                self.instruction_xor_direct_direct(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0xae => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_xor_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0xaf => {
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                self.instruction_xor_direct_direct(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0xb0 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_or_direct_direct(&mut a, self.cpu.r.get_b());
                self.cpu.r.set_a(a);
            }
            0xb1 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_or_direct_direct(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0xb2 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_or_direct_direct(&mut a, self.cpu.r.get_d());
                self.cpu.r.set_a(a);
            }
            0xb3 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_or_direct_direct(&mut a, self.cpu.r.get_e());
                self.cpu.r.set_a(a);
            }
            0xb4 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_or_direct_direct(&mut a, self.cpu.r.get_h());
                self.cpu.r.set_a(a);
            }
            0xb5 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_or_direct_direct(&mut a, self.cpu.r.get_l());
                self.cpu.r.set_a(a);
            }
            0xb6 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_or_direct_indirect(&mut a, self.cpu.r.get_hl());
                self.cpu.r.set_a(a);
            }
            0xb7 => {
                let mut a = self.cpu.r.get_a();
                let a_copy = a;
                self.instruction_or_direct_direct(&mut a, a_copy);
                self.cpu.r.set_a(a);
            }
            0xb8 => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_b()),
            0xb9 => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_c()),
            0xba => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_d()),
            0xbb => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_e()),
            0xbc => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_h()),
            0xbd => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_l()),
            0xbe => self.s_instruction_cp_direct_indirect(self.cpu.r.get_a(), self.cpu.r.get_hl()),
            0xbf => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_a()),
            0xc0 => self.s_instruction_ret_condition(!self.cpu.r.get_zf()),
            0xc1 => {
                let mut bc = self.cpu.r.get_bc();
                self.s_instruction_pop_direct(&mut bc);
                self.cpu.r.set_bc(bc);
            }
            0xc2 => self.s_instruction_jp_condition_address(!self.cpu.r.get_zf()),
            0xc3 => self.s_instruction_jp_condition_address(true),
            0xc4 => self.s_instruction_call_condition_address(!self.cpu.r.get_zf()),
            0xc5 => self.s_instruction_push_direct(self.cpu.r.get_bc()),
            0xc6 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_add_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            0xc7 => self.s_instruction_rst_implied(0),
            0xc8 => self.s_instruction_ret_condition(self.cpu.r.get_zf()),
            0xc9 => self.s_instruction_ret(),
            0xca => self.s_instruction_jp_condition_address(self.cpu.r.get_zf()),
            0xcb => self.s_instruction_cb(),
            0xcc => self.s_instruction_call_condition_address(self.cpu.r.get_zf()),
            0xcd => self.s_instruction_call_condition_address(true),
            0xce => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_adc_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            0xcf => self.s_instruction_rst_implied(0x08),
            0xd0 => self.s_instruction_ret_condition(!self.cpu.r.get_cf()),
            0xd1 => {
                let mut de = self.cpu.r.get_de();
                self.s_instruction_pop_direct(&mut de);
                self.cpu.r.set_de(de);
            }
            0xd2 => self.s_instruction_jp_condition_address(!self.cpu.r.get_cf()),
            0xd4 => self.s_instruction_call_condition_address(!self.cpu.r.get_cf()),
            0xd5 => self.s_instruction_push_direct(self.cpu.r.get_de()),
            0xd6 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_sub_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            0xd7 => self.s_instruction_rst_implied(0x10),
            0xd8 => self.s_instruction_ret_condition(self.cpu.r.get_cf()),
            0xd9 => self.s_instruction_reti(),
            0xda => self.s_instruction_jp_condition_address(self.cpu.r.get_cf()),
            0xdc => self.s_instruction_call_condition_address(self.cpu.r.get_cf()),
            0xde => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_sbc_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            0xdf => self.s_instruction_rst_implied(0x18),
            0xe0 => self.s_instruction_ldh_address_direct(self.cpu.r.get_a()),
            0xe1 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_pop_direct(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0xe2 => self.s_instruction_ldh_indirect_direct(self.cpu.r.get_c(), self.cpu.r.get_a()),
            0xe5 => self.s_instruction_push_direct(self.cpu.r.get_hl()),
            0xe6 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_and_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            0xe7 => self.s_instruction_rst_implied(0x20),
            0xe8 => {
                let mut sp = self.cpu.r.get_sp();
                self.s_instruction_add_direct_relative(&mut sp);
                self.cpu.r.set_sp(sp);
            }
            0xe9 => self.instruction_jp_direct(self.cpu.r.get_hl()),
            0xea => self.s_instruction_ld_address_direct_8(self.cpu.r.get_a()),
            0xee => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_xor_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            0xef => self.s_instruction_rst_implied(0x28),
            0xf0 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_ldh_direct_address(&mut a);
                self.cpu.r.set_a(a);
            }
            0xf1 => {
                let mut af = self.cpu.r.get_af();
                self.s_instruction_pop_direct_af(&mut af);
                self.cpu.r.set_af(af);
            }
            0xf2 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_ldh_direct_indirect(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0xf3 => self.instruction_di(),
            0xf5 => self.s_instruction_push_direct(self.cpu.r.get_af()),
            0xf6 => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_or_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            0xf7 => self.s_instruction_rst_implied(0x30),
            0xf8 => {
                let mut hl = self.cpu.r.get_hl();
                self.s_instruction_ld_direct_direct_relative(&mut hl, self.cpu.r.get_sp());
                self.cpu.r.set_hl(hl);
            }
            0xf9 => {
                let mut sp = self.cpu.r.get_sp();
                self.s_instruction_ld_direct_direct_16(&mut sp, self.cpu.r.get_hl());
                self.cpu.r.set_sp(sp);
            }
            0xfa => {
                let mut a = self.cpu.r.get_a();
                self.s_instruction_ld_direct_address(&mut a);
                self.cpu.r.set_a(a);
            }
            0xfb => self.instruction_ei(),
            0xfe => self.s_instruction_cp_direct_data(self.cpu.r.get_a()),
            0xff => self.s_instruction_rst_implied(0x38),
            _ => {}
        }
    }

    // synchronized
    pub fn s_instruction_cb(&mut self) {
        let sync_point = if self.cpu_thread_state == ThreadState::Resuming {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 | 1 => self.s_instruction_cb_fresh(),
            2 => self.s_instruction_cb_resume_at_2(),
            3 => self.s_instruction_cb_resume_at_3(),
            4 => self.s_instruction_cb_resume_at_4(),
            5 => self.s_instruction_cb_resume_at_5(),
            6 => self.s_instruction_cb_resume_at_6(),
            7 => self.s_instruction_cb_resume_at_7(),
            8 => self.s_instruction_cb_resume_at_8(),
            9 => self.s_instruction_cb_resume_at_9(),
            10 => self.s_instruction_cb_resume_at_10(),
            _ => panic!(),
        }
    }

    fn s_instruction_cb_fresh(&mut self) {
        // ** S1
        let opcode = self.s_cpu_operand();
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(1);
            return;
        }
        match opcode {
            0x00 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_rlc_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x01 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_rlc_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x02 => {
                let mut d = self.cpu.r.get_d();
                self.instruction_rlc_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x03 => {
                let mut e = self.cpu.r.get_e();
                self.instruction_rlc_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x04 => {
                let mut h = self.cpu.r.get_h();
                self.instruction_rlc_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x05 => {
                let mut l = self.cpu.r.get_l();
                self.instruction_rlc_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S2
            0x06 => {
                self.s_instruction_rlc_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(2);
                }
            }
            0x07 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_rlc_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            0x08 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_rrc_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x09 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_rrc_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x0a => {
                let mut d = self.cpu.r.get_d();
                self.instruction_rrc_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x0b => {
                let mut e = self.cpu.r.get_e();
                self.instruction_rrc_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x0c => {
                let mut h = self.cpu.r.get_h();
                self.instruction_rrc_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x0d => {
                let mut l = self.cpu.r.get_l();
                self.instruction_rrc_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S3
            0x0e => {
                self.s_instruction_rrc_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(3);
                }
            }
            0x0f => {
                let mut a = self.cpu.r.get_a();
                self.instruction_rrc_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            0x10 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_rl_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x11 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_rl_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x12 => {
                let mut d = self.cpu.r.get_d();
                self.instruction_rl_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x13 => {
                let mut e = self.cpu.r.get_e();
                self.instruction_rl_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x14 => {
                let mut h = self.cpu.r.get_h();
                self.instruction_rl_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x15 => {
                let mut l = self.cpu.r.get_l();
                self.instruction_rl_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S4
            0x16 => {
                self.s_instruction_rl_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(4);
                }
            }
            0x17 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_rl_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            0x18 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_rr_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x19 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_rr_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x1a => {
                let mut d = self.cpu.r.get_d();
                self.instruction_rr_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x1b => {
                let mut e = self.cpu.r.get_e();
                self.instruction_rr_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x1c => {
                let mut h = self.cpu.r.get_h();
                self.instruction_rr_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x1d => {
                let mut l = self.cpu.r.get_l();
                self.instruction_rr_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S5
            0x1e => {
                self.s_instruction_rr_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(5);
                }
            }
            0x1f => {
                let mut a = self.cpu.r.get_a();
                self.instruction_rr_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            0x20 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_sla_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x21 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_sla_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x22 => {
                let mut d = self.cpu.r.get_d();
                self.instruction_sla_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x23 => {
                let mut e = self.cpu.r.get_e();
                self.instruction_sla_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x24 => {
                let mut h = self.cpu.r.get_h();
                self.instruction_sla_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x25 => {
                let mut l = self.cpu.r.get_l();
                self.instruction_sla_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S6
            0x26 => {
                self.s_instruction_sla_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(6);
                }
            }
            0x27 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sla_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            0x28 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_sra_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x29 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_sra_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x2a => {
                let mut d = self.cpu.r.get_d();
                self.instruction_sra_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x2b => {
                let mut e = self.cpu.r.get_e();
                self.instruction_sra_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x2c => {
                let mut h = self.cpu.r.get_h();
                self.instruction_sra_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x2d => {
                let mut l = self.cpu.r.get_l();
                self.instruction_sra_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S7
            0x2e => {
                self.s_instruction_sra_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(7);
                }
            }
            0x2f => {
                let mut a = self.cpu.r.get_a();
                self.instruction_sra_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            0x30 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_swap_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x31 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_swap_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x32 => {
                let mut d = self.cpu.r.get_d();
                self.instruction_swap_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x33 => {
                let mut e = self.cpu.r.get_e();
                self.instruction_swap_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x34 => {
                let mut h = self.cpu.r.get_h();
                self.instruction_swap_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x35 => {
                let mut l = self.cpu.r.get_l();
                self.instruction_swap_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S8
            0x36 => {
                self.s_instruction_swap_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(8);
                }
            }
            0x37 => {
                let mut a = self.cpu.r.get_a();
                self.instruction_swap_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            0x38 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_srl_direct(&mut b);
                self.cpu.r.set_b(b);
            }
            0x39 => {
                let mut c = self.cpu.r.get_c();
                self.instruction_srl_direct(&mut c);
                self.cpu.r.set_c(c);
            }
            0x3a => {
                let mut d = self.cpu.r.get_d();
                self.instruction_srl_direct(&mut d);
                self.cpu.r.set_d(d);
            }
            0x3b => {
                let mut e = self.cpu.r.get_e();
                self.instruction_srl_direct(&mut e);
                self.cpu.r.set_e(e);
            }
            0x3c => {
                let mut h = self.cpu.r.get_h();
                self.instruction_srl_direct(&mut h);
                self.cpu.r.set_h(h);
            }
            0x3d => {
                let mut l = self.cpu.r.get_l();
                self.instruction_srl_direct(&mut l);
                self.cpu.r.set_l(l);
            }
            // ** S9
            0x3e => {
                self.s_instruction_srl_indirect(self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(9);
                }
            }
            0x3f => {
                let mut a = self.cpu.r.get_a();
                self.instruction_srl_direct(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S10
            _ => {
                self.s_bit_instruction_cb(opcode);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(10);
                    self.cpu_local_u8s.push(opcode);
                }
            }
        }
    }

    fn s_instruction_cb_resume_at_2(&mut self) {
        // ** S2
        self.s_instruction_rlc_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(2);
        }
    }

    fn s_instruction_cb_resume_at_3(&mut self) {
        // ** S3
        self.s_instruction_rrc_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(3);
        }
    }

    fn s_instruction_cb_resume_at_4(&mut self) {
        // ** S4
        self.s_instruction_rl_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(4);
        }
    }

    fn s_instruction_cb_resume_at_5(&mut self) {
        // ** S5
        self.s_instruction_rr_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(5);
        }
    }

    fn s_instruction_cb_resume_at_6(&mut self) {
        // ** S6
        self.s_instruction_sla_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(6);
        }
    }

    fn s_instruction_cb_resume_at_7(&mut self) {
        // ** S7
        self.s_instruction_sra_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(7);
        }
    }

    fn s_instruction_cb_resume_at_8(&mut self) {
        // ** S8
        self.s_instruction_swap_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(8);
        }
    }

    fn s_instruction_cb_resume_at_9(&mut self) {
        // ** S9
        self.s_instruction_srl_indirect(self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(9);
        }
    }

    fn s_instruction_cb_resume_at_10(&mut self) {
        // ** S10
        let opcode = self.cpu_local_u8s.pop();
        self.s_bit_instruction_cb(opcode);
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(10);
            self.cpu_local_u8s.push(opcode);
        }
    }

    // synchronized
    pub fn s_bit_instruction_cb(&mut self, opcode: u8) {
        let sync_point = if self.cpu_thread_state == ThreadState::Resuming {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 => self.s_bit_instruction_cb_fresh(opcode),
            1 => self.s_bit_instruction_cb_resume_at_1(),
            2 => self.s_bit_instruction_cb_resume_at_2(),
            3 => self.s_bit_instruction_cb_resume_at_3(),
            _ => panic!(),
        }
    }

    fn s_bit_instruction_cb_fresh(&mut self, opcode: u8) {
        // opcodes 0x40-0xff [op(0x00 - 0x07) declared above]
        let bit = U3::new(opcode.get_bits(3, 6));
        match opcode.get_bits(6, 8) << 3 | opcode.get_bits(0, 3) {
            0x08 => self.instruction_bit_index_direct(bit, self.cpu.r.get_b()),
            0x09 => self.instruction_bit_index_direct(bit, self.cpu.r.get_c()),
            0x0a => self.instruction_bit_index_direct(bit, self.cpu.r.get_d()),
            0x0b => self.instruction_bit_index_direct(bit, self.cpu.r.get_e()),
            0x0c => self.instruction_bit_index_direct(bit, self.cpu.r.get_h()),
            0x0d => self.instruction_bit_index_direct(bit, self.cpu.r.get_l()),
            // ** S1
            0x0e => {
                self.s_instruction_bit_index_indirect(bit, self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(1);
                    self.cpu_local_u3s.push(bit);
                }
            }
            0x0f => self.instruction_bit_index_direct(bit, self.cpu.r.get_a()),
            0x10 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_res_index_direct(bit, &mut b);
                self.cpu.r.set_b(b);
            }
            0x11 => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_res_index_direct(bit, &mut c);
                self.cpu.r.set_c(c);
            }
            0x12 => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_res_index_direct(bit, &mut d);
                self.cpu.r.set_d(d);
            }
            0x13 => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_res_index_direct(bit, &mut e);
                self.cpu.r.set_e(e);
            }
            0x14 => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_res_index_direct(bit, &mut h);
                self.cpu.r.set_h(h);
            }
            0x15 => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_res_index_direct(bit, &mut l);
                self.cpu.r.set_l(l);
            }
            // ** S2
            0x16 => {
                self.s_instruction_res_index_indirect(bit, self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(2);
                    self.cpu_local_u3s.push(bit);
                }
            }
            0x17 => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_res_index_direct(bit, &mut a);
                self.cpu.r.set_a(a);
            }
            0x18 => {
                let mut b = self.cpu.r.get_b();
                System::<P>::instruction_set_index_direct(bit, &mut b);
                self.cpu.r.set_b(b);
            }
            0x19 => {
                let mut c = self.cpu.r.get_c();
                System::<P>::instruction_set_index_direct(bit, &mut c);
                self.cpu.r.set_c(c);
            }
            0x1a => {
                let mut d = self.cpu.r.get_d();
                System::<P>::instruction_set_index_direct(bit, &mut d);
                self.cpu.r.set_d(d);
            }
            0x1b => {
                let mut e = self.cpu.r.get_e();
                System::<P>::instruction_set_index_direct(bit, &mut e);
                self.cpu.r.set_e(e);
            }
            0x1c => {
                let mut h = self.cpu.r.get_h();
                System::<P>::instruction_set_index_direct(bit, &mut h);
                self.cpu.r.set_h(h);
            }
            0x1d => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_set_index_direct(bit, &mut l);
                self.cpu.r.set_l(l);
            }
            // ** S3
            0x1e => {
                self.s_instruction_set_index_indirect(bit, self.cpu.r.get_hl());
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(3);
                    self.cpu_local_u3s.push(bit);
                }
            }
            0x1f => {
                let mut a = self.cpu.r.get_a();
                System::<P>::instruction_set_index_direct(bit, &mut a);
                self.cpu.r.set_a(a);
            }
            _ => {}
        }
    }

    fn s_bit_instruction_cb_resume_at_1(&mut self) {
        let bit = self.cpu_local_u3s.pop();
        self.s_instruction_bit_index_indirect(bit, self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(1);
            self.cpu_local_u3s.push(bit);
        }
    }

    fn s_bit_instruction_cb_resume_at_2(&mut self) {
        let bit = self.cpu_local_u3s.pop();
        self.s_instruction_res_index_indirect(bit, self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(2);
            self.cpu_local_u3s.push(bit);
        }
    }

    fn s_bit_instruction_cb_resume_at_3(&mut self) {
        let bit = self.cpu_local_u3s.pop();
        self.s_instruction_set_index_indirect(bit, self.cpu.r.get_hl());
        if self.cpu_thread_state == ThreadState::Pausing {
            self.cpu_sync_points.push(3);
            self.cpu_local_u3s.push(bit);
        }
    }
}
