mod infra;

// Your tests go here!
success_tests! {
    {
        name: cobra_false_val,
        file: "cobra_false_val.snek",
        expected: "false",
    },
    {
        name: cobra_input_compare_1,
        file: "cobra_input_compare.snek",
        input: "2",
        expected: "false",
    },
    {
        name: cobra_input_compare_2,
        file: "cobra_input_compare.snek",
        input: "10",
        expected: "true",
    },
    {
        name: cobra_add1,
        file: "cobra_add1.snek",
        expected: "73",
    },
    {
        name: cobra_add,
        file: "cobra_add.snek",
        expected: "15",
    },
    {
        name: cobra_nested_arith,
        file: "cobra_nested_arith.snek",
        expected: "25",
    },
    {
        name: cobra_binding,
        file: "cobra_binding.snek",
        expected: "5",
    },
    {
        name: cobra_nesting_hell1,
        file: "cobra_nesting_hell1.snek",
        expected: "1105",
    },
    {
        name: cobra_nesting_hell2,
        file: "cobra_nesting_hell2.snek",
        expected: "10546",
    },
    {
        name: cobra_nesting_hell3,
        file: "cobra_nesting_hell3.snek",
        expected: "12",
    },
    {
        name: cobra_nesting_hell4,
        file: "cobra_nesting_hell4.snek",
        expected: "100",
    },
    {
        name: cobra_shadowing,
        file: "cobra_shadowing.snek",
        expected: "6",
    },
    {
        name: cobra_many_bindings,
        file: "cobra_many_bindings.snek",
        expected: "11111111",
    },
    {
        name: cobra_sub_pos,
        file: "cobra_sub_pos.snek",
        expected: "50",
    },
    {
        name: cobra_sub_neg,
        file: "cobra_sub_neg.snek",
        expected: "-50",
    },
    {
        name: cobra_sub1,
        file: "cobra_sub1.snek",
        expected: "899",
    },
    {
        name: cobra_mult,
        file: "cobra_mult.snek",
        expected: "100",
    },
    {
        name: cobra_mult_by_one,
        file: "cobra_mult_by_one.snek",
        expected: "10",
    },
    {
        name: cobra_mult_by_zero,
        file: "cobra_mult_by_zero.snek",
        expected: "0",
    },
    {
        name: cobra_nested_mult,
        file: "cobra_nested_mult.snek",
        expected: "1000000",
    },
    {
        name: cobra_nested_add,
        file: "cobra_nested_add.snek",
        expected: "16",
    },
    {
        name: cobra_nested_sub,
        file: "cobra_nested_sub.snek",
        expected: "6",
    },
    {
        name: cobra_given_test_set,
        file: "cobra_given_test_set.snek",
        expected: "6",
    },
    {
        name: cobra_given_complex_loop,
        file: "cobra_given_complex_loop.snek",
        expected: "-6",
    },
    {
        name: cobra_given_factorial_input_1,
        file: "cobra_given_factorial_input.snek",
        input: "1",
        expected: "1",
    },
    {
        name: cobra_given_factorial_input_2,
        file: "cobra_given_factorial_input.snek",
        input: "2",
        expected: "2",
    },
    {
        name: cobra_given_factorial_input_4,
        file: "cobra_given_factorial_input.snek",
        input: "4",
        expected: "24",
    },
    {
        name: cobra_given_factorial_input_10,
        file: "cobra_given_factorial_input.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: cobra_test_1_greater_0_input,
        file: "cobra_test_if_greater0_input.snek",
        input: "1",
        expected: "true",
    },
    {
        name: cobra_test_0_greater_0_input,
        file: "cobra_test_if_greater0_input.snek",
        input: "0",
        expected: "false",
    },
    {
        name: cobra_test_999_greater_0_input,
        file: "cobra_test_if_greater0_input.snek",
        input: "999",
        expected: "true",
    },
    {
        name: cobra_test_1_greatereq_0_input,
        file: "cobra_test_if_greatereq0_input.snek",
        input: "1",
        expected: "true",
    },
    {
        name: cobra_test_0_greatereq_0_input,
        file: "cobra_test_if_greatereq0_input.snek",
        input: "0",
        expected: "true",
    },
    {
        name: cobra_test_neg1_greater_0_input,
        file: "cobra_test_if_greater0_input.snek",
        input: "-1",
        expected: "false",
    },
    {
        name: cobra_test_neg1_less_0_input,
        file: "cobra_test_if_less0_input.snek",
        input: "-1",
        expected: "true",
    },
    {
        name: cobra_test_1_less_0_input,
        file: "cobra_test_if_less0_input.snek",
        input: "1",
        expected: "false",
    },
    {
        name: cobra_test_1_lesseq_0_input,
        file: "cobra_test_if_lesseq0_input.snek",
        input: "1",
        expected: "false",
    },
    {
        name: cobra_test_0_lesseq_0_input,
        file: "cobra_test_if_lesseq0_input.snek",
        input: "0",
        expected: "true",
    },
    {
        name: cobra_isnum_input_1,
        file: "cobra_isnum_input.snek",
        input: "1",
        expected: "true",
    },
    {
        name: cobra_isnum_input_neg1,
        file: "cobra_isnum_input.snek",
        input: "-1",
        expected: "true",
    },
    {
        name: cobra_isnum_input_100,
        file: "cobra_isnum_input.snek",
        input: "100",
        expected: "true",
    },
    {
        name: cobra_isnum_input_false,
        file: "cobra_isnum_input.snek",
        input: "false",
        expected: "false",
    },
    {
        name: cobra_isnum_input_true,
        file: "cobra_isnum_input.snek",
        input: "true",
        expected: "false",
    },
    {
        name: cobra_isbool_input_true,
        file: "cobra_isbool_input.snek",
        input: "true",
        expected: "true",
    },
    {
        name: cobra_isbool_input_false,
        file: "cobra_isbool_input.snek",
        input: "false",
        expected: "true",
    },
    {
        name: cobra_isbool_input_100,
        file: "cobra_isbool_input.snek",
        input: "100",
        expected: "false",
    },
    {
        name: cobra_isbool_input_neg100,
        file: "cobra_isbool_input.snek",
        input: "-100",
        expected: "false",
    },
     {
        name: dimondback_fact,
        file: "fact.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: dimondback_fact_recursive_single_recurrsion,
        file: "recurssive_factorial.snek",
        input: "1",
        expected: "1",
    },
    {
        name: dimondback_fact_recursive,
        file: "recurssive_factorial.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: dimondback_fact_recursive_deep,
        file: "recurssive_factorial.snek",
        input: "20",
        expected: "2432902008176640000",
    },
    {
        name: dimondback_even_odd_1,
        file: "even_odd.snek",
        input: "10",
        expected: "10\ntrue\ntrue",
    },
    {
        name: dimondback_even_odd_2,
        file: "even_odd.snek",
        input: "9",
        expected: "9\nfalse\nfalse",
    },
    {
        name: dimondback_many_print,
        file: "many_print.snek",
        expected: "11",
    },
    {
        name: dimondback_print_boolean,
        file: "mytest.snek",
        expected: "false\nfalse",
    },
    {
        name: dimondback_fun_no_args,
        file: "func_no_args.snek",
        expected: "32\n132\ntrue\ntrue",
    },
    {
        name: dimondback_fun_nested_call,
        file: "max.snek",
        expected: "10",
    },
    {
        name: dimondback_fun_nested_call_2,
        file: "nested_fun.snek",
        expected: "2",
    },
    {
        name: dimondback_fun_many_args,
        file: "fun_many_args.snek",
        expected: "2432902008176640000\n2432902008176640000",
    }
}

