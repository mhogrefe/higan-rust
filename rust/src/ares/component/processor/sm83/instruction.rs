use ares::emulator::types::U3;
use ares::gb::system::{System, ThreadState};
use ares::platform::Platform;
use malachite_base::num::logic::traits::BitBlockAccess;

impl<P: Platform> System<P> {
    // synchronized
    pub fn s_instruction(&mut self) {
        let sync_point = if self.cpu_thread_state == ThreadState::Resuming {
            self.cpu_sync_points.pop()
        } else {
            0
        };
        match sync_point {
            0 | 1 => self.s_instruction_fresh(),
            _ => panic!(),
        }
    }

    fn s_instruction_fresh(&mut self) {
        // ** S1
        match self.s_cpu_operand() {
            0x00 => System::<P>::instruction_nop(),
            0x01 => {
                let mut bc = 0;
                // ** S2
                self.s_instruction_ld_direct_data_16(&mut bc);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(2);
                    return;
                }
                self.cpu.r.set_bc(bc);
            }
            // ** S3
            0x02 => {
                let bc = self.cpu.r.get_bc();
                let a = self.cpu.r.get_a();
                self.s_instruction_ld_indirect_direct(bc, a);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(3);
                    self.cpu_local_u16s.push(bc);
                    self.cpu_local_u8s.push(a);
                }
            }
            0x03 => {
                let mut bc = self.cpu.r.get_bc();
                // ** S4
                self.s_instruction_inc_direct_16(&mut bc);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(4);
                    self.cpu_local_u16s.push(bc);
                    return;
                }
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
                let mut b = 0;
                // ** S5
                self.s_instruction_ld_direct_data_8(&mut b);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(5);
                    return;
                }
                self.cpu.r.set_b(b);
            }
            0x07 => self.instruction_rlca(),
            // ** S6
            0x08 => {
                let sp = self.cpu.r.get_sp();
                self.s_instruction_ld_address_direct_16(sp);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(6);
                    self.cpu_local_u16s.push(sp);
                }
            }
            0x09 => {
                let mut hl = self.cpu.r.get_hl();
                let bc = self.cpu.r.get_bc();
                // ** S7
                self.s_instruction_add_direct_direct_16(&mut hl, bc);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(7);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u16s.push(bc);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x0a => {
                let mut a = self.cpu.r.get_a();
                let bc = self.cpu.r.get_bc();
                // ** S8
                self.s_instruction_ld_direct_indirect(&mut a, bc);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(8);
                    self.cpu_local_u8s.push(a);
                    self.cpu_local_u16s.push(bc);
                    return;
                }
                self.cpu.r.set_a(a);
            }
            0x0b => {
                let mut bc = self.cpu.r.get_bc();
                // ** S9
                self.s_instruction_dec_direct_16(&mut bc);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(9);
                    self.cpu_local_u16s.push(bc);
                    return;
                }
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
                let mut c = 0;
                // ** S10
                self.s_instruction_ld_direct_data_8(&mut c);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(10);
                    return;
                }
                self.cpu.r.set_c(c);
            }
            0x0f => self.instruction_rrca(),
            0x10 => self.instruction_stop(),
            0x11 => {
                let mut de = 0;
                // ** S11
                self.s_instruction_ld_direct_data_16(&mut de);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(11);
                    return;
                }
                self.cpu.r.set_de(de);
            }
            // ** S12
            0x12 => {
                let de = self.cpu.r.get_de();
                let a = self.cpu.r.get_a();
                self.s_instruction_ld_indirect_direct(de, a);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(12);
                    self.cpu_local_u16s.push(de);
                    self.cpu_local_u8s.push(a);
                }
            }
            0x13 => {
                let mut de = self.cpu.r.get_de();
                // ** S13
                self.s_instruction_inc_direct_16(&mut de);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(13);
                    self.cpu_local_u16s.push(de);
                    return;
                }
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
                let mut d = 0;
                // ** S14
                self.s_instruction_ld_direct_data_8(&mut d);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(14);
                    return;
                }
                self.cpu.r.set_d(d);
            }
            0x17 => self.instruction_rla(),
            // ** S15
            0x18 => {
                self.s_instruction_jr_condition_relative(true);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(15);
                }
            }
            0x19 => {
                let mut hl = self.cpu.r.get_hl();
                let de = self.cpu.r.get_de();
                // ** S16
                self.s_instruction_add_direct_direct_16(&mut hl, de);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(16);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u16s.push(de);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x1a => {
                let mut a = self.cpu.r.get_a();
                let de = self.cpu.r.get_de();
                // ** S17
                self.s_instruction_ld_direct_indirect(&mut a, de);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(17);
                    self.cpu_local_u8s.push(a);
                    self.cpu_local_u16s.push(de);
                    return;
                }
                self.cpu.r.set_a(a);
            }
            0x1b => {
                let mut de = self.cpu.r.get_de();
                // ** S18
                self.s_instruction_dec_direct_16(&mut de);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(18);
                    self.cpu_local_u16s.push(de);
                    return;
                }
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
                let mut e = 0;
                // ** S19
                self.s_instruction_ld_direct_data_8(&mut e);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(19);
                    self.cpu_local_u8s.push(e);
                    return;
                }
                self.cpu.r.set_e(e);
            }
            0x1f => self.instruction_rra(),
            // ** S20
            0x20 => {
                let zf = self.cpu.r.get_zf();
                self.s_instruction_jr_condition_relative(!zf);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(20);
                    self.cpu_local_bools.push(zf);
                }
            }
            0x21 => {
                let mut hl = 0;
                // ** S21
                self.s_instruction_ld_direct_data_16(&mut hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(21);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x22 => {
                let mut hl = self.cpu.r.get_hl();
                let a = self.cpu.r.get_a();
                // ** S22
                self.s_instruction_ld_indirect_increment_direct(&mut hl, a);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(22);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(a);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x23 => {
                let mut hl = self.cpu.r.get_hl();
                // ** S23
                self.s_instruction_inc_direct_16(&mut hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(23);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
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
                let mut h = 0;
                // ** S24
                self.s_instruction_ld_direct_data_8(&mut h);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(24);
                    return;
                }
                self.cpu.r.set_h(h);
            }
            0x27 => self.instruction_daa(),
            // ** S25
            0x28 => {
                let zf = self.cpu.r.get_zf();
                self.s_instruction_jr_condition_relative(zf);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(25);
                    self.cpu_local_bools.push(zf);
                }
            }
            0x29 => {
                let mut hl = self.cpu.r.get_hl();
                let hl_copy = hl;
                // ** S26
                self.s_instruction_add_direct_direct_16(&mut hl, hl_copy);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(26);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x2a => {
                let mut a = self.cpu.r.get_a();
                let mut hl = self.cpu.r.get_hl();
                // ** S27
                self.s_instruction_ld_direct_indirect_increment(&mut a, &mut hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(27);
                    self.cpu_local_u8s.push(a);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
                self.cpu.r.set_a(a);
                self.cpu.r.set_hl(hl);
            }
            0x2b => {
                let mut hl = self.cpu.r.get_hl();
                // ** S28
                self.s_instruction_dec_direct_16(&mut hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(28);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
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
                let mut l = 0;
                // ** S29
                self.s_instruction_ld_direct_data_8(&mut l);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(29);
                    return;
                }
                self.cpu.r.set_l(l);
            }
            0x2f => self.instruction_cpl(),
            // ** S30
            0x30 => {
                let cf = self.cpu.r.get_cf();
                self.s_instruction_jr_condition_relative(!cf);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(30);
                    self.cpu_local_bools.push(cf);
                }
            }
            0x31 => {
                let mut sp = 0;
                // ** S31
                self.s_instruction_ld_direct_data_16(&mut sp);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(31);
                    return;
                }
                self.cpu.r.set_sp(sp);
            }
            0x32 => {
                let mut hl = self.cpu.r.get_hl();
                let a = self.cpu.r.get_a();
                // ** S32
                self.s_instruction_ld_indirect_decrement_direct(&mut hl, a);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(32);
                    self.cpu_local_u8s.push(a);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x33 => {
                let mut sp = self.cpu.r.get_sp();
                // ** S33
                self.s_instruction_inc_direct_16(&mut sp);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(33);
                    self.cpu_local_u16s.push(sp);
                    return;
                }
                self.cpu.r.set_sp(sp);
            }
            0x34 => {
                let mut hl = self.cpu.r.get_hl();
                // ** S34
                self.s_instruction_inc_direct_16(&mut hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(34);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x35 => {
                let mut hl = self.cpu.r.get_hl();
                // ** S35
                self.s_instruction_dec_direct_16(&mut hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(35);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            // ** S36
            0x36 => {
                let hl = self.cpu.r.get_hl();
                self.s_instruction_ld_indirect_data(hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(36);
                    self.cpu_local_u16s.push(hl);
                }
            }
            0x37 => self.instruction_scf(),
            // ** S37
            0x38 => {
                let cf = self.cpu.r.get_cf();
                self.s_instruction_jr_condition_relative(cf);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(37);
                    self.cpu_local_bools.push(cf);
                }
            },
            0x39 => {
                let mut hl = self.cpu.r.get_hl();
                let sp = self.cpu.r.get_sp();
                // ** S38
                self.s_instruction_add_direct_direct_16(&mut hl, sp);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(38);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u16s.push(sp);
                    return;
                }
                self.cpu.r.set_hl(hl);
            }
            0x3a => {
                let mut a = self.cpu.r.get_a();
                let mut hl = self.cpu.r.get_hl();
                // ** S39
                self.s_instruction_ld_direct_indirect_decrement(&mut a, &mut hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(39);
                    self.cpu_local_u8s.push(a);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
                self.cpu.r.set_a(a);
                self.cpu.r.set_hl(hl);
            }
            0x3b => {
                let mut sp = self.cpu.r.get_sp();
                // ** S40
                self.s_instruction_dec_direct_16(&mut sp);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(40);
                    self.cpu_local_u16s.push(sp);
                    return;
                }
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
                let mut a = 0;
                // ** S41
                self.s_instruction_ld_direct_data_8(&mut a);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(41);
                    return;
                }
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
                let mut b = 0;
                let hl = self.cpu.r.get_hl();
                // ** S42
                self.s_instruction_ld_direct_indirect(&mut b, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(42);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
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
                let mut c = 0;
                let hl = self.cpu.r.get_hl();
                // ** S43
                self.s_instruction_ld_direct_indirect(&mut c, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(43);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
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
                let hl = self.cpu.r.get_hl();
                // ** S44
                self.s_instruction_ld_direct_indirect(&mut d, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(44);
                    self.cpu_local_u8s.push(d);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
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
                let hl = self.cpu.r.get_hl();
                // ** S45
                self.s_instruction_ld_direct_indirect(&mut e, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(45);
                    self.cpu_local_u8s.push(e);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
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
                let hl = self.cpu.r.get_hl();
                // ** S46
                self.s_instruction_ld_direct_indirect(&mut h, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(46);
                    self.cpu_local_u8s.push(h);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
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
                let hl = self.cpu.r.get_hl();
                // ** S47
                self.s_instruction_ld_direct_indirect(&mut l, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(47);
                    self.cpu_local_u8s.push(l);
                    self.cpu_local_u16s.push(hl);
                    return;
                }
                self.cpu.r.set_l(l);
            }
            0x6f => {
                let mut l = self.cpu.r.get_l();
                System::<P>::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_a());
                self.cpu.r.set_l(l);
            }
            // ** S48
            0x70 => {
                let hl = self.cpu.r.get_hl();
                let b = self.cpu.r.get_b();
                self.s_instruction_ld_indirect_direct(hl, b);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(48);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(b);
                }
            }
            // ** S49
            0x71 => {
                let hl = self.cpu.r.get_hl();
                let c = self.cpu.r.get_c();
                self.s_instruction_ld_indirect_direct(hl, c);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(49);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(c);
                }
            }
            // ** S50
            0x72 => {
                let hl = self.cpu.r.get_hl();
                let d = self.cpu.r.get_d();
                self.s_instruction_ld_indirect_direct(hl, d);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(50);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(d);
                }
            }
            // ** S51
            0x73 => {
                let hl = self.cpu.r.get_hl();
                let e = self.cpu.r.get_e();
                self.s_instruction_ld_indirect_direct(hl, e);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(51);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(e);
                }
            }
            // ** S52
            0x74 => {
                let hl = self.cpu.r.get_hl();
                let h = self.cpu.r.get_h();
                self.s_instruction_ld_indirect_direct(hl, h);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(52);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(h);
                }
            }
            // ** S53
            0x75 => {
                let hl = self.cpu.r.get_hl();
                let l = self.cpu.r.get_l();
                self.s_instruction_ld_indirect_direct(hl, l);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(53);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(l);
                }
            }
            // ** S54
            0x76 => {
                self.s_instruction_halt();
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(54);
                }
            },
            // ** S55
            0x77 => {
                let hl = self.cpu.r.get_hl();
                let a = self.cpu.r.get_b();
                self.s_instruction_ld_indirect_direct(hl, a);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(55);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(a);
                }
            }
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
            // ** S56
            0x7e => {
                let mut a = self.cpu.r.get_a();
                let hl = self.cpu.r.get_hl();
                self.s_instruction_ld_direct_indirect(&mut a, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(56);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(a);
                }
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
            // ** S57
            0x86 => {
                let mut a = self.cpu.r.get_a();
                let hl = self.cpu.r.get_hl();
                self.s_instruction_add_direct_indirect(&mut a, hl);
                if self.cpu_thread_state == ThreadState::Pausing {
                    self.cpu_sync_points.push(57);
                    self.cpu_local_u16s.push(hl);
                    self.cpu_local_u8s.push(a);
                }
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
                // ** S58 TODO
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
                // ** S59
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
                // ** S60
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
                // ** S61
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
                // ** S62
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
                // ** S63
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
            // ** S64
            0xbe => self.s_instruction_cp_direct_indirect(self.cpu.r.get_a(), self.cpu.r.get_hl()),
            0xbf => self.instruction_cp_direct_direct(self.cpu.r.get_a(), self.cpu.r.get_a()),
            // ** S65
            0xc0 => self.s_instruction_ret_condition(!self.cpu.r.get_zf()),
            0xc1 => {
                let mut bc = self.cpu.r.get_bc();
                // ** S66
                self.s_instruction_pop_direct(&mut bc);
                self.cpu.r.set_bc(bc);
            }
            // ** S67
            0xc2 => self.s_instruction_jp_condition_address(!self.cpu.r.get_zf()),
            // ** S68
            0xc3 => self.s_instruction_jp_condition_address(true),
            // ** S69
            0xc4 => self.s_instruction_call_condition_address(!self.cpu.r.get_zf()),
            // ** S70
            0xc5 => self.s_instruction_push_direct(self.cpu.r.get_bc()),
            0xc6 => {
                let mut a = self.cpu.r.get_a();
                // ** S71
                self.s_instruction_add_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S72
            0xc7 => self.s_instruction_rst_implied(0),
            // ** S73
            0xc8 => self.s_instruction_ret_condition(self.cpu.r.get_zf()),
            // ** S74
            0xc9 => self.s_instruction_ret(),
            // ** S75
            0xca => self.s_instruction_jp_condition_address(self.cpu.r.get_zf()),
            // ** S76
            0xcb => self.s_instruction_cb(),
            // ** S77
            0xcc => self.s_instruction_call_condition_address(self.cpu.r.get_zf()),
            // ** S78
            0xcd => self.s_instruction_call_condition_address(true),
            0xce => {
                let mut a = self.cpu.r.get_a();
                // ** S79
                self.s_instruction_adc_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S80
            0xcf => self.s_instruction_rst_implied(0x08),
            // ** S81
            0xd0 => self.s_instruction_ret_condition(!self.cpu.r.get_cf()),
            0xd1 => {
                let mut de = self.cpu.r.get_de();
                // ** S82
                self.s_instruction_pop_direct(&mut de);
                self.cpu.r.set_de(de);
            }
            // ** S83
            0xd2 => self.s_instruction_jp_condition_address(!self.cpu.r.get_cf()),
            // ** S84
            0xd4 => self.s_instruction_call_condition_address(!self.cpu.r.get_cf()),
            // ** S85
            0xd5 => self.s_instruction_push_direct(self.cpu.r.get_de()),
            0xd6 => {
                let mut a = self.cpu.r.get_a();
                // ** S86
                self.s_instruction_sub_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S87
            0xd7 => self.s_instruction_rst_implied(0x10),
            // ** S88
            0xd8 => self.s_instruction_ret_condition(self.cpu.r.get_cf()),
            // ** S89
            0xd9 => self.s_instruction_reti(),
            // ** S90
            0xda => self.s_instruction_jp_condition_address(self.cpu.r.get_cf()),
            // ** S91
            0xdc => self.s_instruction_call_condition_address(self.cpu.r.get_cf()),
            0xde => {
                let mut a = self.cpu.r.get_a();
                // ** S92
                self.s_instruction_sbc_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S93
            0xdf => self.s_instruction_rst_implied(0x18),
            // ** S94
            0xe0 => self.s_instruction_ldh_address_direct(self.cpu.r.get_a()),
            0xe1 => {
                let mut hl = self.cpu.r.get_hl();
                // ** S95
                self.s_instruction_pop_direct(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            // ** S96
            0xe2 => self.s_instruction_ldh_indirect_direct(self.cpu.r.get_c(), self.cpu.r.get_a()),
            // ** S97
            0xe5 => self.s_instruction_push_direct(self.cpu.r.get_hl()),
            0xe6 => {
                let mut a = self.cpu.r.get_a();
                // ** S98
                self.s_instruction_and_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S99
            0xe7 => self.s_instruction_rst_implied(0x20),
            0xe8 => {
                let mut sp = self.cpu.r.get_sp();
                // ** S100
                self.s_instruction_add_direct_relative(&mut sp);
                self.cpu.r.set_sp(sp);
            }
            0xe9 => self.instruction_jp_direct(self.cpu.r.get_hl()),
            // ** S101
            0xea => self.s_instruction_ld_address_direct_8(self.cpu.r.get_a()),
            0xee => {
                let mut a = self.cpu.r.get_a();
                // ** S102
                self.s_instruction_xor_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S103
            0xef => self.s_instruction_rst_implied(0x28),
            0xf0 => {
                let mut a = self.cpu.r.get_a();
                // ** S104
                self.s_instruction_ldh_direct_address(&mut a);
                self.cpu.r.set_a(a);
            }
            0xf1 => {
                let mut af = self.cpu.r.get_af();
                // ** S105
                self.s_instruction_pop_direct_af(&mut af);
                self.cpu.r.set_af(af);
            }
            0xf2 => {
                let mut a = self.cpu.r.get_a();
                // ** S106
                self.s_instruction_ldh_direct_indirect(&mut a, self.cpu.r.get_c());
                self.cpu.r.set_a(a);
            }
            0xf3 => self.instruction_di(),
            // ** S107
            0xf5 => self.s_instruction_push_direct(self.cpu.r.get_af()),
            0xf6 => {
                let mut a = self.cpu.r.get_a();
                // ** S108
                self.s_instruction_or_direct_data(&mut a);
                self.cpu.r.set_a(a);
            }
            // ** S109
            0xf7 => self.s_instruction_rst_implied(0x30),
            0xf8 => {
                let mut hl = self.cpu.r.get_hl();
                // ** S110
                self.s_instruction_ld_direct_direct_relative(&mut hl, self.cpu.r.get_sp());
                self.cpu.r.set_hl(hl);
            }
            0xf9 => {
                let mut sp = self.cpu.r.get_sp();
                // ** S111
                self.s_instruction_ld_direct_direct_16(&mut sp, self.cpu.r.get_hl());
                self.cpu.r.set_sp(sp);
            }
            0xfa => {
                let mut a = self.cpu.r.get_a();
                // ** S112
                self.s_instruction_ld_direct_address(&mut a);
                self.cpu.r.set_a(a);
            }
            0xfb => self.instruction_ei(),
            // ** S113
            0xfe => self.s_instruction_cp_direct_data(self.cpu.r.get_a()),
            // ** S114
            0xff => self.s_instruction_rst_implied(0x38),
            _ => {}
        }
    }

    /*
    fn s_instruction_resume_at_2(&mut self) {
            let mut bc = self.cpu.r.get_bc();
            // ** S2
            self.s_instruction_ld_direct_data_16(&mut bc);
            self.cpu.r.set_bc(bc);
    }*/

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
