| name                                          | binding | emit   | output | result |
--------------------------------------------------------------------------------------
| basic block expression                        | :check | :check | :check | :check |
| block expression in expression                | :check | :check | :check | :check |
| block expression with statement in expression | :check | :check | :check | :check |
| block expression with statement in expression | :check | :check | :check | :check |
| block statement                               | :check | :check | :check | :check |
| anonymous function call                       | :check |        | :check | :check |
| user-fn function call                         | :check |        | :check | :check |
| user call from function                       | :check |        | :check | :check |
| curried function                              | :check |        | :check | :check |
| one-arg lambda                                | :check |        |        |        |
| one-arg statement lambda                      | :check |        |        |        |
| 2-arg expression lambda                       | :check |        |        |        |
| int literal                                   |        |        |        | :check |
| string literal                                |        |        |        | :check |
| symbol literal                                |        |        |        | :check |
| float literal                                 |        |        |        | :check |
| bool literal: true                            |        |        |        | :check |
| bool literal: false                           |        |        |        | :check |
| addition                                      | :check | :check |        | :check |
| subtraction                                   | :check | :check |        | :check |
| multiplication                                | :check | :check |        | :check |
| division                                      | :check | :check |        | :check |
| addition                                      | :check |        | :check | :check |
| subtraction                                   | :check |        | :check | :check |
| multiplication                                | :check |        | :check | :check |
| division                                      | :check |        | :check | :check |
| addition and multiplication                   | :check |        | :check | :check |
| addition and multiplication (with parens)     | :check |        | :check | :check |
| subtraction and division                      | :check |        | :check | :check |
| subtraction and division (with parens)        | :check |        | :check | :check |
| if statement                                  | :check |        | :check | :check |
| if expression both branches take true         | :check |        | :check |        |
| if statement both branches take false         |        |        | :check | :check |
