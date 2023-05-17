# How big is the RESAST?

## Spanned

| Item                                                                                    | Size    | Notes               |
| ---                                                                                     | ----    | ---                 |
|[ProgramPart](https://docs.rs/resast/0.5.0/resast/spanned/enum.ProgramPart.html)         |   1.84k |                     |
|[Stmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/enum.Stmt.html)                  |   1.84k | ProgramPart variant |
|[Decl](https://docs.rs/resast/0.5.0/resast/spanned/decl/enum.Decl.html)                  | 840.00b | ProgramPart variant |
|[Dir](https://docs.rs/resast/0.5.0/resast/spanned/struct.Dir.html)                       | 320.00b | ProgramPart variant |
|[ForStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.ForStmt.html)          |   1.83k |        Stmt variant |
|[ForOfStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.ForOfStmt.html)      |   1.53k |        Stmt variant |
|[ForInStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.ForInStmt.html)      |   1.52k |        Stmt variant |
|[TryStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.TryStmt.html)          | 832.00b |        Stmt variant |
|[SwitchStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.SwitchStmt.html)    | 832.00b |        Stmt variant |
|[DoWhileStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.DoWhileStmt.html)  | 824.00b |        Stmt variant |
|[LoopLeft](https://docs.rs/resast/0.5.0/resast/spanned/stmt/enum.LoopLeft.html)          | 800.00b |        Stmt variant |
|[IfStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.IfStmt.html)            | 712.00b |        Stmt variant |
|[WithStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.WithStmt.html)        | 704.00b |        Stmt variant |
|[WhileStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.WhileStmt.html)      | 704.00b |        Stmt variant |
|[LoopInit](https://docs.rs/resast/0.5.0/resast/spanned/stmt/enum.LoopInit.html)          | 528.00b |        Stmt variant |
|[BlockStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.BlockStmt.html)      | 136.00b |        Stmt variant |
|[LabeledStmt](https://docs.rs/resast/0.5.0/resast/spanned/stmt/struct.LabeledStmt.html)  | 120.00b |        Stmt variant |
|(VarKind, VarDecl)                                                                       | 800.00b |    LoopLeft variant |
|[Expr](https://docs.rs/resast/0.5.0/resast/spanned/expr/enum.Expr.html)                  | 528.00b |                     |
|[Pat](https://docs.rs/resast/0.5.0/resast/spanned/pat/enum.Pat.html)                     | 144.00b |                     |
|[Slice](https://docs.rs/resast/0.5.0/resast/spanned/struct.Slice.html)                   |  56.00b |                     |
|[SourceLocation](https://docs.rs/resast/0.5.0/resast/spanned/struct.SourceLocation.html) |  32.00b |                     |
|[Position](https://docs.rs/resast/0.5.0/resast/spanned/struct.Position.html)             |  16.00b |                     |
