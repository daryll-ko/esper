pub struct VM {
    registers: [i32; 32],
}

impl VM {
    pub fn new() -> VM {
        VM { registers: [0; 32] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_creation() {
        let test_vm = VM::new();
        for i in 0..32 {
            assert_eq!(test_vm.registers[i], 0);
        }
    }
}
