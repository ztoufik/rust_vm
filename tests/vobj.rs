use vm::vobj::*;
use vm::err::*;

#[test]
fn add(){
    let value1=Vobj::Double(0.3);
    let value2=Vobj::Double(0.15);
    let result=Vobj::add(&value1, &value2).unwrap();
    let expected=Vobj::Double(0.3+0.15);
    assert_eq!(expected,result);
}

#[test]
fn sub(){
    let value1=Vobj::Double(0.3);
    let value2=Vobj::Double(0.15);
    let result=Vobj::sub(&value1, &value2).unwrap();
    let expected=Vobj::Double(0.3-0.15);
    assert_eq!(expected,result);
}

#[test]
fn mul(){
    let value1=Vobj::Double(0.3);
    let value2=Vobj::Int(10);
    let result=Vobj::mul(&value1, &value2).unwrap();
    let expected=Vobj::Double(3.0);
    assert_eq!(expected,result);
}

#[test]
fn div_float(){
    let value1=Vobj::Double(1.0);
    let value2=Vobj::Double(0.15);
    let result=Vobj::div(&value1, &value2).unwrap();
    let expected=Vobj::Double(1.0/0.15);
    assert_eq!(expected,result);
}

#[test]
fn div_int(){
    let value1=Vobj::Int(10);
    let value2=Vobj::Int(3);
    let result=Vobj::div(&value1, &value2).unwrap();
    let expected=Vobj::Int(10/3);
    assert_eq!(expected,result);
}

#[test]
fn div_byzero(){
    let value1=Vobj::Double(10.0);
    let value2=Vobj::Int(0);
    let result=Vobj::div(&value1, &value2);
    if let Err(VMError::DivisionByZeroErr)=result{
        assert!(true,"divisionbyzero error returned")
    }
    else {
        assert!(false,"no divisionbyzero error returned")
    }
}

#[test]
fn greater_than(){
    let value1=Vobj::Double(10.0);
    let value2=Vobj::Int(0);
    let result=Vobj::greater_than(&value1, &value2).unwrap();
    let expected=true;
    assert_eq!(expected,result);
}

#[test]
fn greater_eq(){
    let value1=Vobj::Double(10.0);
    let value2=Vobj::Int(0);
    let result=Vobj::greater_eq(&value1, &value2).unwrap();
    let expected=true;
    assert_eq!(expected,result);
}

#[test]
fn less_than(){
    let value1=Vobj::Int(10);
    let value2=Vobj::Int(30);
    let result=Vobj::less_than(&value1, &value2).unwrap();
    let expected=true;
    assert_eq!(expected,result);
}

#[test]
fn less_eq(){
    let value1=Vobj::Int(100);
    let value2=Vobj::Int(999);
    let result=Vobj::less_eq(&value1, &value2).unwrap();
    let expected=true;
    assert_eq!(expected,result);
}
