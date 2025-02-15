use crate::{BiteCode, Compile};

impl Compile for &str {
    type Context = ();
    fn compile(&self, bite_code: &mut BiteCode, _: &Self::Context) {
        let var_u: ffi::VAR_U = (*self).try_into().unwrap();
        bite_code.extend(var_u.as_array().iter().cloned())
    }
}
