use std::{collections::HashMap, fmt::Debug};

use symbol_table::VariableName;

use crate::{
    ByteCode, Compile, byte_code::Location, runtime::program_counter::ProgramCounter, var_u::Ident,
};

#[derive(Debug)]
pub struct CompiledRoutine {
    byte_code: ByteCode,
    tags: HashMap<Ident, (Location, Option<Vec<VariableName>>)>,
}

impl CompiledRoutine {
    pub fn beginning_of_file(&self) -> ProgramCounter<'_> {
        ProgramCounter::new(&self.byte_code)
    }
    /// Returns position + parameter list details for the requested tag.
    /// NOTE: Follows do calling convention so None -> start of the routine.
    pub fn tag(
        &self,
        tag_name: Option<Ident>,
    ) -> Option<(ProgramCounter<'_>, &Option<Vec<VariableName>>)> {
        match tag_name {
            Some(tag) => {
                let (index, parameters) = self.tags.get(&tag)?;
                let mut pc = ProgramCounter::new(&self.byte_code);
                pc.jump(*index);
                Some((pc, parameters))
            }
            None => Some((self.beginning_of_file(), &None)),
        }
    }
}

pub fn compile_routine(routine: ir::Routine) -> CompiledRoutine {
    let mut byte_code = ByteCode::new();
    let mut tags = HashMap::new();
    for (i, line) in routine.iter().enumerate() {
        if let Some(tag) = &line.tag {
            let tag = &tag.inner;

            if let Some(_) = tags.insert(
                tag.name.as_str().try_into().unwrap(),
                (
                    byte_code.current_location(),
                    tag.parameters.as_ref().map(|x| {
                        x.iter()
                            .map(|x| VariableName::new(x.0.as_bytes()).unwrap())
                            .collect()
                    }),
                ),
            ) {
                panic!("Duplicate key");
            };
        }

        line.compile(&mut byte_code, &(i as u16));
    }
    CompiledRoutine { byte_code, tags }
}
#[cfg(test)]
pub mod test {
    use ir::Routine;

    use crate::compile_routine;
    pub fn test_compile_command(source_code: &str) -> Vec<u8> {
        let commands = frontend::parse_routine_print_errors(&format!(" {source_code}")).unwrap();
        dbg!(&commands);
        const LENGTH_OF_LINE_START: usize = 5;
        compile_routine(commands).byte_code.0[LENGTH_OF_LINE_START..].to_vec()
    }

    pub fn parse_routine(source_code: &str) -> Routine {
        frontend::parse_routine_print_errors(source_code).unwrap()
    }
}
