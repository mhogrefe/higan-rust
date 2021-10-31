use ares::emulator::types::U3;
use ares::gb::system::System;
use ares::platform::Platform;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, WrappingAddAssign, WrappingSubAssign};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;

// See higan-rust/rust/src/ares/component/processor/sm83/instructions.rs
impl<P: Platform> System<P> {
    pub fn instruction_adc_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.add(*target, op, self.cpu.r.get_cf());
    }

    pub fn instruction_adc_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.add(*target, source, self.cpu.r.get_cf());
    }

    pub fn instruction_adc_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
        *target = self.cpu.r.add(*target, s, self.cpu.r.get_cf());
    }

    pub fn instruction_add_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.add(*target, op, false);
    }

    pub fn instruction_add_direct_direct_8(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.add(*target, source, false);
    }

    pub fn instruction_add_direct_direct_16(&mut self, target: &mut u16, source: u16) {
        self.cpu_idle();
        let x = u32::from(*target) + u32::from(source);
        let y = u32::from(target.mod_power_of_2(12)) + u32::from(source.mod_power_of_2(12));
        *target = u16::wrapping_from(x);
        self.cpu.r.set_cf(x > 0xffff);
        self.cpu.r.set_hf(y > 0x0fff);
        self.cpu.r.set_nf(false);
    }

    pub fn instruction_add_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
        *target = self.cpu.r.add(*target, s, false);
    }

    pub fn instruction_add_direct_relative(&mut self, target: &mut u16) {
        let data = self.cpu_operand();
        self.cpu_idle();
        self.cpu_idle();
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
    }

    pub fn instruction_and_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.and(*target, op);
    }

    pub fn instruction_and_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.and(*target, source);
    }

    pub fn instruction_and_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
        *target = self.cpu.r.and(*target, s);
    }

    pub fn instruction_bit_index_direct(&mut self, index: U3, data: u8) {
        self.cpu.r.bit(index, data);
    }

    pub fn instruction_bit_index_indirect(&mut self, index: U3, address: u16) {
        let data = self.cpu_read(address);
        self.cpu.r.bit(index, data);
    }

    pub fn instruction_call_condition_address(&mut self, take: bool) {
        let address = self.cpu_operands();
        if !take {
            return;
        }
        self.cpu_idle();
        self.cpu_push(self.cpu.r.get_pc());
        self.cpu.r.set_pc(address);
    }

    pub fn instruction_ccf(&mut self) {
        self.cpu.r.set_cf(self.cpu.r.get_cf());
        self.cpu.r.set_hf(false);
        self.cpu.r.set_nf(false);
    }

    pub fn instruction_cp_direct_data(&mut self, target: u8) {
        let op = self.cpu_operand();
        self.cpu.r.cp(target, op);
    }

    pub fn instruction_cp_direct_direct(&mut self, target: u8, source: u8) {
        self.cpu.r.cp(target, source);
    }

    pub fn instruction_cp_direct_indirect(&mut self, target: u8, source: u16) {
        let s = self.cpu_read(source);
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

    pub fn instruction_dec_direct_16(&mut self, data: &mut u16) {
        self.cpu_idle();
        *data -= 1;
    }

    pub fn instruction_dec_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.dec(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_di(&mut self) {
        self.cpu.r.ime = false;
    }

    pub fn instruction_ei(&mut self) {
        self.cpu.r.ei = true;
    }

    pub fn instruction_halt(&mut self) {
        self.cpu.r.halt = true;
        self.cpu_halt_bug_trigger();
        while self.cpu.r.halt {
            self.cpu_halt();
        }
    }

    pub fn instruction_inc_direct_8(&mut self, data: &mut u8) {
        *data = self.cpu.r.inc(*data);
    }

    pub fn instruction_inc_direct_16(&mut self, data: &mut u16) {
        self.cpu_idle();
        *data += 1;
    }

    pub fn instruction_inc_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.inc(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_jp_condition_address(&mut self, take: bool) {
        let address = self.cpu_operands();
        if !take {
            return;
        }
        self.cpu_idle();
        self.cpu.r.set_pc(address);
    }

    pub fn instruction_jp_direct(&mut self, data: u16) {
        self.cpu.r.set_pc(data);
    }

    pub fn instruction_jr_condition_relative(&mut self, take: bool) {
        let data = self.cpu_operand();
        if !take {
            return;
        }
        self.cpu_idle();
        let mut pc = self.cpu.r.get_pc();
        let data = i8::wrapping_from(data);
        let abs_data = u16::wrapping_from(data.unsigned_abs());
        if data >= 0 {
            pc.wrapping_add_assign(abs_data);
        } else {
            pc.wrapping_sub_assign(abs_data);
        }
        self.cpu.r.set_pc(pc);
    }

    pub fn instruction_ld_address_direct_8(&mut self, data: u8) {
        let op = self.cpu_operands();
        self.cpu_write(op, data);
    }

    pub fn instruction_ld_address_direct_16(&mut self, data: u16) {
        let op = self.cpu_operands();
        self.cpu_store(op, data);
    }

    pub fn instruction_ld_direct_address(&mut self, data: &mut u8) {
        let op = self.cpu_operands();
        *data = self.cpu_read(op);
    }

    pub fn instruction_ld_direct_data_8(&mut self, target: &mut u8) {
        *target = self.cpu_operand();
    }

    pub fn instruction_ld_direct_data_16(&mut self, target: &mut u16) {
        *target = self.cpu_operands();
    }

    pub fn instruction_ld_direct_direct_8(target: &mut u8, source: u8) {
        *target = source;
    }

    pub fn instruction_ld_direct_direct_16(&mut self, target: &mut u16, source: u16) {
        self.cpu_idle();
        *target = source;
    }

    pub fn instruction_ld_direct_direct_relative(&mut self, target: &mut u16, source: u16) {
        let data = self.cpu_operand();
        self.cpu_idle();
        self.cpu
            .r
            .set_cf(source.mod_power_of_2(8) + u16::from(data) > 0xff);
        self.cpu
            .r
            .set_hf(source.mod_power_of_2(4) + u16::from(data.mod_power_of_2(4)) > 0x0f);
        self.cpu.r.set_nf(false);
        self.cpu.r.set_zf(false);
        let data = i8::wrapping_from(data);
        let abs_data = u16::from(data.unsigned_abs());
        *target = if data >= 0 {
            source.wrapping_add(abs_data)
        } else {
            source.wrapping_sub(abs_data)
        };
    }

    pub fn instruction_ld_direct_indirect(&mut self, target: &mut u8, source: u16) {
        *target = self.cpu_read(source);
    }

    pub fn instruction_ld_direct_indirect_decrement(&mut self, target: &mut u8, source: &mut u16) {
        *target = self.cpu_read(*source);
        source.wrapping_sub_assign(1);
    }

    pub fn instruction_ld_direct_indirect_increment(&mut self, target: &mut u8, source: &mut u16) {
        *target = self.cpu_read(*source);
        source.wrapping_add_assign(1);
    }

    pub fn instruction_ld_indirect_data(&mut self, target: u16) {
        let op = self.cpu_operand();
        self.cpu_write(target, op);
    }

    pub fn instruction_ld_indirect_direct(&mut self, target: u16, source: u8) {
        self.cpu_write(target, source);
    }

    pub fn instruction_ld_indirect_decrement_direct(&mut self, target: &mut u16, source: u8) {
        self.cpu_write(*target, source);
        target.wrapping_sub_assign(1);
    }

    pub fn instruction_ld_indirect_increment_direct(&mut self, target: &mut u16, source: u8) {
        self.cpu_write(*target, source);
        target.wrapping_add_assign(1);
    }

    pub fn instruction_ldh_address_direct(&mut self, data: u8) {
        let op = self.cpu_operand();
        self.cpu_write(0xff00 | u16::from(op), data);
    }

    pub fn instruction_ldh_direct_address(&mut self, data: &mut u8) {
        let op = self.cpu_operand();
        *data = self.cpu_read(0xff00 | u16::from(op));
    }

    pub fn instruction_ldh_direct_indirect(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu_read(0xff00 | u16::from(source));
    }

    pub fn instruction_ldh_indirect_direct(&mut self, target: u8, source: u8) {
        self.cpu_write(0xff00 | u16::from(target), source);
    }

    pub fn instruction_nop() {}

    pub fn instruction_or_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.or(*target, op);
    }

    pub fn instruction_or_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.or(*target, source);
    }

    pub fn instruction_or_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
        *target = self.cpu.r.or(*target, s);
    }

    pub fn instruction_pop_direct(&mut self, data: &mut u16) {
        *data = self.cpu_pop();
    }

    pub fn instruction_pop_direct_af(&mut self, data: &mut u16) {
        *data = self.cpu_pop() & !15; // flag bits 0-3 are forced to zero
    }

    pub fn instruction_push_direct(&mut self, data: u16) {
        self.cpu_idle();
        self.cpu_push(data);
    }

    pub fn instruction_res_index_direct(index: U3, data: &mut u8) {
        data.clear_bit(u64::from(index));
    }

    pub fn instruction_res_index_indirect(&mut self, index: U3, address: u16) {
        let mut data = self.cpu_read(address);
        data.clear_bit(u64::from(index));
        self.cpu_write(address, data);
    }

    pub fn instruction_ret(&mut self) {
        let address = self.cpu_pop();
        self.cpu_idle();
        self.cpu.r.set_pc(address);
    }

    pub fn instruction_ret_condition(&mut self, take: bool) {
        self.cpu_idle();
        if !take {
            return;
        }
        let p = self.cpu_pop();
        self.cpu.r.set_pc(p);
        self.cpu_idle();
    }

    pub fn instruction_reti(&mut self) {
        let address = self.cpu_pop();
        self.cpu_idle();
        self.cpu.r.set_pc(address);
        self.cpu.r.ime = true;
    }

    pub fn instruction_rl_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rl(*data);
    }

    pub fn instruction_rl_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.rl(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_rla(&mut self) {
        let a = self.cpu.r.rl(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn instruction_rlc_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rlc(*data);
    }

    pub fn instruction_rlc_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.rlc(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_rlca(&mut self) {
        let a = self.cpu.r.rlc(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn instruction_rr_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rr(*data);
    }

    pub fn instruction_rr_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.rr(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_rra(&mut self) {
        let a = self.cpu.r.rr(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn instruction_rrc_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.rrc(*data);
    }

    pub fn instruction_rrc_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.rrc(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_rrca(&mut self) {
        let a = self.cpu.r.rrc(self.cpu.r.get_a());
        self.cpu.r.set_a(a);
        self.cpu.r.set_zf(false);
    }

    pub fn instruction_rst_implied(&mut self, vector: u8) {
        self.cpu_idle();
        self.cpu_push(self.cpu.r.get_pc());
        self.cpu.r.set_pc(u16::from(vector));
    }

    pub fn instruction_sbc_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.sub(*target, op, self.cpu.r.get_cf());
    }

    pub fn instruction_sbc_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.sub(*target, source, self.cpu.r.get_cf());
    }

    pub fn instruction_sbc_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
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

    pub fn instruction_set_index_indirect(&mut self, index: U3, address: u16) {
        let mut data = self.cpu_read(address);
        data.set_bit(u64::from(index));
        self.cpu_write(address, data);
    }

    pub fn instruction_sla_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.sla(*data);
    }

    pub fn instruction_sla_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.sla(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_sra_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.sra(*data);
    }

    pub fn instruction_sra_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.sra(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_srl_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.srl(*data);
    }

    pub fn instruction_srl_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.srl(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_stop(&mut self) {
        if !self.cpu.stoppable() {
            return;
        }
        self.cpu.r.stop = true;
        while self.cpu.r.stop {
            self.cpu_stop();
        }
    }

    pub fn instruction_sub_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.sub(*target, op, false);
    }

    pub fn instruction_sub_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.sub(*target, source, false);
    }

    pub fn instruction_sub_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
        *target = self.cpu.r.sub(*target, s, false);
    }

    pub fn instruction_swap_direct(&mut self, data: &mut u8) {
        *data = self.cpu.r.swap(*data);
    }

    pub fn instruction_swap_indirect(&mut self, address: u16) {
        let data = self.cpu_read(address);
        let d = self.cpu.r.swap(data);
        self.cpu_write(address, d);
    }

    pub fn instruction_xor_direct_data(&mut self, target: &mut u8) {
        let op = self.cpu_operand();
        *target = self.cpu.r.xor(*target, op);
    }

    pub fn instruction_xor_direct_direct(&mut self, target: &mut u8, source: u8) {
        *target = self.cpu.r.xor(*target, source);
    }

    pub fn instruction_xor_direct_indirect(&mut self, target: &mut u8, source: u16) {
        let s = self.cpu_read(source);
        *target = self.cpu.r.xor(*target, s);
    }
}
