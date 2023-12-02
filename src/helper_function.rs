//use crate::span::Span1;
use crate::asm::{Arg32, Arg64, BinArgs, Instr, MemRef, MovArgs, Reg, Reg32, Offset, JmpArg};
use crate::syntax::{Exp, ImmExp, Prim, SeqExp, FunDecl, SeqProg, VarOrLabel};
use crate::compile::CompileErr;
use std::convert::TryFrom;
use std::vec;

static SNAKE_TRU: u64 = 0xFF_FF_FF_FF_FF_FF_FF_FF;
static SNAKE_FLS: u64 = 0x7F_FF_FF_FF_FF_FF_FF_FF;
static MAX_SNAKE_INT: i64 = i64::max_value() >> 1;
static MIN_SNAKE_INT: i64 = i64::min_value() >> 1;
static BOOL_MASK: u64 = 0x8000_0000_0000_0000;
static TAG_MASK: u64 = 0x7F_FF_FF_FF_FF_FF_FF_FF;
static FLIP_MASK: u64 = 0x80_00_00_00_00_00_00_00;

fn tag_exp_helper<T>(e:&Exp<T>, num:&mut u32) -> Exp<u32> {
    match e {
        Exp::Num(n, _) => Exp::Num(*n, *num),
        Exp::Var(name, _) => Exp::Var(name.clone(), *num),
        Exp::Bool(bool, _) => Exp::Bool(bool.clone(), *num),
        Exp::Prim(pr, exp_lst, _) => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_exp_lst = vec![];
            for exp in exp_lst{
                tag_exp_lst.push(Box::new(tag_exp_helper(exp, num)));
            }
            Exp::Prim(*pr, tag_exp_lst, tag)
        }
        Exp::Let {bindings, body, ..} => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_bindings = vec![];
            for (name, exp) in bindings{
                tag_bindings.push((name.clone(), tag_exp_helper(exp, num)));
            }
            Exp::Let {bindings: tag_bindings, body: Box::new(tag_exp_helper(body, num)), ann: tag}
        }
        Exp::If {cond, thn, els, ..} =>{
            let tag = *num;
            *num = *num + 1;
            Exp::If{
                cond:Box::new(tag_exp_helper(cond, num)), 
                thn:Box::new(tag_exp_helper(thn, num)), 
                els:Box::new(tag_exp_helper(els, num)), 
                ann: tag
            }
        }
        Exp::FunDefs { decls, body, .. } => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_decls = vec![];
            for decl in decls{
                tag_decls.push(FunDecl{
                    name: decl.name.clone(), 
                    parameters: decl.parameters.clone(), 
                    body: tag_exp_helper(&decl.body, num), 
                    ann: *num
                });
                *num = *num + 1;
            }
            Exp::FunDefs {decls: tag_decls, body: Box::new(tag_exp_helper(body, num)), ann: tag}
        }
        Exp::Call(name, args, ..) => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_args = vec![];
            for arg in args{
                tag_args.push(tag_exp_helper(arg, num));
            }
            *num = *num + 1;
            Exp::Call(Box::new(tag_exp_helper(name, num)), tag_args, tag)
        }
        Exp::ExternalCall { fun, args, is_tail, .. } => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_args = vec![];
            for arg in args{
                tag_args.push(tag_exp_helper(arg, num));
            }
            Exp::ExternalCall {fun: fun.clone(), args: tag_args, is_tail: *is_tail, ann: tag}
        }
        Exp::InternalTailCall(name, args, ..) => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_args = vec![];
            for arg in args{
                tag_args.push(tag_exp_helper(arg, num));
            }
            Exp::InternalTailCall(name.clone(), tag_args, tag)
        }
        Exp::ClosureCall(fun, args, ..) => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_args = vec![];
            for arg in args{
                tag_args.push(tag_exp_helper(arg, num));
            }
            Exp::ClosureCall(Box::new(tag_exp_helper(fun, num)), tag_args, tag)
        }
        Exp::DirectCall(fun, args, ..) => {
            let tag = *num;
            *num = *num + 1;
            let mut tag_args = vec![];
            for arg in args{
                tag_args.push(tag_exp_helper(arg, num));
            }
            Exp::DirectCall(fun.clone(), tag_args, tag)
        }
        Exp::Lambda { parameters, body, .. } => {
            let tag = *num;
            *num = *num + 1;
            Exp::Lambda {parameters: parameters.clone(), body: Box::new(tag_exp_helper(body, num)), ann: tag}
        }
        Exp::MakeClosure { arity, label, env, .. } => {
            let tag = *num;
            *num = *num + 1;
            Exp::MakeClosure {arity: *arity, label: label.clone(), env: Box::new(tag_exp_helper(env, num)), ann: tag}
        }
        Exp::Semicolon { .. } => {
            panic!("found semicolon");
        }
    }
}
fn tag_seqexp_helper<T>(e:&SeqExp<T>, num:&mut u32) -> SeqExp<u32>{
    match e {
        SeqExp::Imm(imm, _) => {
            let tag = *num;
            *num +=1;
            SeqExp::Imm(imm.clone(), tag)
        },
        SeqExp::Prim(pr, exp_lst, _) => {
            let tag = *num;
            *num +=1;
            SeqExp::Prim(*pr, exp_lst.clone(), tag)
        },
        SeqExp::Let {var, bound_exp, body, ..} => {
            let tag = *num;
            *num +=1;
            SeqExp::Let {
                var: var.clone(), 
                bound_exp: Box::new(tag_seqexp_helper(bound_exp, num)), 
                body: Box::new(tag_seqexp_helper(body, num)), 
                ann: tag
            }
        }
        SeqExp::If {cond, thn, els, ..} =>{
            let tag = *num;
            *num +=1;
            SeqExp::If{
                cond:cond.clone(), 
                thn:Box::new(tag_seqexp_helper(thn, num)), 
                els:Box::new(tag_seqexp_helper(els, num)), 
                ann: tag
            }
        }
        SeqExp::FunDefs { decls, body, .. } => {
            let tag = *num;
            *num +=1;
            let mut tag_decls = vec![];
            for decl in decls{
                tag_decls.push(FunDecl{
                    name: decl.name.clone(), 
                    parameters: decl.parameters.clone(), 
                    body: tag_seqexp_helper(&decl.body, num), 
                    ann: *num
                });
                *num +=1;
            }
            SeqExp::FunDefs {decls: tag_decls, body: Box::new(tag_seqexp_helper(body, num)), ann: tag}
        }
        SeqExp::ExternalCall { fun, args, is_tail, .. } => {
            let tag = *num;
            *num +=1;
            let mut tag_args = vec![];
            for arg in args{
                tag_args.push(arg.clone());
            }
            SeqExp::ExternalCall {fun: fun.clone(), args: tag_args, is_tail: *is_tail, ann: tag}
        }
        SeqExp::InternalTailCall(name, args, ..) => {
            let tag = *num;
            *num +=1;
            let mut tag_args = vec![];
            for arg in args{
                tag_args.push(arg.clone());
            }
            SeqExp::InternalTailCall(name.clone(), tag_args, tag)
        }
        SeqExp::MakeClosure { arity, label, env, ..} => {
            let tag = *num;
            *num +=1;
            SeqExp::MakeClosure {arity: *arity, label: label.clone(), env: env.clone(), ann: tag}
        }
        SeqExp::Semicolon { .. } => {
            panic!("found semicolon");
        }
    }
}
pub fn tag_seqexp<T>(e:&SeqExp<T>) -> SeqExp<u32>{
    tag_seqexp_helper(e, &mut 0)
}
pub fn tag_exp<T>(e:&Exp<T>) -> Exp<u32>{
    tag_exp_helper(e,&mut 0)
}
pub fn tag_prog<T>(p: SeqProg<T>) -> SeqProg<u32>{
    let mut num = 0;
    let mut tag_funs = vec![];
    for fun in p.funs{
        tag_funs.push(FunDecl{
            name: fun.name, 
            parameters: fun.parameters, 
            body: tag_seqexp_helper(&fun.body, &mut num), 
            ann: num
        });
        num += 1;
    }
    SeqProg{
        funs: tag_funs, 
        main: tag_seqexp_helper(&p.main, &mut num), 
        ann: num
    }
}
fn usize_to_i32(x: usize) -> i32 {
    TryFrom::try_from(x).unwrap()
}
fn env_get<Span>(env: &Vec<(String, i32)>, name: &str) -> Option<i32> {
    for (y, e) in (*env).iter().rev(){
        if *y == name {
            return Some(*e);
        }
    }
    None
}
impl<Ann> Exp<Ann> {
    pub fn ann(&self) -> Ann
    where
        Ann: Clone,
    {
        match self {
            Exp::Num(_, a) => a.clone(),
            Exp::Var(_, a) => a.clone(),
            Exp::Prim(_, _, a) => a.clone(),
            Exp::Let { ann: a, .. } => a.clone(),
            Exp::If { ann: a, ..} => a.clone(),
            Exp::Bool(_, a) => a.clone(),
            Exp::Call(_, _, a) => a.clone(),
            Exp::FunDefs { ann: a, .. } => a.clone(),
            Exp::ExternalCall { ann: a, ..} => a.clone(),
            Exp::InternalTailCall(_, _, a) => a.clone(),
            Exp::ClosureCall(_, _, a) => a.clone(),
            Exp::Semicolon { ann: a, .. } => a.clone(),
            Exp::Lambda { ann:a, .. } => a.clone(),
            Exp::MakeClosure { ann: a, .. } => a.clone(),
            Exp::DirectCall(_, _, a) => a.clone(),
        }
    }
}
fn check_duplicate_binding<Span>(bindings: &Vec<(String, Exp<Span>)>) -> Result<(), CompileErr<Span>> 
where Span: Clone
{
    let mut names = vec![];
    for (name, e) in bindings {
        if names.contains(name) {
            return Err(CompileErr::DuplicateBinding {
                duplicated_name: name.to_string(),
                location: e.ann(),
            });
        }
        names.push(name.to_string());
    }
    Ok(())
}
fn check_duplicate_functions<Ann>(decls: &Vec<FunDecl<Exp<Ann>, Ann>>) -> Result<(), CompileErr<Ann>> 
where Ann: Clone
{
    let mut names = vec![];
    for decl in decls {
        if names.contains(&decl.name) {
            return Err(CompileErr::DuplicateFunName {
                duplicated_name: decl.name.to_string(),
                location: decl.ann.clone(),
            });
        }
        names.push(decl.name.to_string());
    }
    Ok(())
}
pub fn check_prog_helper<Span>(e: &Exp<Span>, mut env: Vec<(String, i32)>) -> Result<(), CompileErr<Span>>
where Span: Clone // this means you can use the clone method on Span (which you will need to do)
{
    match e {
        Exp::Num(n,..) => {
        if *n > MAX_SNAKE_INT || *n < MIN_SNAKE_INT {
            return Err(CompileErr::Overflow {
                num: *n,
                location: e.ann(),
            })
        } else {
            Ok(())
        }
        }
        Exp::Bool(..) => Ok(()),
        Exp::Var(name, span) => {
            if env_get::<Span>(&env, name).is_none() {
                return Err(CompileErr::UnboundVariable {
                    unbound: name.to_string(),
                    location: (*span).clone(),
                })
            } Ok(())
        },
        Exp::Prim(_, e, _) => {
            for exp in e{
                match check_prog_helper(exp, env.clone()){
                    Ok(_) => continue,
                    Err(a) => return Err(a),
                }
            }
            Ok(())
        },
        Exp::Let{bindings, body, ann: _} => {
            check_duplicate_binding(bindings)?;
            for (name, e) in bindings {
                check_prog_helper(e, env.clone())?;
                //let offset = usize_to_i32(env.len());
                // -1 for var, n for fun with n args
                env.push((name.to_string(), -1));
            }
            check_prog_helper(body, env)
        }
        Exp::If{cond, thn, els, ann: _} => {
            check_prog_helper(cond, env.clone())?;
            check_prog_helper(thn, env.clone())?;
            check_prog_helper(els, env)
        }
        Exp::FunDefs { decls, body, ann: _ } => {
            check_duplicate_functions(decls)?;
            for decl in decls {
                let mut arg_names = vec![];
                for arg in &decl.parameters {
                    if arg_names.contains(arg) {
                        return Err(CompileErr::DuplicateArgName {
                            duplicated_name: arg.to_string(),
                            location: decl.ann.clone(),
                        });
                    }
                    arg_names.push(arg.to_string());
                }
                let args_num = decl.parameters.len();
                env.push((decl.name.to_string(), args_num as i32));
            }
            check_prog_helper(body, env)
        }
        Exp::Call(fun, args, ..) => {
            check_prog_helper(fun, env.clone())?;
            for arg in args{
                check_prog_helper(arg, env.clone())?;
            }
            Ok(())
        }
        Exp::Lambda { parameters, body, .. } => {
            let mut para = vec![];
            for arg in parameters{
                if para.contains(arg) {
                    return Err(CompileErr::DuplicateArgName {
                        duplicated_name: arg.to_string(),
                        location: e.ann(),
                    });
                }
                para.push(arg.to_string());
            }
            for para in parameters{
                env.push((para.to_string(), -1));
            }
            check_prog_helper(&body, env)
        }
        Exp::Semicolon { .. } => {
            panic!("found semicolon");
        }
        _ => {
            panic!("internel exp found in desugar_semicolon")
        }
}
}
pub fn sequentialize(e: &Exp<u32>, funs: &mut Vec<(String, Vec<String>)>) -> SeqExp<()> {
    match e {
        Exp::Num(n, _) => SeqExp::Imm(ImmExp::Num(*n), ()),
        Exp::Bool(b, _) => SeqExp::Imm(ImmExp::Bool(*b), ()),
        Exp::Var(name, _) => SeqExp::Imm(ImmExp::Var(name.clone()), ()),
        Exp::Prim(op, exp_lst, ann) => {
            match op {
                | Prim::GetCode | Prim::GetEnv | Prim::CheckArityAndUntag(_) | Prim::Add1 | Prim::Sub1 | Prim::Not| Prim::Print | Prim::IsBool | Prim::IsNum | Prim::IsArray | Prim::IsFun | Prim::Length=>{
                    let s_e1 = sequentialize(exp_lst.get(0).unwrap(), funs);
                    let name1 = format!("#prim1_{}", ann);
                    SeqExp::Let { var: name1.clone(), bound_exp: Box::new(s_e1), ann: (),
                        body: Box::new(SeqExp::Prim(*op, vec![ImmExp::Var(name1)], ()))
                    }
                }
                Prim::ArrayGet| Prim::Add | Prim::And | Prim::Eq | Prim::Ge | Prim::Gt | Prim::Le | Prim::Lt | Prim::Mul | Prim::Neq | Prim::Or | Prim::Sub =>{
                    let s_e1 = sequentialize(exp_lst.get(0).unwrap(), funs);
                    let s_e2 = sequentialize(exp_lst.get(1).unwrap(), funs);
                    let name1 = format!("#prim2_1_{}", ann);
                    let name2 = format!("#prim2_2_{}", ann);
                    SeqExp::Let { var: name1.clone(), bound_exp: Box::new(s_e1), ann: (),
                        body: Box::new(
                        SeqExp::Let { var: name2.clone(), bound_exp: Box::new(s_e2), ann: (),
                            body: Box::new(SeqExp::Prim(*op, vec![ImmExp::Var(name1),ImmExp::Var(name2)], ()))
                        })
                    }
                }
                Prim::ArraySet => {
                    let s_e1 = sequentialize(exp_lst.get(0).unwrap(), funs);
                    let s_e2 = sequentialize(exp_lst.get(1).unwrap(), funs);
                    let s_e3 = sequentialize(exp_lst.get(2).unwrap(), funs);
                    let name1 = format!("#prim2_1_{}", ann);
                    let name2 = format!("#prim2_2_{}", ann);
                    let name3 = format!("#prim2_3_{}", ann);
                    SeqExp::Let { var: name1.clone(), bound_exp: Box::new(s_e1), ann: (),
                        body: Box::new(
                        SeqExp::Let { var: name2.clone(), bound_exp: Box::new(s_e2), ann: (),
                            body: Box::new(
                            SeqExp::Let { var: name3.clone(), bound_exp: Box::new(s_e3), ann: (),
                                body: Box::new(SeqExp::Prim(*op, vec![ImmExp::Var(name1),ImmExp::Var(name2),ImmExp::Var(name3)], ()))
                            })
                        })
                    }
                }
                Prim::MakeArray => {
                    let mut seqs_arg = vec![];
                    let mut names_arg = vec![];
                    let mut arg_count = 0;
                    for arg in exp_lst{
                        let s_arg = sequentialize(arg, funs);
                        let name_arg = format!("#ArrayArg{}_{}", ann, arg_count);
                        arg_count += 1;
                        seqs_arg.push(s_arg);
                        names_arg.push(name_arg);
                    }
                    let mut imm_args = vec![];
                    for name in names_arg.iter(){
                        imm_args.push(ImmExp::Var(name.clone()));
                    }
                    let mut res = SeqExp::Prim(*op, imm_args, ());
                    for i in (0..exp_lst.len()).rev(){
                        res = SeqExp::Let { var: names_arg.get(i).unwrap().clone(), bound_exp: Box::new(seqs_arg.get(i).unwrap().clone()), ann: (),
                        body: Box::new(res)
                        }
                    }
                    res
                }
            }
        }
        Exp::Let { bindings, body, .. } => {
            let mut res = sequentialize(body, funs);
            for (name, exp) in bindings.iter().rev() {
                res = SeqExp::Let { var: name.clone(), bound_exp: Box::new(sequentialize(exp, funs)), ann: (),
                body: Box::new(res)
                }
            }
            res
        }
        Exp::If { cond, thn, els, ann } => {
            let s_e1 = sequentialize(cond, funs);
            let name1 = format!("#if_cond_{}", ann);
            SeqExp::Let { var: name1.clone(), bound_exp: Box::new(s_e1), ann: (),
                body: Box::new(SeqExp::If { 
                    cond: ImmExp::Var(name1), 
                    thn: Box::new(sequentialize(thn, funs)), 
                    els: Box::new(sequentialize(els, funs)), 
                    ann: () }
                )
            }
        }
        Exp::FunDefs { decls, body, .. } => {
            let mut new_decls = vec![];
            for decl in decls.iter() {
                funs.push((decl.name.clone(), decl.parameters.clone()));
            }
            for decl in decls.iter() {
                new_decls.push(FunDecl{
                    name: decl.name.clone(), 
                    parameters: decl.parameters.clone(), 
                    body: sequentialize(&decl.body, funs), 
                    ann: ()
                });
            }
            let new_body = sequentialize(&body, funs);
            SeqExp::FunDefs { decls: new_decls, body: Box::new(new_body), ann: () }
        }
        Exp::ExternalCall { fun, args, is_tail, ann } => {
            match fun {
                VarOrLabel::Label(name) => {
                    let mut seqs_arg = vec![];
                    let mut names_arg = vec![];
                    let mut arg_count = 0;
                    let mut expected_args = vec![];
                    for fun in funs.clone(){
                        if fun.0==*name {
                            expected_args = fun.1.clone();
                        }
                    }
                    println!("name: {} expected_args: {:?} args: {:?}", name, expected_args, args);
                    if expected_args.len() > args.len(){
                        for arg in 0..(expected_args.len()-args.len()){
                            let s_arg = sequentialize(&Exp::Var(expected_args.get(arg).unwrap().clone(), *ann), funs);
                            let name_arg = format!("#arg{}_{}", ann, arg_count);
                            arg_count += 1;
                            seqs_arg.push(s_arg);
                            names_arg.push(name_arg);
                        }    
                    }
                    for arg in args{
                        let s_arg = sequentialize(arg, funs);
                        let name_arg = format!("#arg{}_{}", ann, arg_count);
                        arg_count += 1;
                        seqs_arg.push(s_arg);
                        names_arg.push(name_arg);
                    }
                    let mut imm_args = vec![];
                    for name in names_arg.iter(){
                        imm_args.push(ImmExp::Var(name.clone()));
                    }
                    let mut res = SeqExp::ExternalCall{fun: fun.clone(), args: imm_args, is_tail: *is_tail, ann: ()};
                    for i in (0..expected_args.len()).rev(){
                        res = SeqExp::Let { var: names_arg.get(i).unwrap().clone(), bound_exp: Box::new(seqs_arg.get(i).unwrap().clone()), ann: (),
                        body: Box::new(res)
                        }
                    }
                    println!("res: {:?}", res);
                    res
                }
                VarOrLabel::Var(_) => {
                    let mut seqs_arg = vec![];
                    let mut names_arg = vec![];
                    let mut arg_count = 0;
                    for arg in args{
                        let s_arg = sequentialize(arg, funs);
                        let name_arg = format!("#VarArg{}_{}", ann, arg_count);
                        arg_count += 1;
                        seqs_arg.push(s_arg);
                        names_arg.push(name_arg);
                    }
                    let mut imm_args = vec![];
                    for name in names_arg.iter(){
                        imm_args.push(ImmExp::Var(name.clone()));
                    }
                    let mut res = SeqExp::ExternalCall{fun: fun.clone(), args: imm_args, is_tail: *is_tail, ann: ()};
                    for i in (0..args.len()).rev(){
                        res = SeqExp::Let { var: names_arg.get(i).unwrap().clone(), bound_exp: Box::new(seqs_arg.get(i).unwrap().clone()), ann: (),
                        body: Box::new(res)
                        }
                    }
                    res
                
                }
            }
        }
        Exp::InternalTailCall(name, args, ann) => {
            let mut seqs_arg = vec![];
            let mut names_arg = vec![];
            let mut arg_count = 0;
            let mut expected_args = vec![];
            for fun in funs.clone(){
                if fun.0==*name {
                    expected_args = fun.1.clone();
                }
            }
            if expected_args.len() > args.len(){
                for arg in 0..(expected_args.len()-args.len()){
                    let s_arg = sequentialize(&Exp::Var(expected_args.get(arg).unwrap().clone(), *ann), funs);
                    let name_arg = format!("#arg{}_{}", ann, arg_count);
                    arg_count += 1;
                    seqs_arg.push(s_arg);
                    names_arg.push(name_arg);
                }
            }
            for arg in args{
                let s_arg = sequentialize(arg, funs);
                let name_arg = format!("#arg{}_{}", ann, arg_count);
                arg_count += 1;
                seqs_arg.push(s_arg);
                names_arg.push(name_arg);
            }
            let mut imm_args = vec![];
            for name in names_arg.iter(){
                imm_args.push(ImmExp::Var(name.clone()));
            }
            let mut res = SeqExp::InternalTailCall(name.clone(), imm_args, ());
            for i in (0..expected_args.len()).rev(){
                res = SeqExp::Let { var: names_arg.get(i).unwrap().clone(), bound_exp: Box::new(seqs_arg.get(i).unwrap().clone()), ann: (),
                body: Box::new(res)
                }
            }
            res
        }

        Exp::MakeClosure { arity, label, env, ann } => {
            let s_env = sequentialize(env, funs);
            let name_env = format!("#env_{}", ann);
            SeqExp::Let { var: name_env.clone(), bound_exp: Box::new(s_env), ann: (),
                body: Box::new(SeqExp::MakeClosure{arity: *arity, label: label.clone(), env: ImmExp::Var(name_env), ann: ()})
            }
        }
        Exp::DirectCall(..) | Exp::Lambda{..} | Exp::ClosureCall(..) | Exp::Call(..) | Exp::Semicolon { .. } => {
            panic!("sugar forms")
        }
    }
}
//fn imm_to_arg64(i: &ImmExp, env: &Vec<(String, i32)>) -> Arg64 {
//    match i {
//        ImmExp::Num(n) => Arg64::Signed(TryFrom::try_from(*n << 1).unwrap()),
//        ImmExp::Bool(b) => if *b {
//            Arg64::Unsigned(SNAKE_TRU)
//        }else{
//            Arg64::Unsigned(SNAKE_FLS)
//        }
//        ImmExp::Var(name) => {
//            let offset = match env_get::<u32>(env, name) {
//                Some(n) => n,
//                None => panic!("{} not found", name)
//            };
//            Arg64::Mem(MemRef{reg:Reg::Rsp, offset:Offset::Constant(offset)})
//        }
//    }
//}
fn space_needed(env: &Vec<(String, i32)>) -> i32 {
    let res = usize_to_i32(env.len());
    if res % 2 == 0 {
        res*8
    }else{
        res*8+8
    }
}
fn snake_space_needed(env: &Vec<(String, i32)>) -> i32 {
    let res = usize_to_i32(env.len());
    if res % 2 == 0 {
        res*8
    }else{
        res*8
    }
}
fn check_ari_num() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Test(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
    is.push(Instr::Jnz(JmpArg::Label(format!("error_ari_not_number"))));
    is
}
fn check_com_num() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Test(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
    is.push(Instr::Jnz(JmpArg::Label(format!("error_com_not_number"))));
    is
}
fn check_index_num() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Test(BinArgs::ToReg(Reg::R14, Arg32::Unsigned(1))));
    is.push(Instr::Jnz(JmpArg::Label(format!("error_index_not_number"))));
    is
}
fn check_index_into_array() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
    is.push(Instr::Xor(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(1))));
    is.push(Instr::Test(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(7))));
    is.push(Instr::Jnz(JmpArg::Label(format!("error_index_into_nonarray"))));
    is
}
fn check_length_into_array() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
    is.push(Instr::Xor(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(1))));
    is.push(Instr::Test(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(7))));
    is.push(Instr::Jnz(JmpArg::Label(format!("error_length_into_nonarray"))));
    is
}
fn check_index_bound() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Sar(BinArgs::ToReg(Reg::R14, Arg32::Unsigned(1))));
    is.push(Instr::Cmp(BinArgs::ToReg(Reg::R14, Arg32::Signed(0))));
    is.push(Instr::Jl(JmpArg::Label(format!("error_index_out_of_bound"))));
    is.push(Instr::Cmp(BinArgs::ToReg(Reg::R14, Arg32::Mem(MemRef { reg: Reg::Rax, offset: Offset::Constant(0) }))));
    is.push(Instr::Jge(JmpArg::Label(format!("error_index_out_of_bound"))));
    is
}
fn check_if_bool() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
    is.push(Instr::Xor(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(7))));
    is.push(Instr::Test(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(7))));
    is.push(Instr::Jnz(JmpArg::Label(format!("error_if_not_boolean"))));
    is
}
fn check_logic_bool() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
    is.push(Instr::Xor(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(7))));
    is.push(Instr::Test(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(7))));
    is.push(Instr::Jnz(JmpArg::Label(format!("error_logic_not_boolean"))));
    is
}
fn check_overflow() -> Vec<Instr> {
    let mut is = vec![];
    is.push(Instr::Jo(JmpArg::Label(format!("error_overflow"))));
    is
}
fn compile_prim(op: &Prim) -> Vec<Instr>{
    match op {
        Prim::Add1 => vec![Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Signed(2)))],
        Prim::Sub1 => vec![Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(2)))],
        Prim::Add => vec![Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10)))],
        Prim::Sub => vec![Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10)))],
        Prim::Mul => vec![Instr::IMul(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10)))],
        Prim::And => vec![Instr::And(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10)))],
        Prim::Or => vec![Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10)))],
        Prim::Gt | Prim::Le => vec![Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10)))],
        Prim::Lt | Prim::Ge => vec![Instr::Sub(BinArgs::ToReg(Reg::R10, Arg32::Reg(Reg::Rax))), rax_equal_r10()],
        _ => panic!("compile_prim1 called with non-unary prim"),
    }
}
fn rax_equal_r10()-> Instr{
    Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R10)))
}
fn need_flip(op: &Prim)->Vec<Instr>{
    let mut is = vec![];
    match op{
        Prim::IsNum | Prim::Ge | Prim::Le | Prim::Neq => {
            is.append(&mut rax_flip());
            is
        }
        _ => is
    }
}
fn rax_flip()->Vec<Instr>{
    let mut is = vec![];
    is.push(Instr::Comment("flip".to_owned()));
    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(FLIP_MASK))));
    is.push(Instr::Xor(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
    is

}
fn print_imm_exp(e: &ImmExp<>) -> () {
    match e {
        ImmExp::Num(n) => print!("{}", n),
        ImmExp::Bool(b) => print!("{}", b),
        ImmExp::Var(name) => print!("{}", name),
    }
}
fn op_to_string (op: &Prim) -> String {
    match op {
        Prim::Add1 => "+1".to_owned(),
        Prim::Sub1 => "-1".to_owned(),
        Prim::Add => "+".to_owned(),
        Prim::Sub => "-".to_owned(),
        Prim::Mul => "*".to_owned(),
        Prim::And => "and".to_owned(),
        Prim::Or => "or".to_owned(),
        Prim::Not => "not".to_owned(),
        Prim::Gt => ">".to_owned(),
        Prim::Lt => "<".to_owned(),
        Prim::Ge => ">=".to_owned(),
        Prim::Le => "<=".to_owned(),
        Prim::Eq => "==".to_owned(),
        Prim::Neq => "!=".to_owned(),
        Prim::IsNum => "number?".to_owned(),
        Prim::IsBool => "boolean?".to_owned(),
        Prim::Print => "print".to_owned(),
        Prim::IsFun => "function?".to_owned(),
        Prim::IsArray => "array?".to_owned(),
        Prim::ArrayGet => "array-get".to_owned(),
        Prim::ArraySet => "array-set".to_owned(),
        Prim::Length => "length".to_owned(),
        Prim::GetCode => "get-code".to_owned(),
        Prim::GetEnv => "get-env".to_owned(),
        Prim::CheckArityAndUntag(_) => "check-arity-and-untag".to_owned(),
        Prim::MakeArray => "make-array".to_owned(),
    }
}
pub fn print_seq_exp<Ann>(e: &SeqExp<Ann>) -> () {
    match e {
        SeqExp::Imm(imm_exp, _) => print_imm_exp(imm_exp),
        SeqExp::Prim(op, exp, _) =>{
            print!("({}", op_to_string(op));
            for e in exp{
                print!(" ");
                print_imm_exp(e);
            }
            print!(")");
        }
        SeqExp::Let {var, bound_exp, body, ..} => {
            print!("(let ([{} ", var);
            print_seq_exp(bound_exp);
            print!("]) in \n");
            print_seq_exp(body);
            print!(")");
        }
        SeqExp::If {cond, thn, els, ..} =>{
            print!("(if ");
            print_imm_exp(cond);
            print!(" ");
            print_seq_exp(thn);
            print!(" ");
            print_seq_exp(els);
            print!(")");
        }
        SeqExp::FunDefs { decls, body, .. } => {
            print!("(letrec (");
            for decl in decls{
                print!("({} (", decl.name);
                for arg in &decl.parameters{
                    print!("{} ", arg);
                }
                print!(") -> ");
                print_seq_exp(&decl.body);
                print!(") ");
            }
            print!(") in \n");
            print_seq_exp(body);
            print!(")");
        }
        SeqExp::ExternalCall { fun, args, .. } => {
            print!("(external-call {:?} ", fun);
            for arg in args{
                print_imm_exp(arg);
                print!(" ");
            }
            print!(")");
        }
        SeqExp::InternalTailCall(name, args, ..) => {
            print!("(tail-call {} ", name);
            for arg in args{
                print_imm_exp(arg);
                print!(" ");
            }
            print!(")");
        }
        SeqExp::MakeClosure { arity, label, env, .. } => {
            print!("(make-closure {} {} {:?}", arity, label, print_imm_exp(env));
        }
        SeqExp::Semicolon { e1, e2, .. } => {
            print_seq_exp(e1);
            print!(";\n");
            print_seq_exp(e2);
        }
    }
}
fn compile_to_instrs_helper(e: &SeqExp<u32>, env: &mut Vec<(String, i32)>, is_def: bool, last: &mut Vec<Instr>) -> Vec<Instr>{
    let mut is = vec![];
    match e {
        SeqExp::Imm(imm_exp, _) => {
            let arg64 = {
                match imm_exp {
                    ImmExp::Num(n) => Arg64::Signed(TryFrom::try_from(*n << 1).unwrap()),
                    ImmExp::Bool(b) => if *b {
                        Arg64::Unsigned(SNAKE_TRU)
                    }else{
                        Arg64::Unsigned(SNAKE_FLS)
                    }
                    ImmExp::Var(name) => {
                        let offset = match env_get::<u32>(env, name) {
                            Some(n) => n,
                            None => {
                                println!("e: {:?}", e);
                                panic!("{} not found", name)
                            }
                        };
                        Arg64::Mem(MemRef{reg:Reg::Rsp, offset:Offset::Constant(offset)})
                    }
                }
            };
            is.extend(vec![Instr::Mov(MovArgs::ToReg(Reg::Rax, arg64))])
        },
        SeqExp::Prim(op, exp, ann) =>{
            match op {
                Prim::Add1 | Prim::Sub1 => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("add1_sub1".to_owned()));
                    is.append(&mut check_ari_num());
                    is.append(&mut compile_prim(op));
                    is.append(&mut check_overflow());
                }
                Prim::Add | Prim::Sub | Prim::Mul => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("add_sub_mul".to_owned()));
                    let sndimm = exp.get(1).unwrap();
                    is.append(&mut check_ari_num());
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Sar(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(1))));
                    let arg64 = {
                        match sndimm {
                            ImmExp::Num(n) => Arg64::Signed(TryFrom::try_from(*n << 1).unwrap()),
                            ImmExp::Bool(b) => if *b {
                                Arg64::Unsigned(SNAKE_TRU)
                            }else{
                                Arg64::Unsigned(SNAKE_FLS)
                            }
                            ImmExp::Var(name) => {
                                let offset = match env_get::<u32>(env, name) {
                                    Some(n) => n,
                                    None => {
                                        println!("e: {:?}", e);
                                        panic!("{} not found", name)
                                    }
                                };
                                Arg64::Mem(MemRef{reg:Reg::Rsp, offset:Offset::Constant(offset)})
                            }
                        }
                    };
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, arg64)));
                    is.append(&mut check_ari_num());
                    is.push(Instr::Sar(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R11, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::R11))));
                    is.append(&mut compile_prim(op));
                    is.append(&mut check_overflow());
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.append(&mut check_overflow());
                }
                Prim::Not => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("not".to_owned()));
                    is.append(&mut check_logic_bool());
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(BOOL_MASK))));
                    is.push(Instr::Xor(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                }
                Prim::And | Prim::Or => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("and_or".to_owned()));
                    let sndimm = exp.get(1).unwrap();
                    is.append(&mut check_logic_bool());
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    let arg64 = {
                        match sndimm {
                            ImmExp::Num(n) => Arg64::Signed(TryFrom::try_from(*n << 1).unwrap()),
                            ImmExp::Bool(b) => if *b {
                                Arg64::Unsigned(SNAKE_TRU)
                            }else{
                                Arg64::Unsigned(SNAKE_FLS)
                            }
                            ImmExp::Var(name) => {
                                let offset = match env_get::<u32>(env, name) {
                                    Some(n) => n,
                                    None => {
                                        println!("e: {:?}", e);
                                        panic!("{} not found", name)
                                    }
                                };
                                Arg64::Mem(MemRef{reg:Reg::Rsp, offset:Offset::Constant(offset)})
                            }
                        }
                    };
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, arg64)));
                    is.append(&mut check_logic_bool());
                    is.append(&mut compile_prim(op));
                }
                Prim::Eq | Prim::Neq => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("eq or neq".to_owned()));
                    let sndimm = exp.get(1).unwrap();
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    let arg64 = {
                        match sndimm {
                            ImmExp::Num(n) => Arg64::Signed(TryFrom::try_from(*n << 1).unwrap()),
                            ImmExp::Bool(b) => if *b {
                                Arg64::Unsigned(SNAKE_TRU)
                            }else{
                                Arg64::Unsigned(SNAKE_FLS)
                            }
                            ImmExp::Var(name) => {
                                let offset = match env_get::<u32>(env, name) {
                                    Some(n) => n,
                                    None => {
                                        println!("e: {:?}", e);
                                        panic!("{} not found", name)
                                    }
                                };
                                Arg64::Mem(MemRef{reg:Reg::Rsp, offset:Offset::Constant(offset)})
                            }
                        }
                    };
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, arg64)));
                    is.push(Instr::Cmp(BinArgs::ToReg(Reg::R10, Arg32::Reg(Reg::Rax))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Unsigned(SNAKE_TRU))));
                    is.push(Instr::Jz(JmpArg::Label(format!("eq_done{}", ann))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Unsigned(SNAKE_FLS))));
                    is.push(Instr::Label(format!("eq_done{}", ann)));
                    is.append(&mut need_flip(op));
                }
                Prim::Print => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("print".to_owned()));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rdi, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Sub(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(space_needed(env)))));
                    is.push(Instr::Call(JmpArg::Label("print_snake_val".to_owned())));
                    is.push(Instr::Add(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(space_needed(env)))));
                }
                Prim::IsBool => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("isbool".to_owned()));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(63))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
                    //r9 = ...xx1
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(62))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::And(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::Rax))));
                    //r9 = ...x11
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(61))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::And(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::Rax))));
                    //r9 = ...111
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(TAG_MASK))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R9))));
                }
                Prim::IsNum => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("isnum".to_owned()));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(63))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R8, Arg64::Unsigned(BOOL_MASK))));
                    is.push(Instr::Xor(BinArgs::ToReg(Reg::R10, Arg32::Reg(Reg::R8))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(TAG_MASK))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                }
                Prim::IsArray => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("isarray".to_owned()));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(63))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
                    //r9 = ...xx1
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(62))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R8, Arg64::Unsigned(BOOL_MASK))));
                    is.push(Instr::Xor(BinArgs::ToReg(Reg::R10, Arg32::Reg(Reg::R8))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::And(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::Rax))));
                    //r9 = ...x11
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(61))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R8, Arg64::Unsigned(BOOL_MASK))));
                    is.push(Instr::Xor(BinArgs::ToReg(Reg::R10, Arg32::Reg(Reg::R8))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::And(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::Rax))));
                    //r9 = ...111
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(TAG_MASK))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R9))));
                }
                Prim::IsFun => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("isfun".to_owned()));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(63))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
                    //r9 = ...xx1
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(62))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::And(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::Rax))));
                    //r9 = ...x11
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(61))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R8, Arg64::Unsigned(BOOL_MASK))));
                    is.push(Instr::Xor(BinArgs::ToReg(Reg::R10, Arg32::Reg(Reg::R8))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Shr(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
                    is.push(Instr::And(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::Rax))));
                    //r9 = ...111
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(TAG_MASK))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::R10))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R9))));
                }
                Prim::Gt | Prim::Lt | Prim::Ge | Prim::Le => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Comment("gt_lt_ge_le".to_owned()));
                    let sndimm = exp.get(1).unwrap();
                    is.append(&mut check_com_num());
                    is.push(Instr::Sar(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Reg(Reg::Rax))));
                    let arg64 = {
                        match sndimm {
                            ImmExp::Num(n) => Arg64::Signed(TryFrom::try_from(*n << 1).unwrap()),
                            ImmExp::Bool(b) => if *b {
                                Arg64::Unsigned(SNAKE_TRU)
                            }else{
                                Arg64::Unsigned(SNAKE_FLS)
                            }
                            ImmExp::Var(name) => {
                                let offset = match env_get::<u32>(env, name) {
                                    Some(n) => n,
                                    None => {
                                        println!("e: {:?}", e);
                                        panic!("{} not found", name)
                                    }
                                };
                                Arg64::Mem(MemRef{reg:Reg::Rsp, offset:Offset::Constant(offset)})
                            }
                        }
                    };
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, arg64)));
                    is.append(&mut check_com_num());
                    is.push(Instr::Sar(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))));
                    is.append(&mut compile_prim(op));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R11, Arg64::Unsigned(BOOL_MASK))));
                    is.push(Instr::And(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R11))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R11, Arg64::Unsigned(TAG_MASK))));
                    is.push(Instr::Or(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R11))));
                    is.append(&mut need_flip(op));
                }
                Prim::ArrayGet => {
                    is.push(Instr::Comment("array_get".to_owned()));
                    let sndimm = exp.get(1).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(sndimm.clone(), *ann), env, false, last));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R14, Arg64::Reg(Reg::Rax))));
                    is.append(&mut check_index_num());
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.extend(check_index_into_array());
                    is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(1))));
                    is.extend(check_index_bound());
                    //is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::R14))));
                    //is.push(Instr::Shl(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(3))));
                    //is.push(Instr::Add(BinArgs::ToReg(Reg::R9, Arg32::Reg(Reg::Rax))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Mem(MemRef{reg:Reg::Rax, 
                        offset:Offset::Computed { reg: Reg::R14, factor: 8, constant: 8 }}))));
                }
                Prim::ArraySet => {
                    is.push(Instr::Comment("array_set".to_owned()));
                    let sndimm = exp.get(1).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(sndimm.clone(), *ann), env, false, last));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R14, Arg64::Reg(Reg::Rax))));
                    is.append(&mut check_index_num());
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.extend(check_index_into_array());
                    is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(1))));
                    is.extend(check_index_bound());
                    //is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::R14))));
                    //is.push(Instr::Shl(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(3))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
                    let thirdimm = exp.get(2).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(thirdimm.clone(), *ann), env, false, last));
                    is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg:Reg::R9, 
                        offset:Offset::Computed { reg: Reg::R14, factor: 8, constant: 8 }}, Reg32::Reg(Reg::Rax))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R9))));
                    is.push(Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Signed(1))));
                }
                Prim::MakeArray => {
                    is.push(Instr::Comment("make_array".to_owned()));
                    is.push(Instr::Mov(MovArgs::ToMem(MemRef { reg: Reg::R15, offset: Offset::Constant(0) }
                    , Reg32::Signed(exp.len() as i32))));
                    let mut i = 0;
                    for element in exp {
                        i = i + 1;
                        is.extend(compile_to_instrs_helper(&SeqExp::Imm(element.clone(), *ann), env, false, last));
                        is.push(Instr::Mov(MovArgs::ToMem(MemRef { reg: Reg::R15, offset: Offset::Constant(i*8) }
                        , Reg32::Reg(Reg::Rax))));
                    }
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R15))));
                    is.push(Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Signed(1))));
                    is.push(Instr::Add(BinArgs::ToReg(Reg::R15, Arg32::Signed(8*(i+1)))));
                }
                Prim::Length => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.extend(check_length_into_array());
                    is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(1))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Mem(MemRef{reg:Reg::Rax, 
                        offset:Offset::Constant(0)}))));
                    is.push(Instr::Shl(BinArgs::ToReg(Reg::Rax, Arg32::Unsigned(1))))
                }
                Prim::CheckArityAndUntag(count) => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R9, Arg64::Reg(Reg::Rax))));
                    is.push(Instr::Xor(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(3))));
                    is.push(Instr::Test(BinArgs::ToReg(Reg::R9, Arg32::Unsigned(7))));
                    is.push(Instr::Jnz(JmpArg::Label(format!("error_not_closure"))));                
                    is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(3))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Mem(MemRef{reg:Reg::Rax, 
                        offset:Offset::Constant(0)}))));
                    is.push(Instr::Cmp(BinArgs::ToReg(Reg::R10, Arg32::Unsigned(TryFrom::try_from(*count).unwrap()))));
                    is.push(Instr::Jne(JmpArg::Label(format!("error_wrong_arity"))));
                    is.push(Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Signed(3))));
                }
                Prim::GetCode => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(3))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Mem(MemRef{reg:Reg::Rax, 
                        offset:Offset::Constant(8)}))));
                    }
                Prim::GetEnv => {
                    let fstimm = exp.get(0).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(fstimm.clone(), *ann), env, false, last));
                    is.push(Instr::Sub(BinArgs::ToReg(Reg::Rax, Arg32::Signed(3))));
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Mem(MemRef{reg:Reg::Rax, 
                        offset:Offset::Constant(16)}))));
                }
            }
        },
        SeqExp::Let { var, bound_exp, body, .. } => {
            is.push(Instr::Comment("let_def".to_owned()));
            is.append(&mut compile_to_instrs_helper(bound_exp, env, false, last));
            let offset = usize_to_i32(env.len());
            env.push((String::from(var), -8*(offset+1)));
            is.push(Instr::Mov(
                MovArgs::ToMem(
                    MemRef{reg:Reg::Rsp, offset:Offset::Constant(-8*(offset+1))},
                    Reg32::Reg(Reg::Rax)
                )
            ));
            is.push(Instr::Comment("let_def_end".to_owned()));
            is.push(Instr::Comment("let_body".to_owned()));
            is.append(&mut compile_to_instrs_helper(body, env, false, last));
            is.push(Instr::Comment("let_body_end".to_owned()));

        },
        SeqExp::If { cond, thn, els, ann } => {
            let else_lab = format!("if_false{}", ann);
            let done_lab = format!("done{}", ann);
            is.extend(compile_to_instrs_helper(&SeqExp::Imm(cond.clone(), *ann), env, false, last));
            is.append(&mut check_if_bool());
            is.push(Instr::Comment("cond".to_owned()));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::R10, Arg64::Unsigned(SNAKE_FLS))));
            is.push(Instr::Cmp(BinArgs::ToReg(Reg::Rax, Arg32::Reg(Reg::R10))));
            is.push(Instr::Je(JmpArg::Label(else_lab.clone())));
            is.extend(compile_to_instrs_helper(thn, env, false, last));
            is.push(Instr::Jmp(JmpArg::Label(done_lab.clone())));
            is.push(Instr::Label(else_lab.clone()));
            is.extend(compile_to_instrs_helper(els, env, false, last));
            is.push(Instr::Label(done_lab));
        }
        SeqExp::FunDefs { decls, body, .. } => {
            is.push(Instr::Comment("fun_body".to_owned()));
            is.extend(compile_to_instrs_helper(body, env, false, last));
            is.push(Instr::Comment("fun_body_end".to_owned()));
            if is_def{
                is.push(Instr::Ret);
            }
            for decl in decls {
                let mut last2 = vec![];
                last.push(Instr::Label(decl.name.clone()));
                let mut fun_env = vec![];
                let mut offset = 0;
                for arg in &decl.parameters {
                    offset = offset - 1;
                    fun_env.push((arg.clone(), offset*8));
                    println!("{} {}", arg, offset*8)
                }
                last.extend(compile_to_instrs_helper(&decl.body, &mut fun_env, false, &mut last2));
                last.push(Instr::Ret);
                last.extend(last2);
            }
        }
        SeqExp::InternalTailCall(name, args, ann) => {
            for arg in 0..args.len(){
                let name_of_arg = args.get(arg).unwrap();
                is.extend(compile_to_instrs_helper(&SeqExp::Imm(name_of_arg.clone(), *ann), env, false, last));
                is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg:Reg::Rsp, offset: Offset::Constant(-8*(arg+1) as i32)}, Reg32::Reg(Reg::Rax))));
            }
            is.push(Instr::Jmp(JmpArg::Label(name.clone())));
        }
        SeqExp::ExternalCall { fun, args, is_tail, ann } => {
            match fun {
                VarOrLabel::Label(_) => (),
                VarOrLabel::Var(name) => {
                    let offset = match env_get::<u32>(env, name) {
                        Some(n) => n,
                        None => panic!("{} not found", name)
                    };
                    let rsp = Arg64::Mem(MemRef{reg:Reg::Rsp, offset:Offset::Constant(offset)});
                    is.push(Instr::Mov(MovArgs::ToReg(Reg::R8, rsp)));
                }
            };
            if *is_tail{
                for arg in 0..args.len(){
                    let name_of_arg = args.get(arg).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(name_of_arg.clone(), *ann), env, false, last));
                    is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg:Reg::Rsp, offset: Offset::Constant(-8*(arg+1) as i32)}, Reg32::Reg(Reg::Rax))));
                }
                match fun {
                    VarOrLabel::Label(name) => {
                        is.push(Instr::Jmp(JmpArg::Label(name.clone())));
                    }
                    VarOrLabel::Var(_) => {
                        is.push(Instr::Jmp(JmpArg::Reg(Reg::R8)));
                    }
                }
            }else{
                for arg in 0..args.len(){
                    let name_of_arg = args.get(arg).unwrap();
                    is.extend(compile_to_instrs_helper(&SeqExp::Imm(name_of_arg.clone(), *ann), env, false, last));
                    is.push(Instr::Mov(MovArgs::ToMem(MemRef{reg:Reg::Rsp, offset: Offset::Constant(-8*(arg+2+env.len()) as i32)}, Reg32::Reg(Reg::Rax))));
                }
                is.push(Instr::Sub(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(snake_space_needed(env)))));
                match fun {
                    VarOrLabel::Label(name) => {
                        is.push(Instr::Call(JmpArg::Label(name.clone())));
                    }
                    VarOrLabel::Var(_) => {
                        is.push(Instr::Call(JmpArg::Reg(Reg::R8)));
                    }
                }
                is.push(Instr::Add(BinArgs::ToReg(Reg::Rsp, Arg32::Signed(snake_space_needed(env)))));
            }        
}
        SeqExp::MakeClosure { arity, label, env:clo_env, ann } => {
            is.push(Instr::Comment("make_closure".to_owned()));
            is.push(Instr::Mov(MovArgs::ToMem(MemRef { reg: Reg::R15, offset: Offset::Constant(0) }, Reg32::Unsigned(TryFrom::try_from(*arity).unwrap()))));
            is.push(Instr::RelativeLoadAddress(Reg::R9, label.clone()));
            is.push(Instr::Mov(MovArgs::ToMem(MemRef { reg: Reg::R15, offset: Offset::Constant(8) }, Reg32::Reg(Reg::R9))));
            is.extend(compile_to_instrs_helper(&SeqExp::Imm(clo_env.clone(), *ann), env, false, last));
            is.push(Instr::Mov(MovArgs::ToMem(MemRef { reg: Reg::R15, offset: Offset::Constant(16) }, Reg32::Reg(Reg::Rax))));
            is.push(Instr::Mov(MovArgs::ToReg(Reg::Rax, Arg64::Reg(Reg::R15))));
            is.push(Instr::Add(BinArgs::ToReg(Reg::Rax, Arg32::Signed(3))));
            is.push(Instr::Add(BinArgs::ToReg(Reg::R15, Arg32::Signed(24))));
        }
        SeqExp::Semicolon { .. } => {
            panic!("compile semicolon")
        }
    }
    is
}
pub fn compile_to_instrs(e: &SeqProg<u32>) -> Vec<Instr> {
    let mut res = vec![];
    for decl in &e.funs {
        res.push(Instr::Label(decl.name.clone()));
        let mut env = vec![];
        let mut offset = 1;
        for arg in &decl.parameters {
            env.push((arg.clone(), -8*offset));
            offset = offset + 1;
        }
        let mut last = vec![];
        println!("decl.name: {:?}", decl.name);
        println!("decl.body: {:?}", decl.body);
        res.extend(compile_to_instrs_helper(&decl.body, &mut env, false, &mut last));
        println!("end of decl.body");
        res.push(Instr::Ret);
        res.extend(last);
    }
    res.push(Instr::Label("main".to_string()));
    println!("e.main: {:?}", e.main);
    let mut last = vec![];
    match e.main {
        SeqExp::FunDefs{..} => {
            res.extend(compile_to_instrs_helper(&e.main, &mut vec![], true, &mut last));
        },
        _ => {
            res.extend(compile_to_instrs_helper(&e.main, &mut vec![], false, &mut last));
            res.push(Instr::Ret); 
        }
    }
    res.extend(last);
    res
}
