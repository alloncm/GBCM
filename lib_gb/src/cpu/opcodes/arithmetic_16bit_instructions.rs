use super::opcodes_utils::{
    check_for_half_carry_third_nible_add, get_arithmetic_16reg,
    signed_check_for_carry_first_nible_add, signed_check_for_half_carry_first_nible_add,
};
use crate::cpu::flag::Flag;
use crate::cpu::gb_cpu::GbCpu;

pub fn add_hl_rr(cpu: &mut GbCpu, opcode: u8) -> u8 {
    let reg = opcode >> 4;
    let reg = *get_arithmetic_16reg(cpu, reg);
    let hl_value = *cpu.hl.value();

    let (value, overflow) = hl_value.overflowing_add(reg);
    cpu.set_by_value(Flag::Carry, overflow);
    cpu.set_by_value(
        Flag::HalfCarry,
        check_for_half_carry_third_nible_add(hl_value, reg),
    );
    cpu.unset_flag(Flag::Subtraction);

    *cpu.hl.value() = value;

    //cycles
    return 2;
}

pub fn add_sp_dd(cpu: &mut GbCpu, opcode: u16) -> u8 {
    let dd = (opcode & 0xFF) as i8;
    let temp = cpu.stack_pointer as i16;

    cpu.stack_pointer = temp.wrapping_add(dd as i16) as u16;

    cpu.unset_flag(Flag::Zero);
    cpu.unset_flag(Flag::Subtraction);
    cpu.set_by_value(
        Flag::Carry,
        signed_check_for_carry_first_nible_add(temp as i16, dd),
    );
    cpu.set_by_value(
        Flag::HalfCarry,
        signed_check_for_half_carry_first_nible_add(temp as i16, dd),
    );

    //cycles
    return 4;
}

pub fn inc_rr(cpu: &mut GbCpu, opcode: u8) -> u8 {
    let reg = (opcode & 0xF0) >> 4;
    let reg = get_arithmetic_16reg(cpu, reg);
    *reg = (*reg).wrapping_add(1);

    //cycles
    return 2;
}

pub fn dec_rr(cpu: &mut GbCpu, opcode: u8) -> u8 {
    let reg = (opcode & 0xF0) >> 4;
    let reg = get_arithmetic_16reg(cpu, reg);
    *reg = (*reg).wrapping_sub(1);

    //cycles
    return 2;
}
