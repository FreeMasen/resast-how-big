fn main() {
    {
        use resast::spanned::{
            decl::{Decl, VarDecl},
            expr::Expr,
            pat::Pat,
            stmt::{
                BlockStmt, DoWhileStmt, ForInStmt, ForOfStmt, ForStmt, IfStmt, LabeledStmt,
                LoopInit, LoopLeft, Stmt, SwitchStmt, TryStmt, WhileStmt, WithStmt,
            },
            *,
        };
        println!("----------\nSpanned\n----------");
        print_size::<ProgramPart>("ProgramPart");
        // Largest ProgramPart variant
        print_size::<Stmt>("Stmt");
        print_size::<BlockStmt>("BlockStmt");
        print_size::<WithStmt>("WithStmt");
        print_size::<LabeledStmt>("LabeledStmt");
        print_size::<IfStmt>("IfStmt");
        print_size::<SwitchStmt>("SwitchStmt");
        print_size::<TryStmt>("TryStmt");
        print_size::<WhileStmt>("WhileStmt");
        print_size::<DoWhileStmt>("DoWhileStmt");
        // Largest Stmt variant
        print_size::<ForStmt>("ForStmt");
        print_size::<LoopInit>("LoopInit");
        print_size::<ForInStmt>("ForInStmt");
        print_size::<ForOfStmt>("ForOfStmt");
        print_size::<LoopLeft>("LoopLeft");
        // Largest LoopLeft variant
        print_size::<(VarKind, VarDecl)>("(VarKind, VarDecl)");
        print_size::<Decl>("Decl");
        print_size::<Dir>("Dir");
        print_size::<Expr>("Expr");
        print_size::<Pat>("Pat");
        print_size::<Slice>("Slice");
        print_size::<SourceLocation>("SourceLocation");
        print_size::<Position>("Position");
    }
    {
        use resast::{decl::Decl, expr::Expr, stmt::Stmt, *};
        println!("----------\nUnspanned\n----------");
        print_size::<ProgramPart>("ProgramPart");
        print_size::<Stmt>("Stmt");
        print_size::<Decl>("Decl");
        print_size::<Dir>("Dir");
        print_size::<Expr>("Expr");
    }
}

fn print_size<T>(name: &str) {
    let mut size = std::mem::size_of::<T>() as f64;
    let mut suffix = "b";
    while size > 1024.0 {
        if suffix == "b" {
            suffix = "k";
        } else {
            break;
        }
        size /= 1024.0;
    }
    println!("{name}: {size:.2}{suffix}");
}
