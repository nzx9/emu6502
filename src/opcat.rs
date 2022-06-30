#[derive(Debug)]
pub enum OpCat {
    LoadStore,
    Register,
    Stack,
    Logical,
    Arithmetic,
    IncDec,
    Shifts,
    JumpCall,
    Branch,
    StatusCtrl,
    SysFun,
    Unimpl,
}

impl OpCat {
    pub fn as_str(&self) -> String {
        let op_cat = match self {
            OpCat::LoadStore => "Load/Store",
            OpCat::Register => "Register",
            OpCat::Stack => "Stack",
            OpCat::Logical => "Logical",
            OpCat::Arithmetic => "Arithmetic",
            OpCat::IncDec => "Inc/Dec",
            OpCat::Shifts => "Shifts",
            OpCat::JumpCall => "Jump/Call",
            OpCat::Branch => "Branch",
            OpCat::StatusCtrl => "Status Contrl",
            OpCat::SysFun => "System Function",
            OpCat::Unimpl => "Unimplemented",
        };
        op_cat.to_string()
    }
}
