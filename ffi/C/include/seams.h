typedef struct number_result{
    int is_error;
    int error;
    int value;
}NumberResult;


NumberResult parse_env(char* env, uci_tab* vol_label);
