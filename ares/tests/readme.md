| name                                                                         | binding            | emit               | output             | result             |
|---|---|---|---|---|
| ./tests/aa_continuation.artest/no args reset                                 | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/no args reset with value                      | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/one arg reset no body                         | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/reset and shift                               | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/nested resets                                 |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/nested resets with shift                      |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/nested resets with shift 2                    |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/nested shifts                                 |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/resume 1                                      |                    |                    | :heavy_check_mark: |                    |
| ./tests/aa_continuation.artest/resume 2                                      |                    |                    | :heavy_check_mark: |                    |
| ./tests/aa_continuation.artest/pauses                                        |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/resume with value                             |                    |                    | :heavy_check_mark: |                    |
| ./tests/aa_continuation.artest/stack messiness                               |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/shift internal                                |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/aa_continuation.artest/multiple-shift internal                       |                    |                    | :heavy_check_mark: |                    |
| ./tests/aa_continuation.artest/external resume                               |                    |                    | :heavy_check_mark: |                    |
| ./tests/assignment.artest/local assignment                                   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/assignment.artest/assignment to argument                             | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/block.artest/basic block expression                                  | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/block.artest/block expression in expression                          | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/block.artest/block expression with statement in expression           | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/block.artest/block expression with statement in expression           | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/block.artest/block statement                                         | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/closure.artest/close over local                                      | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/closure.artest/closure counters                                      |                    |                    | :heavy_check_mark: |                    |
| ./tests/closure.artest/toggler                                               |                    |                    | :heavy_check_mark: |                    |
| ./tests/closure.artest/incrementor argument                                  |                    | :heavy_check_mark: |                    |                    |
| ./tests/closure.artest/fibonacci                                             |                    |                    | :heavy_check_mark: |                    |
| ./tests/closure.artest/combination of upvars =                               |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/closure.artest/abomination                                           |                    |                    |                    | :heavy_check_mark: |
| ./tests/closure.artest/y combinator                                          |                    |                    | :heavy_check_mark: |                    |
| ./tests/comparison.artest/less than                                          | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/less than 2                                        |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/less than 3                                        |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/less than or equal to                              | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/less than or equal to 2                            |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/less than or equal to 2                            |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/greater than                                       | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/greater than 2                                     |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/greater than 3                                     |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/greater than or equal to                           | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/greater than or equal to 2                         |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/greater than 2                                     |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/comparison.artest/eq                                                 |                    |                    | :heavy_check_mark: |                    |
| ./tests/comparison.artest/neq                                                |                    |                    | :heavy_check_mark: |                    |
| ./tests/examples.artest/recursion                                            | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                    |
| ./tests/examples.artest/fibb                                                 |                    |                    | :heavy_check_mark: |                    |
| ./tests/function_call.artest/anonymous function call                         | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/function_call.artest/user-fn function call                           | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/function_call.artest/user call from function                         | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/function_call.artest/curried function                                | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/function_call.artest/empty function                                  | :heavy_check_mark: |                    |                    |                    |
| ./tests/global_vars.artest/global_variable                                   | :heavy_check_mark: |                    |                    |                    |
| ./tests/if.artest/if statement                                               | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/if.artest/if expression both branches take true                      | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/if.artest/if statement both branches take false                      |                    |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/if.artest/nested if expression                                       | :heavy_check_mark: | :heavy_check_mark: |                    |                    |
| ./tests/if.artest/nested if statement                                        | :heavy_check_mark: | :heavy_check_mark: |                    |                    |
| ./tests/lambda.artest/one-arg lambda                                         | :heavy_check_mark: | :heavy_check_mark: |                    |                    |
| ./tests/lambda.artest/one-arg statement lambda                               | :heavy_check_mark: | :heavy_check_mark: |                    |                    |
| ./tests/lambda.artest/2-arg expression lambda                                | :heavy_check_mark: | :heavy_check_mark: |                    |                    |
| ./tests/lambda.artest/empty closure                                          |                    | :heavy_check_mark: |                    |                    |
| ./tests/literals.artest/int literal                                          |                    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/literals.artest/string literal                                       |                    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/literals.artest/symbol literal                                       |                    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/literals.artest/float literal                                        |                    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/literals.artest/bool literal: true                                   |                    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/literals.artest/bool literal: false                                  |                    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/math.artest/addition                                                 | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/math.artest/subtraction                                              | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/math.artest/multiplication                                           | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/math.artest/division                                                 | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/old_emit_tests.artest/test_add_emit_1                                |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/test_add_emit_2                                |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/test_sub_emit                                  |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/if expression                                  |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/test_fn_with_expr                              |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/test_emit_fn_call                              |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/test_emit_one_arg_lambda                       |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/test_emit_if_statement_no_else                 |                    | :heavy_check_mark: |                    |                    |
| ./tests/old_emit_tests.artest/test_emit_if_statement                         |                    | :heavy_check_mark: |                    |                    |
| ./tests/operator_precedence.artest/addition                                  | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/operator_precedence.artest/subtraction                               | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/operator_precedence.artest/multiplication                            | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/operator_precedence.artest/division                                  | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/operator_precedence.artest/addition and multiplication               | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/operator_precedence.artest/addition and multiplication (with parens) | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/operator_precedence.artest/subtraction and division                  | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/operator_precedence.artest/subtraction and division (with parens)    | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/top_level.artest/var followed by print                               | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                    |
| ./tests/top_level.artest/"returned" value                                    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/top_level.artest/global if                                           | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/variable_definition.artest/use local and argument                    | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/variable_definition.artest/two locals                                | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
| ./tests/variable_definition.artest/two locals with expr inbetween            | :heavy_check_mark: |                    | :heavy_check_mark: | :heavy_check_mark: |
