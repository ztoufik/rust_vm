
#[cfg(test)]
mod test {

    use vm::vm::*;
    use vm::err::Source;
    use vm::vobj::*;
    use vm::instruction::Instruction;


    #[test]
    fn vm_load_test() {
        let operand1 = Vobj::Double(3.3);
        let src = Source::new("test.zt", 0);
        let inst = Instruction::load_instruction(operand1.clone(), src);
        let code = vec![inst];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, operand1);
        }
        else {
            assert!(false,"empty stack")
        }
    }

    #[test]
    fn vm_add_test() {
        let operand1 = Vobj::Double(3.3);
        let operand2 = Vobj::Int(10);
        let expected = Vobj::Double(13.3);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::load_instruction(operand2.clone(), src.clone());
        let inst3 = Instruction::add(src);
        let code = vec![inst1,inst2,inst3];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }

    #[test]
    fn vm_sub_test() {
        let operand1 = Vobj::Int(11);
        let operand2 = Vobj::Double(1.0);
        let expected = Vobj::Double(10.0);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::load_instruction(operand2.clone(), src.clone());
        let inst3 = Instruction::sub(src);
        let code = vec![inst1,inst2,inst3];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }

    #[test]
    fn vm_mul_test() {
        let operand1 = Vobj::Double(11.33);
        let operand2 = Vobj::Int(10);
        let expected = Vobj::Double(113.3);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::load_instruction(operand2.clone(), src.clone());
        let inst3 = Instruction::mul(src);
        let code = vec![inst1,inst2,inst3];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }
     
    #[test]
    fn vm_div_test() {
        let operand1 = Vobj::Double(113.3);
        let operand2 = Vobj::Int(10);
        let expected = Vobj::Double(11.33);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::load_instruction(operand2.clone(), src.clone());
        let inst3 = Instruction::div(src);
        let code = vec![inst1,inst2,inst3];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }

    #[test]
    fn vm_divbyzero_test() {
        let operand1 = Vobj::Double(113.3);
        let operand2 = Vobj::Int(0);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::load_instruction(operand2.clone(), src.clone());
        let inst3 = Instruction::div(src);
        let code = vec![inst1,inst2,inst3];
        let vm = Vm::load(code);
        let result=vm.run();
        if let Err(msg)=result{
            assert!(true,"{}",msg)
        }
        else {
            assert!(false,"division by zero error should be thrown")
        }
    }

    #[test]
    fn vm_br_test() {
        let operand1 = Vobj::Int(1);
        let operand2 = Vobj::Int(2);
        let operand3 = Vobj::Int(3);
        let operand4 = Vobj::Int(4);
        let expected = Vobj::Int(7);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1, src.clone());
        let inst2 = Instruction::load_instruction(operand2, src.clone());
        let inst3 = Instruction::add(src.clone());
        let inst4 = Instruction::br(5,src.clone());
        let inst5 = Instruction::load_instruction(operand3, src.clone());
        let inst6 = Instruction::load_instruction(operand4, src.clone());
        let inst7 = Instruction::add(src);
        let code = vec![inst1,inst2,inst3,inst4,inst5,inst6,inst7];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }

    #[test]
    fn vm_beq_test() {
        let operand1 = Vobj::Int(1);
        let operand2 = Vobj::Str("op2".to_string());
        let operand3 = Vobj::Int(3);
        let operand4 = Vobj::Int(4);
        let expected = Vobj::Int(5);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1.clone(), src.clone());
        let inst2 = Instruction::load_instruction(operand2.clone(), src.clone());
        let inst3 = Instruction::load_instruction(operand2, src.clone());
        let inst4 = Instruction::beq(5,src.clone());
        let inst5 = Instruction::load_instruction(operand3, src.clone());
        let inst6 = Instruction::load_instruction(operand4, src.clone());
        let inst7 = Instruction::add(src.clone());
        let code = vec![inst1,inst2,inst3,inst4,inst5,inst6,inst7,];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }

    #[test]
    fn vm_bgt_test() {
        let operand1 = Vobj::Double(1.1);
        let operand2 = Vobj::Double(3.1);
        let operand3 = Vobj::Int(3);
        let operand4 = Vobj::Int(4);
        let expected = Vobj::Double(5.1);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1, src.clone());
        let inst2 = Instruction::load_instruction(operand2, src.clone());
        let inst3 = Instruction::load_instruction(operand3.clone(), src.clone());
        let inst4 = Instruction::bgt(5,src.clone());
        let inst5 = Instruction::load_instruction(operand3, src.clone());
        let inst6 = Instruction::load_instruction(operand4, src.clone());
        let inst7 = Instruction::add(src.clone());
        let code = vec![inst1,inst2,inst3,inst4,inst5,inst6,inst7,];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }

    #[test]
    fn vm_blt_test() {
        let operand1 = Vobj::Double(1.1);
        let operand2 = Vobj::Double(2.9);
        let operand3 = Vobj::Int(3);
        let operand4 = Vobj::Int(4);
        let expected = Vobj::Double(5.1);
        let src = Source::new("test.zt", 0);
        let inst1 = Instruction::load_instruction(operand1, src.clone());
        let inst2 = Instruction::load_instruction(operand2, src.clone());
        let inst3 = Instruction::load_instruction(operand3.clone(), src.clone());
        let inst4 = Instruction::blt(5,src.clone());
        let inst5 = Instruction::load_instruction(operand3, src.clone());
        let inst6 = Instruction::load_instruction(operand4, src.clone());
        let inst7 = Instruction::add(src.clone());
        let code = vec![inst1,inst2,inst3,inst4,inst5,inst6,inst7,];
        let vm = Vm::load(code);
        let _=vm.run();
        let binding=vm.dump_mem();
        let result=(*binding).last();
        if let Some(value)=result{
            assert_eq!(*value, expected);
        }
        else {
            assert!(false,"empty stack")
        }
    }
}
