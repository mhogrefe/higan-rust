use ares::emulator::types::U3;
use ares::gb::system::System;
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, WrappingAddAssign, WrappingSubAssign};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

// See higan-rust/rust/src/ares/component/processor/sm83/instructions.rs
impl<P: Platform> System<P> {
    pub fn s_instruction_adc_direct_data(&mut self, target: &mut u8) {
        let op = self.s_cpu_operand();
        *target = self.cpu.r.add(*target, op, self.cpu.r.get_cf());
    }

    pub fn instruction_adc_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.add(*target, source, self.cpu.r.get_cf());
    }

    pub fn s_instruction_adc_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.s_cpu_read(source);
        *target = self.cpu.r.add(*target, s, self.cpu.r.get_cf());
    }

    pub fn s_instruction_add_direct_data(&mut self, target: &mut u8) {
        let op = self.s_cpu_operand();
        *target = self.cpu.r.add(*target, op, false);
    }

    pub fn instruction_add_direct_direct_8(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.add(*target, source, false);
    }

    // synchronized
    pub fn s_instruction_add_direct_direct_16(&mut self, target: &mut u16, source: u16) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }

        let x = u32::from(*target) + u32::from(source);
        let y = u32::from(target.mod_power_of_2(12)) + u32::from(source.mod_power_of_2(12));
        *target = u16::wrapping_from(x);
        self.cpu.r.set_cf(x > 0xffff);
        self.cpu.r.set_hf(y > 0x0fff);
        self.cpu.r.set_nf(false);
    }

    pub fn s_instruction_add_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.s_cpu_read(source);
        *target = self.cpu.r.add(*target, s, false);
    }

    //TODO fix sync!

    // synchronized
    pub fn s_instruction_add_direct_relative(&mut self, target: &mut u16) {
        match self.cpu_instruction_add_direct_relative_sync_point {
            0 => self.s_instruction_add_direct_relative_fresh(target),
            1 => self.s_instruction_add_direct_relative_fresh_resume_at_1(target),
            2 => self.s_instruction_add_direct_relative_fresh_resume_at_2(target),
            _ => panic!(),
        }
    }

    fn s_instruction_add_direct_relative_fresh(&mut self, target: &mut u16) {
        self.cpu_instruction_add_direct_relative_fresh_data = self.s_cpu_operand();

        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_add_direct_relative_sync_point = 1;
            return;
        }

        // ** S2
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_add_direct_relative_sync_point = 2;
            return;
        }

        let data = self.cpu_instruction_add_direct_relative_fresh_data;
        self.cpu
            .r
            .set_cf(target.mod_power_of_2(8) + u16::from(data) > 0xff);
        self.cpu
            .r
            .set_hf(target.mod_power_of_2(4) + u16::from(data.mod_power_of_2(4)) > 0x0f);
        self.cpu.r.set_nf(false);
        self.cpu.r.set_zf(false);
        let data = i8::wrapping_from(data);
        let abs_data = u16::wrapping_from(data.unsigned_abs());
        if data >= 0 {
            target.wrapping_add_assign(abs_data);
        } else {
            target.wrapping_sub_assign(abs_data);
        }
        self.cpu_instruction_add_direct_relative_sync_point = 0;
    }

    fn s_instruction_add_direct_relative_fresh_resume_at_1(&mut self, target: &mut u16) {
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_add_direct_relative_sync_point = 1;
            return;
        }

        // ** S2
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_add_direct_relative_sync_point = 2;
            return;
        }

        let data = self.cpu_instruction_add_direct_relative_fresh_data;
        self.cpu
            .r
            .set_cf(target.mod_power_of_2(8) + u16::from(data) > 0xff);
        self.cpu
            .r
            .set_hf(target.mod_power_of_2(4) + u16::from(data.mod_power_of_2(4)) > 0x0f);
        self.cpu.r.set_nf(false);
        self.cpu.r.set_zf(false);
        let data = i8::wrapping_from(data);
        let abs_data = u16::wrapping_from(data.unsigned_abs());
        if data >= 0 {
            target.wrapping_add_assign(abs_data);
        } else {
            target.wrapping_sub_assign(abs_data);
        }
        self.cpu_instruction_add_direct_relative_sync_point = 0;
    }

    fn s_instruction_add_direct_relative_fresh_resume_at_2(&mut self, target: &mut u16) {
        // ** S2
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_add_direct_relative_sync_point = 2;
            return;
        }

        let data = self.cpu_instruction_add_direct_relative_fresh_data;
        self.cpu
            .r
            .set_cf(target.mod_power_of_2(8) + u16::from(data) > 0xff);
        self.cpu
            .r
            .set_hf(target.mod_power_of_2(4) + u16::from(data.mod_power_of_2(4)) > 0x0f);
        self.cpu.r.set_nf(false);
        self.cpu.r.set_zf(false);
        let data = i8::wrapping_from(data);
        let abs_data = u16::wrapping_from(data.unsigned_abs());
        if data >= 0 {
            target.wrapping_add_assign(abs_data);
        } else {
            target.wrapping_sub_assign(abs_data);
        }
        self.cpu_instruction_add_direct_relative_sync_point = 0;
    }

    pub fn s_instruction_and_direct_data(&mut self, target: &mut u8) {
        let op = self.s_cpu_operand();
        *target = self.cpu.r.and(*target, op);
    }

    pub fn instruction_and_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.and(*target, source);
    }

    pub fn s_instruction_and_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.s_cpu_read(source);
        *target = self.cpu.r.and(*target, s);
    }

    pub fn instruction_bit_index_direct(&mut self, index: U3, data: u8) {
        self.cpu.r.bit(index, data);
    }

    pub fn s_instruction_bit_index_indirect(&mut self, index: U3, address: u16) {
        let data = self.s_cpu_read(address);
        self.cpu.r.bit(index, data);
    }

    // synchronized
    pub fn s_instruction_call_condition_address(&mut self, take: bool) {
        match self.cpu_instruction_call_condition_address_sync_point {
            0 => self.s_instruction_call_condition_address_fresh(take),
            1 => self.s_instruction_call_condition_address_fresh_resume_at_1(),
            _ => panic!(),
        }
    }

    fn s_instruction_call_condition_address_fresh(&mut self, take: bool) {
        self.cpu_instruction_call_condition_address_address = self.s_cpu_operands();
        if !take {
            return;
        }

        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_call_condition_address_sync_point = 1;
            return;
        }

        self.s_cpu_push(self.cpu.r.get_pc());
        self.cpu
            .r
            .set_pc(self.cpu_instruction_call_condition_address_address);
        self.cpu_instruction_call_condition_address_sync_point = 0;
    }

    fn s_instruction_call_condition_address_fresh_resume_at_1(&mut self) {
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_call_condition_address_sync_point = 1;
            return;
        }

        self.s_cpu_push(self.cpu.r.get_pc());
        self.cpu
            .r
            .set_pc(self.cpu_instruction_call_condition_address_address);
        self.cpu_instruction_call_condition_address_sync_point = 0;
    }

    pub fn instruction_ccf(&mut self) {
        self.cpu.r.set_cf(self.cpu.r.get_cf());
        self.cpu.r.set_hf(false);
        self.cpu.r.set_nf(false);
    }

    pub fn s_instruction_cp_direct_data(&mut self, target: u8) {
        let op = self.s_cpu_operand();
        self.cpu.r.cp(target, op);
    }

    pub fn instruction_cp_direct_direct(&mut self, target: u8, source: u8) {
        self.cpu.r.cp(target, source);
    }

    pub fn s_instruction_cp_direct_indirect(&mut self, target: u8, source: u16) {
        let s = self.s_cpu_read(source);
        self.cpu.r.cp(target, s);
    }

    pub fn instruction_cpl(&mut self) {
        self.cpu.r.set_a(!self.cpu.r.get_a());
        self.cpu.r.set_hf(false);
        self.cpu.r.set_nf(false);
    }

    pub fn instruction_daa(&mut self) {
        let orig_a = self.cpu.r.get_a();
        let mut a = u16::from(orig_a);
        if !self.cpu.r.get_nf() {
            if self.cpu.r.get_hf() || orig_a.mod_power_of_2(4) > 0x09 {
                a += 0x06;
            }
            if self.cpu.r.get_cf() || orig_a > 0x99 {
                a += 0x60;
                self.cpu.r.set_cf(true);
            }
        } else {
            if self.cpu.r.get_hf() {
                a.wrapping_sub_assign(0x06);
            }
            if self.cpu.r.get_cf() {
                a.wrapping_sub_assign(0x60);
            }
        }
        let a = u8::wrapping_from(a);
        self.cpu.r.set_a(a);
        self.cpu.r.set_hf(false);
        self.cpu.r.set_zf(a == 0);
    }

    pub fn instruction_dec_direct_8(&mut self, data: &mut u8) {
        *data = self.cpu.r.dec(*data);
    }

    // synchronized
    pub fn s_instruction_dec_direct_16(&mut self, data: &mut u16) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }
        *data -= 1;
    }

    pub fn s_instruction_dec_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.dec(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_di(&mut self) {
        self.cpu.r.ime = false;
    }

    pub fn instruction_ei(&mut self) {
        self.cpu.r.ei = true;
    }

    pub fn s_instruction_halt(&mut self) {
        self.cpu.r.halt = true;
        self.cpu_halt_bug_trigger();
        while self.cpu.r.halt {
            self.s_cpu_halt();
        }
    }

    pub fn instruction_inc_direct_8(&mut self, data: &mut u8) {
        *data = self.cpu.r.inc(*data);
    }

    // synchronized
    pub fn s_instruction_inc_direct_16(&mut self, data: &mut u16) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }
        *data += 1;
    }

    pub fn s_instruction_inc_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.inc(data);
        self.s_cpu_write(address, d);
    }

    // synchronized
    pub fn s_instruction_jp_condition_address(&mut self, take: bool) {
        match self.cpu_instruction_jp_condition_address_sync_point {
            0 => self.s_instruction_jp_condition_address_fresh(take),
            1 => self.s_instruction_jp_condition_address_resume_at_1(),
            _ => panic!(),
        }
    }

    fn s_instruction_jp_condition_address_fresh(&mut self, take: bool) {
        self.cpu_instruction_jp_condition_address_address = self.s_cpu_operands();
        if !take {
            return;
        }
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_jp_condition_address_sync_point = 1;
            return;
        }

        self.cpu
            .r
            .set_pc(self.cpu_instruction_jp_condition_address_address);
        self.cpu_instruction_jp_condition_address_sync_point = 0;
    }

    fn s_instruction_jp_condition_address_resume_at_1(&mut self) {
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_jp_condition_address_sync_point = 1;
            return;
        }

        self.cpu
            .r
            .set_pc(self.cpu_instruction_jp_condition_address_address);
        self.cpu_instruction_jp_condition_address_sync_point = 0;
    }

    pub fn instruction_jp_direct(&mut self, data: u16) {
        self.cpu.r.set_pc(data);
    }

    // synchronized
    pub fn s_instruction_jr_condition_relative(&mut self, take: bool) {
        match self.cpu_instruction_jr_condition_relative_sync_point {
            0 => self.s_instruction_jr_condition_relative_fresh(take),
            1 => self.s_instruction_jr_condition_relative_resume_at_1(),
            _ => panic!(),
        }
    }

    fn s_instruction_jr_condition_relative_fresh(&mut self, take: bool) {
        self.cpu_instruction_jr_condition_relative_data = self.s_cpu_operand();
        if !take {
            return;
        }

        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_jr_condition_relative_sync_point = 1;
            return;
        }

        let mut pc = self.cpu.r.get_pc();
        let data = i8::wrapping_from(self.cpu_instruction_jr_condition_relative_data);
        let abs_data = u16::wrapping_from(data.unsigned_abs());
        if data >= 0 {
            pc.wrapping_add_assign(abs_data);
        } else {
            pc.wrapping_sub_assign(abs_data);
        }
        self.cpu.r.set_pc(pc);
        self.cpu_instruction_jr_condition_relative_sync_point = 0;
    }

    fn s_instruction_jr_condition_relative_resume_at_1(&mut self) {
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_jr_condition_relative_sync_point = 1;
            return;
        }

        let mut pc = self.cpu.r.get_pc();
        let data = i8::wrapping_from(self.cpu_instruction_jr_condition_relative_data);
        let abs_data = u16::wrapping_from(data.unsigned_abs());
        if data >= 0 {
            pc.wrapping_add_assign(abs_data);
        } else {
            pc.wrapping_sub_assign(abs_data);
        }
        self.cpu.r.set_pc(pc);
        self.cpu_instruction_jr_condition_relative_sync_point = 0;
    }

    pub fn s_instruction_ld_address_direct_8(&mut self, data: u8) {
        let op = self.s_cpu_operands();
        self.s_cpu_write(op, data);
    }

    pub fn s_instruction_ld_address_direct_16(&mut self, data: u16) {
        let op = self.s_cpu_operands();
        self.s_cpu_store(op, data);
    }

    pub fn s_instruction_ld_direct_address(&mut self, data: &mut u8) {
        let op = self.s_cpu_operands();
        *data = self.s_cpu_read(op);
    }

    pub fn s_instruction_ld_direct_data_8(&mut self, target: &mut u8) {
        *target = self.s_cpu_operand();
    }

    pub fn s_instruction_ld_direct_data_16(&mut self, target: &mut u16) {
        *target = self.s_cpu_operands();
    }

    pub fn instruction_ld_direct_direct_8(target: &mut u8, source: u8) {
        *target = source;
    }

    // synchronized
    pub fn s_instruction_ld_direct_direct_16(&mut self, target: &mut u16, source: u16) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }
        *target = source;
    }

    // synchronized
    pub fn s_instruction_ld_direct_direct_relative(&mut self, target: &mut u16, source: u16) {
        match self.cpu_instruction_ld_direct_direct_relative_sync_point {
            0 => self.s_instruction_ld_direct_direct_relative_fresh(target, source),
            1 => self.s_instruction_ld_direct_direct_relative_resume_at_1(target, source),
            _ => panic!(),
        }
    }

    fn s_instruction_ld_direct_direct_relative_fresh(&mut self, target: &mut u16, source: u16) {
        self.cpu_instruction_ld_direct_direct_relative_data = self.s_cpu_operand();

        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_ld_direct_direct_relative_sync_point = 1;
            return;
        }

        self.cpu.r.set_cf(
            source.mod_power_of_2(8)
                + u16::from(self.cpu_instruction_ld_direct_direct_relative_data)
                > 0xff,
        );
        self.cpu.r.set_hf(
            source.mod_power_of_2(4)
                + u16::from(
                    self.cpu_instruction_ld_direct_direct_relative_data
                        .mod_power_of_2(4),
                )
                > 0x0f,
        );
        self.cpu.r.set_nf(false);
        self.cpu.r.set_zf(false);
        let data = i8::wrapping_from(self.cpu_instruction_ld_direct_direct_relative_data);
        let abs_data = u16::from(data.unsigned_abs());
        *target = if data >= 0 {
            source.wrapping_add(abs_data)
        } else {
            source.wrapping_sub(abs_data)
        };
        self.cpu_instruction_ld_direct_direct_relative_sync_point = 0;
    }

    fn s_instruction_ld_direct_direct_relative_resume_at_1(
        &mut self,
        target: &mut u16,
        source: u16,
    ) {
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_ld_direct_direct_relative_sync_point = 1;
            return;
        }

        self.cpu.r.set_cf(
            source.mod_power_of_2(8)
                + u16::from(self.cpu_instruction_ld_direct_direct_relative_data)
                > 0xff,
        );
        self.cpu.r.set_hf(
            source.mod_power_of_2(4)
                + u16::from(
                    self.cpu_instruction_ld_direct_direct_relative_data
                        .mod_power_of_2(4),
                )
                > 0x0f,
        );
        self.cpu.r.set_nf(false);
        self.cpu.r.set_zf(false);
        let data = i8::wrapping_from(self.cpu_instruction_ld_direct_direct_relative_data);
        let abs_data = u16::from(data.unsigned_abs());
        *target = if data >= 0 {
            source.wrapping_add(abs_data)
        } else {
            source.wrapping_sub(abs_data)
        };
        self.cpu_instruction_ld_direct_direct_relative_sync_point = 0;
    }

    pub fn s_instruction_ld_direct_indirect(&mut self, target: &mut u8, source: u16) {
        *target = self.s_cpu_read(source);
    }

    pub fn s_instruction_ld_direct_indirect_decrement(
        &mut self,
        target: &mut u8,
        source: &mut u16,
    ) {
        *target = self.s_cpu_read(*source);
        source.wrapping_sub_assign(1);
    }

    pub fn s_instruction_ld_direct_indirect_increment(
        &mut self,
        target: &mut u8,
        source: &mut u16,
    ) {
        *target = self.s_cpu_read(*source);
        source.wrapping_add_assign(1);
    }

    pub fn s_instruction_ld_indirect_data(&mut self, target: u16) {
        let op = self.s_cpu_operand();
        self.s_cpu_write(target, op);
    }

    pub fn s_instruction_ld_indirect_direct(&mut self, target: u16, source: u8) {
        self.s_cpu_write(target, source);
    }

    pub fn s_instruction_ld_indirect_decrement_direct(&mut self, target: &mut u16, source: u8) {
        self.s_cpu_write(*target, source);
        target.wrapping_sub_assign(1);
    }

    pub fn s_instruction_ld_indirect_increment_direct(&mut self, target: &mut u16, source: u8) {
        self.s_cpu_write(*target, source);
        target.wrapping_add_assign(1);
    }

    pub fn s_instruction_ldh_address_direct(&mut self, data: u8) {
        let op = self.s_cpu_operand();
        self.s_cpu_write(0xff00 | u16::from(op), data);
    }

    pub fn s_instruction_ldh_direct_address(&mut self, data: &mut u8) {
        let op = self.s_cpu_operand();
        *data = self.s_cpu_read(0xff00 | u16::from(op));
    }

    pub fn s_instruction_ldh_direct_indirect(&mut self, target: &mut u8, source: u8) {
        *target = self.s_cpu_read(0xff00 | u16::from(source));
    }

    pub fn s_instruction_ldh_indirect_direct(&mut self, target: u8, source: u8) {
        self.s_cpu_write(0xff00 | u16::from(target), source);
    }

    pub fn instruction_nop() {}

    pub fn s_instruction_or_direct_data(&mut self, target: &mut u8) {
        let op = self.s_cpu_operand();
        *target = self.cpu.r.or(*target, op);
    }

    pub fn instruction_or_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.or(*target, source);
    }

    pub fn s_instruction_or_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.s_cpu_read(source);
        *target = self.cpu.r.or(*target, s);
    }

    pub fn s_instruction_pop_direct(&mut self, data: &mut u16) {
        *data = self.s_cpu_pop();
    }

    pub fn s_instruction_pop_direct_af(&mut self, data: &mut u16) {
        *data = self.s_cpu_pop() & !15; // flag bits 0-3 are forced to zero
    }

    // synchronized
    pub fn s_instruction_push_direct(&mut self, data: u16) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }
        self.s_cpu_push(data);
    }

    pub fn instruction_res_index_direct(index: U3, data: &mut u8) {
        data.clear_bit(u64::from(index));
    }

    pub fn s_instruction_res_index_indirect(&mut self, index: U3, address: u16) {
        let mut data = self.s_cpu_read(address);
        data.clear_bit(u64::from(index));
        self.s_cpu_write(address, data);
    }

    // synchronized
    pub fn instruction_ret(&mut self) {
        match self.cpu_instruction_ret_sync_point {
            0 => self.instruction_ret_fresh(),
            1 => self.s_instruction_ret_resume_at_1(),
            _ => panic!(),
        }
    }

    fn instruction_ret_fresh(&mut self) {
        self.cpu_instruction_ret_address = self.s_cpu_pop();

        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_ret_sync_point = 1;
            return;
        }

        self.cpu.r.set_pc(self.cpu_instruction_ret_address);
        self.cpu_instruction_ret_sync_point = 0;
    }

    fn s_instruction_ret_resume_at_1(&mut self) {
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_ret_sync_point = 1;
            return;
        }

        self.cpu.r.set_pc(self.cpu_instruction_ret_address);
        self.cpu_instruction_ret_sync_point = 0;
    }

    // synchronized
    pub fn s_instruction_ret_condition(&mut self, take: bool) {
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            return;
        }
        if !take {
            return;
        }
        let p = self.s_cpu_pop();
        self.cpu.r.set_pc(p);
        self.s_cpu_idle();
    }

    // synchronized
    pub fn s_instruction_reti(&mut self) {
        match self.cpu_instruction_reti_sync_point {
            0 => self.s_instruction_reti_fresh(),
            1 => self.s_instruction_reti_resume_at_1(),
            _ => panic!(),
        }
    }

    fn s_instruction_reti_fresh(&mut self) {
        self.cpu_instruction_reti_address = self.s_cpu_pop();

        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_reti_sync_point = 1;
            return;
        }

        self.cpu.r.set_pc(self.cpu_instruction_reti_address);
        self.cpu.r.ime = true;
        self.cpu_instruction_reti_sync_point = 0;
    }

    fn s_instruction_reti_resume_at_1(&mut self) {
        // ** S1
        self.s_cpu_idle();
        if self.cpu_return_to_sync {
            self.cpu_instruction_reti_sync_point = 1;
            return;
        }

        self.cpu.r.set_pc(self.cpu_instruction_reti_address);
        self.cpu.r.ime = true;
        self.cpu_instruction_reti_sync_point = 0;
    }

    pub fn instruction_rl_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rl(*data);
    }

    pub fn s_instruction_rl_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.rl(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_rla(&mut self) {
        let a = self.cpu.r.rl(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn instruction_rlc_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rlc(*data);
    }

    pub fn s_instruction_rlc_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.rlc(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_rlca(&mut self) {
        let a = self.cpu.r.rlc(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn instruction_rr_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rr(*data);
    }

    pub fn s_instruction_rr_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.rr(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_rra(&mut self) {
        let a = self.cpu.r.rr(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn instruction_rrc_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rrc(*data);
    }

    pub fn s_instruction_rrc_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.rrc(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_rrca(&mut self) {
        let a = self.cpu.r.rrc(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn s_instruction_rst_implied(&mut self, vector: u8) {
        self.s_cpu_idle();
        self.s_cpu_push(self.cpu.r.get_pc());
        self.cpu.r.set_pc(u16::from(vector));
    }

    pub fn s_instruction_sbc_direct_data(&mut self, target: &mut u8) {
        let op = self.s_cpu_operand();
        *target = self.cpu.r.sub(*target, op, self.cpu.r.get_cf());
    }

    pub fn instruction_sbc_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.sub(*target, source, self.cpu.r.get_cf());
    }

    pub fn s_instruction_sbc_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.s_cpu_read(source);
        *target = self.cpu.r.sub(*target, s, self.cpu.r.get_cf());
    }

    pub fn instruction_scf(&mut self) {
        self.cpu.r.set_cf(true);
        self.cpu.r.set_hf(false);
        self.cpu.r.set_nf(false);
    }

    pub fn instruction_set_index_direct(index: U3, data: &mut u8) {
        data.set_bit(u64::from(index))
    }

    pub fn s_instruction_set_index_indirect(&mut self, index: U3, address: u16) {
        let mut data = self.s_cpu_read(address);
        data.set_bit(u64::from(index));
        self.s_cpu_write(address, data);
    }

    pub fn instruction_sla_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.sla(*data);
    }

    pub fn s_instruction_sla_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.sla(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_sra_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.sra(*data);
    }

    pub fn s_instruction_sra_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.sra(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_srl_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.srl(*data);
    }

    pub fn s_instruction_srl_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.srl(data);
        self.s_cpu_write(address, d);
    }

    pub fn instruction_stop(&mut self) {
        if !self.cpu.stoppable() {
            return;
        }
        self.cpu.r.stop = true;
        while self.cpu.r.stop {
            self.s_cpu_stop();
        }
    }

    pub fn s_instruction_sub_direct_data(&mut self, target: &mut u8) {
        let op = self.s_cpu_operand();
        *target = self.cpu.r.sub(*target, op, false);
    }

    pub fn instruction_sub_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.sub(*target, source, false);
    }

    pub fn s_instruction_sub_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.s_cpu_read(source);
        *target = self.cpu.r.sub(*target, s, false);
    }

    pub fn instruction_swap_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.swap(*data);
    }

    pub fn s_instruction_swap_indirect(&mut self, address: u16) {
        let data = self.s_cpu_read(address);
        let d = self.cpu.r.swap(data);
        self.s_cpu_write(address, d);
    }

    pub fn s_instruction_xor_direct_data(&mut self, target: &mut u8) {
        let op = self.s_cpu_operand();
        *target = self.cpu.r.xor(*target, op);
    }

    pub fn instruction_xor_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.xor(*target, source);
    }

    pub fn s_instruction_xor_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.s_cpu_read(source);
        *target = self.cpu.r.xor(*target, s);
    }
}