runtime_error_tests! {
    {
        name: cobra_invalid_argument,
        file: "cobra_invalid_argument.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_input_compare_3,
        file: "cobra_input_compare.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: cobra_input_invalid_greater,
        file: "cobra_test_if_greater0_input.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: cobra_input_invalid_less,
        file: "cobra_test_if_less0_input.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: cobra_test_overflow,
        file: "cobra_test_overflow.snek",
        expected: "overflow",
    },
    {
        name: cobra_test_invalid_argument_add1,
        file: "cobra_test_invalid_argument_add1.snek",
        expected: "invalid argument",
    },
    {
        name: cobra_test_invalid_argument_sub1,
        file: "cobra_test_invalid_argument_sub1.snek",
        expected: "invalid argument",
    },

}

static_error_tests! {
    {
        name: cobra_unbound_set,
        file: "cobra_invalid_set.snek",
        expected: "Unbound variable identifier y",
    },
    {
        name: cobra_input_keyword_binding_fail,
        file: "cobra_input_keyword_fail.snek",
        expected: "binding overlaps with reserved keyword input",
    },
    {
        name: cobra_number_bounds_fail,
        file: "cobra_number_bounds_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_number_bounds_fail_neg,
        file: "cobra_number_bounds_fail_neg.snek",
        expected: "Invalid",
    },
    {
        name: cobra_unbound_id,
        file: "cobra_unbound_id.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: cobra_duplicate_binding,
        file: "cobra_duplicate_binding.snek",
        expected: "Duplicate binding",
    },
    {
        name: cobra_null_program_fail,
        file: "cobra_null_program_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_let_nobindings_fail,
        file: "cobra_let_nobindings_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_let_wrong_args_fail,
        file: "cobra_let_wrong_args_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_add_wrongargs_fail,
        file: "cobra_add_wrongargs_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_sub_wrongargs_fail,
        file: "cobra_sub_wrongargs_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_mul_wrongargs_fail,
        file: "cobra_mul_wrongargs_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_sub1_fail,
        file: "cobra_sub1_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_add1_fail,
        file: "cobra_add1_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_invalid_id_fail,
        file: "cobra_invalid_id_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_sexp_fail,
        file: "cobra_sexp_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_parse_block_fail,
        file: "cobra_parse_block_fail.snek",
        expected: "Invalid",
    },
    {
        name: cobra_unbound_identifier_if,
        file: "cobra_unbound_identifier_if.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: cobra_unbound_identifier_set,
        file: "cobra_unbound_identifier_set.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: cobra_unbound_identifier_break,
        file: "cobra_unbound_identifier_break.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: cobra_unbound_identifier_block,
        file: "cobra_unbound_identifier_block.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: dimondback_duplicate_parameters,
        file: "duplicate_params.snek",
        expected: "Invalid",
    },
    {
        name: dimondback_fucntion_no_body,
        file: "fun_no_body.snek",
        expected: "Invalid",
    },
    {
        name: dimondback_invalid_function,
        file: "invalid_fun.snek",
        expected: "Invalid",
    },
}