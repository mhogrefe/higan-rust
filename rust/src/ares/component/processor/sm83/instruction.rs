use ares::gb::bus::Bus;

impl Bus {
    pub fn instruction(&mut self) {
        match self.cpu_operand() {
            0x00 => Bus::instruction_nop(),
            0x01 => {
                let mut bc = self.cpu.r.get_bc();
                self.instruction_ld_direct_data_16(&mut bc);
                self.cpu.r.set_bc(bc);
            }
            0x02 => self.instruction_ld_indirect_direct(self.cpu.r.get_bc(), self.cpu.r.get_a()),
            0x03 => {
                let mut bc = self.cpu.r.get_bc();
                self.instruction_inc_direct_16(&mut bc);
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
                self.instruction_ld_direct_data_8(&mut b);
                self.cpu.r.set_b(b);
            }
            0x07 => self.instruction_rlca(),
            0x08 => self.instruction_ld_address_direct_16(self.cpu.r.get_sp()),
            0x09 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_add_direct_direct_16(&mut hl, self.cpu.r.get_bc());
                self.cpu.r.set_hl(hl);
            }
            0x0a => {
                let mut a = self.cpu.r.get_a();
                self.instruction_ld_direct_indirect(&mut a, self.cpu.r.get_bc());
                self.cpu.r.set_a(a);
            }
            0x0b => {
                let mut bc = self.cpu.r.get_bc();
                self.instruction_dec_direct_16(&mut bc);
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
                self.instruction_ld_direct_data_8(&mut c);
                self.cpu.r.set_c(c);
            }
            0x0f => self.instruction_rrca(),
            0x10 => self.instruction_stop(),
            0x11 => {
                let mut de = self.cpu.r.get_de();
                self.instruction_ld_direct_data_16(&mut de);
                self.cpu.r.set_de(de);
            }
            0x12 => self.instruction_ld_indirect_direct(self.cpu.r.get_de(), self.cpu.r.get_a()),
            0x13 => {
                let mut de = self.cpu.r.get_de();
                self.instruction_inc_direct_16(&mut de);
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
                self.instruction_ld_direct_data_8(&mut d);
                self.cpu.r.set_d(d);
            }
            0x17 => self.instruction_rla(),
            0x18 => self.instruction_jr_condition_relative(true),
            0x19 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_add_direct_direct_16(&mut hl, self.cpu.r.get_de());
                self.cpu.r.set_hl(hl);
            }
            0x1a => {
                let mut a = self.cpu.r.get_a();
                self.instruction_ld_direct_indirect(&mut a, self.cpu.r.get_de());
                self.cpu.r.set_a(a);
            }
            0x1b => {
                let mut de = self.cpu.r.get_de();
                self.instruction_dec_direct_16(&mut de);
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
                self.instruction_ld_direct_data_8(&mut e);
                self.cpu.r.set_e(e);
            }
            0x1f => self.instruction_rra(),
            0x20 => self.instruction_jr_condition_relative(!self.cpu.r.get_zf()),
            0x21 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_ld_direct_data_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x22 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_ld_indirect_increment_direct(&mut hl, self.cpu.r.get_a());
                self.cpu.r.set_hl(hl);
            }
            0x23 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_inc_direct_16(&mut hl);
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
                self.instruction_ld_direct_data_8(&mut h);
                self.cpu.r.set_h(h);
            }
            0x27 => self.instruction_daa(),
            0x28 => self.instruction_jr_condition_relative(self.cpu.r.get_zf()),
            0x29 => {
                let mut hl = self.cpu.r.get_hl();
                let hl_copy = hl;
                self.instruction_add_direct_direct_16(&mut hl, hl_copy);
                self.cpu.r.set_hl(hl);
            }
            0x2a => {
                let mut a = self.cpu.r.get_a();
                let mut hl = self.cpu.r.get_hl();
                self.instruction_ld_direct_indirect_increment(&mut a, &mut hl);
                self.cpu.r.set_a(a);
                self.cpu.r.set_hl(hl);
            }
            0x2b => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_dec_direct_16(&mut hl);
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
                self.instruction_ld_direct_data_8(&mut l);
                self.cpu.r.set_l(l);
            }
            0x2f => self.instruction_cpl(),
            0x30 => self.instruction_jr_condition_relative(!self.cpu.r.get_cf()),
            0x31 => {
                let mut sp = self.cpu.r.get_sp();
                self.instruction_ld_direct_data_16(&mut sp);
                self.cpu.r.set_sp(sp);
            }
            0x32 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_ld_indirect_decrement_direct(&mut hl, self.cpu.r.get_a());
                self.cpu.r.set_hl(hl);
            }
            0x33 => {
                let mut sp = self.cpu.r.get_sp();
                self.instruction_inc_direct_16(&mut sp);
                self.cpu.r.set_sp(sp);
            }
            0x34 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_inc_direct_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x35 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_dec_direct_16(&mut hl);
                self.cpu.r.set_hl(hl);
            }
            0x36 => self.instruction_ld_indirect_data(self.cpu.r.get_hl()),
            0x37 => self.instruction_scf(),
            0x38 => self.instruction_jr_condition_relative(self.cpu.r.get_cf()),
            0x39 => {
                let mut hl = self.cpu.r.get_hl();
                self.instruction_add_direct_direct_16(&mut hl, self.cpu.r.get_sp());
                self.cpu.r.set_hl(hl);
            }
            0x3a => {
                let mut a = self.cpu.r.get_a();
                let mut hl = self.cpu.r.get_hl();
                self.instruction_ld_direct_indirect_decrement(&mut a, &mut hl);
                self.cpu.r.set_a(a);
                self.cpu.r.set_hl(hl);
            }
            0x3b => {
                let mut sp = self.cpu.r.get_sp();
                self.instruction_dec_direct_16(&mut sp);
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
                self.instruction_ld_direct_data_8(&mut a);
                self.cpu.r.set_a(a);
            }
            0x3f => self.instruction_ccf(),
            0x40 => {
                let mut b = self.cpu.r.get_b();
                let b_copy = b;
                Bus::instruction_ld_direct_direct_8(&mut b, b_copy);
                self.cpu.r.set_b(b);
            }
            0x41 => {
                let mut b = self.cpu.r.get_b();
                Bus::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_c());
                self.cpu.r.set_b(b);
            }
            0x42 => {
                let mut b = self.cpu.r.get_b();
                Bus::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_d());
                self.cpu.r.set_b(b);
            }
            0x43 => {
                let mut b = self.cpu.r.get_b();
                Bus::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_e());
                self.cpu.r.set_b(b);
            }
            0x44 => {
                let mut b = self.cpu.r.get_b();
                Bus::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_h());
                self.cpu.r.set_b(b);
            }
            0x45 => {
                let mut b = self.cpu.r.get_b();
                Bus::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_l());
                self.cpu.r.set_b(b);
            }
            0x46 => {
                let mut b = self.cpu.r.get_b();
                self.instruction_ld_direct_indirect(&mut b, self.cpu.r.get_hl());
                self.cpu.r.set_b(b);
            }
            0x47 => {
                let mut b = self.cpu.r.get_b();
                Bus::instruction_ld_direct_direct_8(&mut b, self.cpu.r.get_a());
                self.cpu.r.set_b(b);
            }
            0x48 => {
                let mut c = self.cpu.r.get_c();
                Bus::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_b());
                self.cpu.r.set_c(c);
            }
            0x49 => {
                let mut c = self.cpu.r.get_c();
                let c_copy = c;
                Bus::instruction_ld_direct_direct_8(&mut c, c_copy);
                self.cpu.r.set_c(c);
            }
            0x4a => {
                let mut c = self.cpu.r.get_c();
                Bus::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_d());
                self.cpu.r.set_c(c);
            }
            0x4b => {
                let mut c = self.cpu.r.get_c();
                Bus::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_e());
                self.cpu.r.set_c(c);
            }
            0x4c => {
                let mut c = self.cpu.r.get_c();
                Bus::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_h());
                self.cpu.r.set_c(c);
            }
            0x4d => {
                let mut c = self.cpu.r.get_c();
                Bus::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_l());
                self.cpu.r.set_c(c);
            }
            0x4e => {
                let mut c = self.cpu.r.get_c();
                self.instruction_ld_direct_indirect(&mut c, self.cpu.r.get_hl());
                self.cpu.r.set_c(c);
            }
            0x4f => {
                let mut c = self.cpu.r.get_c();
                Bus::instruction_ld_direct_direct_8(&mut c, self.cpu.r.get_a());
                self.cpu.r.set_c(c);
            }
            0x50 => {
                let mut d = self.cpu.r.get_d();
                Bus::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_b());
                self.cpu.r.set_d(d);
            }
            0x51 => {
                let mut d = self.cpu.r.get_d();
                Bus::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_c());
                self.cpu.r.set_d(d);
            }
            0x52 => {
                let mut d = self.cpu.r.get_d();
                let d_copy = d;
                Bus::instruction_ld_direct_direct_8(&mut d, d_copy);
                self.cpu.r.set_d(d);
            }
            0x53 => {
                let mut d = self.cpu.r.get_d();
                Bus::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_e());
                self.cpu.r.set_d(d);
            }
            0x54 => {
                let mut d = self.cpu.r.get_d();
                Bus::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_h());
                self.cpu.r.set_d(d);
            }
            0x55 => {
                let mut d = self.cpu.r.get_d();
                Bus::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_l());
                self.cpu.r.set_d(d);
            }
            0x56 => {
                let mut d = self.cpu.r.get_d();
                self.instruction_ld_direct_indirect(&mut d, self.cpu.r.get_hl());
                self.cpu.r.set_d(d);
            }
            0x57 => {
                let mut d = self.cpu.r.get_d();
                Bus::instruction_ld_direct_direct_8(&mut d, self.cpu.r.get_a());
                self.cpu.r.set_d(d);
            }
            0x58 => {
                let mut e = self.cpu.r.get_e();
                Bus::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_b());
                self.cpu.r.set_e(e);
            }
            0x59 => {
                let mut e = self.cpu.r.get_e();
                Bus::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_c());
                self.cpu.r.set_e(e);
            }
            0x5a => {
                let mut e = self.cpu.r.get_e();
                Bus::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_d());
                self.cpu.r.set_e(e);
            }
            0x5b => {
                let mut e = self.cpu.r.get_e();
                let e_copy = e;
                Bus::instruction_ld_direct_direct_8(&mut e, e_copy);
                self.cpu.r.set_e(e);
            }
            0x5c => {
                let mut e = self.cpu.r.get_e();
                Bus::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_h());
                self.cpu.r.set_e(e);
            }
            0x5d => {
                let mut e = self.cpu.r.get_e();
                Bus::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_l());
                self.cpu.r.set_e(e);
            }
            0x5e => {
                let mut e = self.cpu.r.get_e();
                self.instruction_ld_direct_indirect(&mut e, self.cpu.r.get_hl());
                self.cpu.r.set_e(e);
            }
            0x5f => {
                let mut e = self.cpu.r.get_e();
                Bus::instruction_ld_direct_direct_8(&mut e, self.cpu.r.get_a());
                self.cpu.r.set_e(e);
            }
            0x60 => {
                let mut h = self.cpu.r.get_h();
                Bus::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_b());
                self.cpu.r.set_h(h);
            }
            0x61 => {
                let mut h = self.cpu.r.get_h();
                Bus::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_c());
                self.cpu.r.set_h(h);
            }
            0x62 => {
                let mut h = self.cpu.r.get_h();
                Bus::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_d());
                self.cpu.r.set_h(h);
            }
            0x63 => {
                let mut h = self.cpu.r.get_h();
                Bus::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_e());
                self.cpu.r.set_h(h);
            }
            0x64 => {
                let mut h = self.cpu.r.get_h();
                let h_copy = h;
                Bus::instruction_ld_direct_direct_8(&mut h, h_copy);
                self.cpu.r.set_h(h);
            }
            0x65 => {
                let mut h = self.cpu.r.get_h();
                Bus::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_l());
                self.cpu.r.set_h(h);
            }
            0x66 => {
                let mut h = self.cpu.r.get_h();
                self.instruction_ld_direct_indirect(&mut h, self.cpu.r.get_hl());
                self.cpu.r.set_h(h);
            }
            0x67 => {
                let mut h = self.cpu.r.get_h();
                Bus::instruction_ld_direct_direct_8(&mut h, self.cpu.r.get_a());
                self.cpu.r.set_h(h);
            }
            0x68 => {
                let mut l = self.cpu.r.get_l();
                Bus::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_b());
                self.cpu.r.set_l(l);
            }
            0x69 => {
                let mut l = self.cpu.r.get_l();
                Bus::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_c());
                self.cpu.r.set_l(l);
            }
            0x6a => {
                let mut l = self.cpu.r.get_l();
                Bus::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_d());
                self.cpu.r.set_l(l);
            }
            0x6b => {
                let mut l = self.cpu.r.get_l();
                Bus::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_e());
                self.cpu.r.set_l(l);
            }
            0x6c => {
                let mut l = self.cpu.r.get_l();
                Bus::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_e());
                self.cpu.r.set_l(l);
            }
            0x6d => {
                let mut l = self.cpu.r.get_l();
                let l_copy = l;
                Bus::instruction_ld_direct_direct_8(&mut l, l_copy);
                self.cpu.r.set_l(l);
            }
            0x6e => {
                let mut l = self.cpu.r.get_l();
                self.instruction_ld_direct_indirect(&mut l, self.cpu.r.get_hl());
                self.cpu.r.set_l(l);
            }
            0x6f => {
                let mut l = self.cpu.r.get_l();
                Bus::instruction_ld_direct_direct_8(&mut l, self.cpu.r.get_a());
                self.cpu.r.set_l(l);
            }
            /*
            op(0x70, LD_Indirect_Direct, HL, B)
            op(0x71, LD_Indirect_Direct, HL, C)
            op(0x72, LD_Indirect_Direct, HL, D)
            op(0x73, LD_Indirect_Direct, HL, E)
            op(0x74, LD_Indirect_Direct, HL, H)
            op(0x75, LD_Indirect_Direct, HL, L)
            op(0x76, HALT)
            op(0x77, LD_Indirect_Direct, HL, A)
            op(0x78, LD_Direct_Direct, A, B)
            op(0x79, LD_Direct_Direct, A, C)
            op(0x7a, LD_Direct_Direct, A, D)
            op(0x7b, LD_Direct_Direct, A, E)
            op(0x7c, LD_Direct_Direct, A, H)
            op(0x7d, LD_Direct_Direct, A, L)
            op(0x7e, LD_Direct_Indirect, A, HL)
            op(0x7f, LD_Direct_Direct, A, A)
            op(0x80, ADD_Direct_Direct, A, B)
            op(0x81, ADD_Direct_Direct, A, C)
            op(0x82, ADD_Direct_Direct, A, D)
            op(0x83, ADD_Direct_Direct, A, E)
            op(0x84, ADD_Direct_Direct, A, H)
            op(0x85, ADD_Direct_Direct, A, L)
            op(0x86, ADD_Direct_Indirect, A, HL)
            op(0x87, ADD_Direct_Direct, A, A)
            op(0x88, ADC_Direct_Direct, A, B)
            op(0x89, ADC_Direct_Direct, A, C)
            op(0x8a, ADC_Direct_Direct, A, D)
            op(0x8b, ADC_Direct_Direct, A, E)
            op(0x8c, ADC_Direct_Direct, A, H)
            op(0x8d, ADC_Direct_Direct, A, L)
            op(0x8e, ADC_Direct_Indirect, A, HL)
            op(0x8f, ADC_Direct_Direct, A, A)
            op(0x90, SUB_Direct_Direct, A, B)
            op(0x91, SUB_Direct_Direct, A, C)
            op(0x92, SUB_Direct_Direct, A, D)
            op(0x93, SUB_Direct_Direct, A, E)
            op(0x94, SUB_Direct_Direct, A, H)
            op(0x95, SUB_Direct_Direct, A, L)
            op(0x96, SUB_Direct_Indirect, A, HL)
            op(0x97, SUB_Direct_Direct, A, A)
            op(0x98, SBC_Direct_Direct, A, B)
            op(0x99, SBC_Direct_Direct, A, C)
            op(0x9a, SBC_Direct_Direct, A, D)
            op(0x9b, SBC_Direct_Direct, A, E)
            op(0x9c, SBC_Direct_Direct, A, H)
            op(0x9d, SBC_Direct_Direct, A, L)
            op(0x9e, SBC_Direct_Indirect, A, HL)
            op(0x9f, SBC_Direct_Direct, A, A)
            op(0xa0, AND_Direct_Direct, A, B)
            op(0xa1, AND_Direct_Direct, A, C)
            op(0xa2, AND_Direct_Direct, A, D)
            op(0xa3, AND_Direct_Direct, A, E)
            op(0xa4, AND_Direct_Direct, A, H)
            op(0xa5, AND_Direct_Direct, A, L)
            op(0xa6, AND_Direct_Indirect, A, HL)
            op(0xa7, AND_Direct_Direct, A, A)
            op(0xa8, XOR_Direct_Direct, A, B)
            op(0xa9, XOR_Direct_Direct, A, C)
            op(0xaa, XOR_Direct_Direct, A, D)
            op(0xab, XOR_Direct_Direct, A, E)
            op(0xac, XOR_Direct_Direct, A, H)
            op(0xad, XOR_Direct_Direct, A, L)
            op(0xae, XOR_Direct_Indirect, A, HL)
            op(0xaf, XOR_Direct_Direct, A, A)
            op(0xb0, OR_Direct_Direct, A, B)
            op(0xb1, OR_Direct_Direct, A, C)
            op(0xb2, OR_Direct_Direct, A, D)
            op(0xb3, OR_Direct_Direct, A, E)
            op(0xb4, OR_Direct_Direct, A, H)
            op(0xb5, OR_Direct_Direct, A, L)
            op(0xb6, OR_Direct_Indirect, A, HL)
            op(0xb7, OR_Direct_Direct, A, A)
            op(0xb8, CP_Direct_Direct, A, B)
            op(0xb9, CP_Direct_Direct, A, C)
            op(0xba, CP_Direct_Direct, A, D)
            op(0xbb, CP_Direct_Direct, A, E)
            op(0xbc, CP_Direct_Direct, A, H)
            op(0xbd, CP_Direct_Direct, A, L)
            op(0xbe, CP_Direct_Indirect, A, HL)
            op(0xbf, CP_Direct_Direct, A, A)
            op(0xc0, RET_Condition, ZF == 0)
            op(0xc1, POP_Direct, BC)
            op(0xc2, JP_Condition_Address, ZF == 0)
            op(0xc3, JP_Condition_Address, 1)
            op(0xc4, CALL_Condition_Address, ZF == 0)
            op(0xc5, PUSH_Direct, BC)
            op(0xc6, ADD_Direct_Data, A)
            op(0xc7, RST_Implied, 0x00)
            op(0xc8, RET_Condition, ZF == 1)
            op(0xc9, RET)
            op(0xca, JP_Condition_Address, ZF == 1)
            op(0xcb, CB)
            op(0xcc, CALL_Condition_Address, ZF == 1)
            op(0xcd, CALL_Condition_Address, 1)
            op(0xce, ADC_Direct_Data, A)
            op(0xcf, RST_Implied, 0x08)
            op(0xd0, RET_Condition, CF == 0)
            op(0xd1, POP_Direct, DE)
            op(0xd2, JP_Condition_Address, CF == 0)
            op(0xd4, CALL_Condition_Address, CF == 0)
            op(0xd5, PUSH_Direct, DE)
            op(0xd6, SUB_Direct_Data, A)
            op(0xd7, RST_Implied, 0x10)
            op(0xd8, RET_Condition, CF == 1)
            op(0xd9, RETI)
            op(0xda, JP_Condition_Address, CF == 1)
            op(0xdc, CALL_Condition_Address, CF == 1)
            op(0xde, SBC_Direct_Data, A)
            op(0xdf, RST_Implied, 0x18)
            op(0xe0, LDH_Address_Direct, A)
            op(0xe1, POP_Direct, HL)
            op(0xe2, LDH_Indirect_Direct, C, A)
            op(0xe5, PUSH_Direct, HL)
            op(0xe6, AND_Direct_Data, A)
            op(0xe7, RST_Implied, 0x20)
            op(0xe8, ADD_Direct_Relative, SP)
            op(0xe9, JP_Direct, HL)
            op(0xea, LD_Address_Direct, A)
            op(0xee, XOR_Direct_Data, A)
            op(0xef, RST_Implied, 0x28)
            op(0xf0, LDH_Direct_Address, A)
            op(0xf1, POP_Direct_AF, AF)
            op(0xf2, LDH_Direct_Indirect, A, C)
            op(0xf3, DI)
            op(0xf5, PUSH_Direct, AF)
            op(0xf6, OR_Direct_Data, A)
            op(0xf7, RST_Implied, 0x30)
            op(0xf8, LD_Direct_DirectRelative, HL, SP)
            op(0xf9, LD_Direct_Direct, SP, HL)
            op(0xfa, LD_Direct_Address, A)
            op(0xfb, EI)
            op(0xfe, CP_Direct_Data, A)
            op(0xff, RST_Implied, 0x38)*/
            _ => {}
        }
    }
}
