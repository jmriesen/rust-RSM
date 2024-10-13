#[cfg(test)]
mod tests {
    use ffi::{run::Error, CSTRING, LABEL_BLOCK, UCIS, UCI_TAB, VAR_U};

    #[test]
    fn null_env() {
        let env = c"";
        let ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        assert_eq!(crate::bindings::run::parse_env(env, &ucis), Ok(1));
    }

    #[test]
    fn env_not_found_is_an_error() {
        let env = c"env name";
        let ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        assert_eq!(crate::bindings::run::parse_env(env, &ucis), Err(Error(2)));
    }

    fn copy_into(des: &mut ffi::VAR_U, src: &std::ffi::CStr) {
        let bytes = src.to_bytes();
        let raw = unsafe { &mut des.var_cu };
        let len = bytes.len().min(raw.len());
        raw[..len].copy_from_slice(&bytes[..len]);
    }

    #[test]
    fn env_is_found() {
        let mut ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        let env_one = c"env one";
        let env_two = c"env two";

        copy_into(&mut ucis[1].name, env_one);
        copy_into(&mut ucis[2].name, env_two);
        assert_eq!(crate::bindings::run::parse_env(env_one, &ucis), Ok(2));
        assert_eq!(crate::bindings::run::parse_env(env_two, &ucis), Ok(3));
    }

    #[test]
    fn env_name_is_trucated() {
        let mut ucis = [UCI_TAB {
            name: VAR_U { var_cu: [0; 32] },
            global: 0,
        }; 64];
        let max_var_len_plus_0 = c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let max_var_len_plus_1 = c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        copy_into(&mut ucis[1].name, max_var_len_plus_0);
        assert_eq!(
            crate::bindings::run::parse_env(max_var_len_plus_0, &ucis),
            Ok(2)
        );
        assert_eq!(
            crate::bindings::run::parse_env(max_var_len_plus_1, &ucis),
            Ok(2)
        );
    }
}
