use std::{collections::HashMap, fmt::Debug};

use symbol_table::VariableName;

use crate::{
    BiteCode, Compile, bite_code::Location, runtime::program_counter::ProgramCounter, var_u::Ident,
};

pub struct CompiledRoutine {
    byte_code: Vec<u8>,
    tags: HashMap<Ident, (Location, Option<Vec<VariableName>>)>,
}

#[cfg_attr(test, mutants::skip)]
impl Debug for CompiledRoutine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO Clean up the debugging experience.
        let mut tags = self.tags.iter().collect::<Vec<_>>();
        tags.sort_by_key(|x| x.1.0);
        f.debug_struct("CompiledRoutine")
            .field("tags", &tags)
            .field("byte_code", &self.beginning_of_file())
            .finish()
    }
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
    let mut comp = BiteCode::new();
    let mut tags = HashMap::new();
    for (i, line) in routine.iter().enumerate() {
        if let Some(tag) = &line.tag {
            let tag = &tag.inner;

            if let Some(_) = tags.insert(
                tag.name.as_str().try_into().unwrap(),
                (
                    comp.current_location(),
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

        line.compile(&mut comp, &(i as u16));
    }
    CompiledRoutine {
        byte_code: comp.get_raw(),
        tags,
    }
}
#[cfg(test)]
pub mod test {
    use ir::Routine;

    use crate::compile_routine;
    pub fn test_compile_command(source_code: &str) -> Vec<u8> {
        let commands = frontend::parse_routine_print_errors(&format!(" {source_code}")).unwrap();
        dbg!(&commands);
        const LENGTH_OF_LINE_START: usize = 5;
        compile_routine(commands).byte_code[LENGTH_OF_LINE_START..].to_vec()
    }

    pub fn parse_routine(source_code: &str) -> Routine {
        frontend::parse_routine_print_errors(source_code).unwrap()
    }
}
