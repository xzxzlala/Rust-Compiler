use crate::syntax::{Exp, FunDecl, SeqExp, SeqProg, SurfProg, VarOrLabel, Prim};
use crate::asm::instrs_to_string;
//use std::clone;
use std::collections::{HashMap, HashSet};
use crate::helper_function::{tag_exp, check_prog_helper, compile_to_instrs, sequentialize, tag_prog};

#[derive(Debug, PartialEq, Eq)]
pub enum CompileErr<Span> {
    UnboundVariable {
        unbound: String,
        location: Span,
    },
    UndefinedFunction {
        undefined: String,
        location: Span,
    },
    // The Span here is the Span of the let-expression that has the two duplicated bindings
    DuplicateBinding {
        duplicated_name: String,
        location: Span,
    },

    Overflow {
        num: i64,
        location: Span,
    },

    DuplicateFunName {
        duplicated_name: String,
        location: Span, // the location of the 2nd function
    },

    DuplicateArgName {
        duplicated_name: String,
        location: Span,
    },
}

pub fn check_prog<Span>(p: &SurfProg<Span>) -> Result<(), CompileErr<Span>>
where
    Span: Clone,
{
    check_prog_helper(p, vec![])
}

fn check_hash_extend<'a>(map: &'a mut HashMap<String, u32>, str: &String) -> () {
    match map.get(str) {
        Some(_) =>{
            *map.get_mut(str).unwrap()+=1; 
        }
        None => {
            map.insert(str.clone(), 0);
        }
    }
}
fn uniquify_helper(e: &Exp<u32>, map: &mut HashMap<String, u32>) -> Exp<()>{
    match e {
        Exp::Num(n ,_)  =>  Exp::Num(*n, ()),
        Exp::Bool(b, _) =>  Exp::Bool(*b, ()),
        Exp::Float(f, _) => Exp::Float(*f, ()),
        Exp::Var(name, _) => {
            match map.get(name) {
                Some(n) => Exp::Var(format!("{}{}", name, n), ()),
                None => panic!("unbound var in uniquify"),
            }
        }
        Exp::Prim(p, args, _) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(Box::new(uniquify_helper(arg, map)));
            }
            Exp::Prim(*p, new_args, ())
        }
        Exp::Let { bindings, body, ..} => {
            let mut new_bindings = vec![];
            for (name, exp) in bindings {
                let new_exp = uniquify_helper(exp, map);
                check_hash_extend(map, name);
                let name_str = match map.get(name) {
                    Some(n) => format!("{}{}", name, n),
                    None => panic!("unbound var in uniquify"),
                };
                new_bindings.push((name_str, new_exp));
            }
            let new_body = uniquify_helper(body, map);
            Exp::Let {bindings: new_bindings, body: Box::new(new_body), ann: ()}
        }
        Exp::If {cond, thn, els, ..} => {
            let new_cond = uniquify_helper(cond, map);
            let new_then = uniquify_helper(thn, map);
            let new_els = uniquify_helper(els, map);
            Exp::If {cond: Box::new(new_cond), thn: Box::new(new_then), els: Box::new(new_els), ann: ()}
        }
        Exp::FunDefs {decls, body, ..} => {
            let mut new_decls = vec![];
            let mut body_map = map.clone();
            for decl in decls {
                check_hash_extend(map, &decl.name);
            }
            for decl in decls {
                check_hash_extend(&mut body_map, &decl.name);
                for param in &decl.parameters {
                    check_hash_extend(map, param);
                }
                let mut new_para = vec![];
                for para in decl.parameters.iter(){
                    let para_new_name = map.get(para).unwrap();
                    new_para.push(format!("{}{}", para, para_new_name));
                }
                let new_body = uniquify_helper(&decl.body, map);
                //println!("map: {:?}", map);
                //println!("map_after: {:?}", map);
                let decl_new_name = map.get(&decl.name).unwrap();
                new_decls.push(FunDecl {name: format!("{}{}",decl.name, decl_new_name), parameters: new_para, body: new_body, ann: ()});
            }
            let new_body = uniquify_helper(body, &mut body_map);
            Exp::FunDefs {decls: new_decls, body: Box::new(new_body), ann: ()}
        }
        Exp::Call(name, args, _) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(uniquify_helper(arg, map));
            }
            Exp::Call(Box::new(uniquify_helper(&name, map)), new_args, ())
        }
        Exp::Lambda { parameters, body, .. } => {
            let mut new_para = vec![];
            for para in parameters.iter(){
                check_hash_extend(map, para);
                let para_new_name = map.get(para).unwrap();
                new_para.push(format!("{}{}", para, para_new_name));
            }
            Exp::Lambda { parameters: new_para, body: Box::new(uniquify_helper(body, map)), ann: () }
        }
        Exp::Semicolon {..} | Exp::DirectCall(..) | Exp::ClosureCall(..) | Exp::MakeClosure { .. } | 
        Exp::ExternalCall { .. } | Exp::InternalTailCall(..) => panic!("internal exp uniquify"),
    }
}
fn uniquify(e: &Exp<u32>) -> Exp<()> {
    let mut map = HashMap::new();
    uniquify_helper(e, &mut map)
}

fn eliminate_closures_helper<Ann>(e: &Exp<Ann>, fun: &mut HashSet<String>) -> Exp<()> {
    match e {
        Exp::Bool(b, _) => Exp::Bool(*b, ()),
        Exp::Num(n, _) => Exp::Num(*n, ()),
        Exp::Float(f, _) => Exp::Float(*f, ()),
        Exp::Var(name, _) => Exp::Var(name.clone(), ()),
        Exp::Prim(p, args, _) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(Box::new(eliminate_closures_helper(arg, fun)));
            }
            Exp::Prim(*p, new_args, ())
        }
        Exp::Let { bindings, body, ..} => {
            let mut new_bindings = vec![];
            for (name, exp) in bindings {
                let new_exp = eliminate_closures_helper(exp, fun);
                new_bindings.push((name.clone(), new_exp));
            }
            let new_body = eliminate_closures_helper(body, fun);
            Exp::Let { bindings: new_bindings, body: Box::new(new_body), ann: () }
        }
        Exp::If {cond, thn, els, ..} => {
            let new_cond = eliminate_closures_helper(cond, fun);
            let new_then = eliminate_closures_helper(thn, fun);
            let new_else = eliminate_closures_helper(els, fun);
            Exp::If {cond: Box::new(new_cond), thn: Box::new(new_then), els: Box::new(new_else), ann: ()}
        }
        Exp::FunDefs {decls, body, ..} => {
            let mut new_decls = vec![];
            for decl in decls {
                fun.insert(decl.name.clone());
            }
            for decl in decls {
                let new_body = eliminate_closures_helper(&decl.body, fun);
                new_decls.push(FunDecl {name: decl.name.clone(), parameters: decl.parameters.clone(), body: new_body, ann: ()});
            }
            let new_body = eliminate_closures_helper(body, fun);
            Exp::FunDefs {decls: new_decls, body: Box::new(new_body), ann: ()}
        }
        Exp::Call(name, args, ..) => {
            match eliminate_closures_helper(&name.clone(), fun) {
                Exp::Var(str, _) => {
                    if fun.contains(&str.clone()) {
                        let mut new_args = vec![];
                        for arg in args{
                            new_args.push(eliminate_closures_helper(arg, fun));
                        }
                        Exp::DirectCall(str.clone(), new_args, ())
                    }else{
                        let mut new_args = vec![];
                        for arg in args{
                            new_args.push(eliminate_closures_helper(arg, fun));
                        }
                        Exp::ClosureCall(Box::new(eliminate_closures(&name.clone())), new_args, ())
                    }
                }
                _ => {
                    let mut new_args = vec![];
                    for arg in args{
                        new_args.push(eliminate_closures_helper(arg, fun));
                    }
                    Exp::ClosureCall(Box::new(eliminate_closures(&name.clone())), new_args, ())
                }
            }
        }
        Exp::Lambda { parameters, body, .. } => {
            Exp::Lambda { parameters: parameters.clone(), body: Box::new(eliminate_closures_helper(body, fun)), ann: () }
        }
        Exp::Semicolon { .. } | Exp::ClosureCall(..) | Exp::DirectCall(..) | Exp::MakeClosure { .. } | Exp::InternalTailCall(..) | Exp::ExternalCall { .. }=> {
            panic!("internel exp")
        }
    }
}
// Parse Calls into either DirectCall or ClosureCall
fn eliminate_closures<Ann>(e: &Exp<Ann>) -> Exp<()> {
    eliminate_closures_helper(e, &mut HashSet::new())
}

// Identify which functions should be lifted to the top level
fn should_lift_helper(p: &Exp<bool>, set: &mut HashSet<String>, inside_lifted_function: bool) -> () {
    match p {
        Exp::Num(_, _) => (),
        Exp::Bool(_, _) => (),
        Exp::Var(_, _) => (),
        Exp::Float(_, _) => (),
        Exp::Prim(_, args, _) => {
            for arg in args {
                should_lift_helper(arg, set, inside_lifted_function);
            }
        }
        Exp::Let { bindings, body, ann: _} => {
            for (_, exp) in bindings {
                should_lift_helper(exp, set, inside_lifted_function);
            }
            should_lift_helper(body, set, inside_lifted_function);
        }
        Exp::If {cond, thn, els, ann: _} => {
            should_lift_helper(cond, set, inside_lifted_function);
            should_lift_helper(thn, set, inside_lifted_function);
            should_lift_helper(els, set, inside_lifted_function);
        }
        Exp::FunDefs {decls, body, ann: _} => {
            for decl in decls {
                if set.contains(&decl.name) {
                    should_lift_helper(&decl.body, set, true);
                }else{
                    should_lift_helper(&decl.body, set, inside_lifted_function);
                }
            }
            should_lift_helper(body, set, inside_lifted_function);
        }
        Exp::DirectCall(name, args, is_tail) => {
            if !is_tail{
                set.insert(name.clone());
            }
            if inside_lifted_function {
                set.insert(name.clone());
            }
            for arg in args {
                should_lift_helper(arg, set, inside_lifted_function);
            }
        }
        Exp::ClosureCall(fun, args, _) => {
            should_lift_helper(&fun, set, inside_lifted_function);
            for arg in args {
                should_lift_helper(arg, set, inside_lifted_function);
            }
        }
        Exp::Lambda { parameters: _, body, .. } => {
            should_lift_helper(&body, set, inside_lifted_function);
        }
        Exp::Semicolon {..} | Exp::Call(..) | Exp::MakeClosure { .. } | 
        Exp::ExternalCall { .. } | Exp::InternalTailCall(..) => panic!("internal exp uniquify")
    }
}
// Identify which functions should be lifted to the top level
fn mark_tail<Ann>(p: &Exp<Ann>, is_tail: bool) -> Exp<bool> {
    match p {
        Exp::Num(n, _) => Exp::Num(*n, false),
        Exp::Bool(b, _) => Exp::Bool(*b, false),
        Exp::Float(f, _) => Exp::Float(*f, false),
        Exp::Var(name, _) => Exp::Var(name.clone(), false),
        Exp::Prim(p, args, _) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(Box::new(mark_tail(&arg, false)));
            }
            Exp::Prim(*p, new_args, is_tail)
        }
        Exp::Let { bindings, body, ..} => {
            let mut new_bindings = vec![];
            for (name, exp) in bindings {
                let new_exp = mark_tail(&exp, false);
                new_bindings.push((name.clone(), new_exp));
            }
            Exp::Let {bindings: new_bindings, body: Box::new(mark_tail(&body, is_tail)), ann: is_tail}
        }
        Exp::If {cond, thn, els, ..} => {
            Exp::If {
                cond: Box::new(mark_tail(&cond, false)), 
                thn: Box::new(mark_tail(&thn, is_tail)), 
                els: Box::new(mark_tail(&els, is_tail)), 
                ann: is_tail
            }
        }
        Exp::FunDefs { decls, body, .. } => {
            let mut new_decls = vec![];
            for decl in decls {
                let new_body = mark_tail(&decl.body, true);
                new_decls.push(FunDecl {name: decl.name.clone(), parameters: decl.parameters.clone(), body: new_body, ann: is_tail});
            }
            Exp::FunDefs {decls: new_decls, body: Box::new(mark_tail(&body, is_tail)), ann: is_tail}
        }
        Exp::DirectCall(name, args, _ ) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(mark_tail(&arg, false));
            }
            Exp::DirectCall(name.clone(), new_args, is_tail)
        }
        Exp::ClosureCall(fun, args, _) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(mark_tail(&arg, false));
            }
            Exp::ClosureCall(Box::new(mark_tail(&fun, false)), new_args, is_tail)
        }
        Exp::Lambda { parameters, body, .. } => {
            Exp::Lambda { parameters: parameters.clone(), body: Box::new(mark_tail(&body, is_tail)), ann: is_tail }
        }
        Exp::Semicolon {..} | Exp::Call(..) | Exp::MakeClosure { .. } | 
        Exp::ExternalCall { .. } | Exp::InternalTailCall(..) => panic!("internal exp uniquify")
    }
}
fn should_lift<Ann>(p: &Exp<Ann>) -> HashSet<String> {
    let mut set = HashSet::new();
    let mut new_set = set.clone();
    should_lift_helper(&mark_tail(p, true), &mut new_set, false);
    while new_set != set {
        set = new_set;
        new_set = set.clone();
        should_lift_helper(&mark_tail(p, true), &mut new_set, false);
    }
    set
}

fn body_variables(e: &Exp<bool>, funs : &Vec<(String, Vec<String>)>) -> Vec<String> {
    match e {
        Exp::Num(_, _) => vec![],
        Exp::Bool(_, _) => vec![],
        Exp::Float(_, _) => vec![],
        Exp::Var(name, _) => vec![name.clone()],
        Exp::Prim(_, args, _) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.extend(body_variables(arg, funs));
            }
            new_args
        }
        Exp::Let { bindings, body, ..} => {
            let mut new_bindings = vec![];
            for (name, exp) in bindings {
                new_bindings.push((name.clone(), exp));
            }
            let mut new_body = body_variables(body, funs);
            for (_, exp) in new_bindings {
                new_body.extend(body_variables(exp, funs));
            }
            for (name, _) in bindings{
                if new_body.contains(name) {
                    new_body.remove(new_body.iter().position(|x| x == name).unwrap());
                }
            }
            new_body
        }
        Exp::If {cond, thn, els, ..} => {
            let mut new_cond = body_variables(cond, funs);
            let new_then = body_variables(thn, funs);
            let new_else = body_variables(els, funs);
            new_cond.extend(new_then);
            new_cond.extend(new_else);
            new_cond
        }
        Exp::FunDefs {decls, body, ..} => {
            //also extend all function declares in decls
            let mut body_var = body_variables(body, funs);
            for decl in decls {
                //body_var.extend(decl.parameters.clone());
                for name  in decl.parameters.iter(){
                    if body_var.contains(name) {
                        body_var.remove(body_var.iter().position(|x| x == name).unwrap());
                    }
                }
            }
            body_var
        }
        Exp::DirectCall(name, args, _ ) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.extend(body_variables(arg, funs));
            }

            for (fun_name, fun_para) in funs {
                if fun_name == name {
                    println!("fun_para: {:?}", fun_para);
                    println!("args: {:?}", args);
                    println!("argLen: {}", args.len());
                    for para in 0..(fun_para.len()-args.len()) {
                        if !new_args.contains(fun_para.get(para).unwrap()) {
                            new_args.insert(0, fun_para.get(para).unwrap().clone());
                        }
                    }
                }
            }
            new_args
        }
        Exp::ClosureCall(fun, args, _) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.extend(body_variables(arg, funs));
            }
            //???
            new_args.extend(body_variables(fun, funs));
            new_args
        }
        Exp::Lambda { parameters, body, .. } => {
            let mut body_var = body_variables(body, funs);
            //body_var.extend(decl.parameters.clone());
            for name  in parameters.iter(){
                if body_var.contains(name) {
                    body_var.remove(body_var.iter().position(|x| x == name).unwrap());
                }
            }
            body_var
        }
        Exp::ExternalCall { fun, args, is_tail:_, .. } => {
            let name = match fun {
                VarOrLabel::Label(name) => name.clone(),
                VarOrLabel::Var(name) => name.clone(),
            };
            let mut new_args = vec![];
            //let mut expected_args = vec![];
            //for fun in funs.clone(){
            //    if fun.0==*name {
            //        expected_args = fun.1.clone();
            //    }
            //}
            for arg in args {
                new_args.extend(body_variables(arg, funs));
            }
            for (fun_name, fun_para) in funs {
                if fun_name == &name {
                    //new_args.insert(0, name.clone());
                    println!("fun_para: {:?}", fun_para);
                    println!("args: {:?}", args);
                    println!("argLen: {}", args.len());
                    for para in 0..(fun_para.len()-args.len()) {
                        if !new_args.contains(fun_para.get(para).unwrap()) {
                            println!("para: {}", para);
                            new_args.insert(0, fun_para.get(para).unwrap().clone());
                        }
                    }
                }
            }
            new_args
        }
        Exp::MakeClosure { arity:_, label:_, env, .. } => {
            let mut new_args = vec![];
            new_args.extend(body_variables(env, funs));
            new_args
        }
        Exp::Semicolon {..} | Exp::Call(..) | Exp::InternalTailCall(..) => panic!("internal exp body")
    }
}
fn variable_capture_helper(p: Exp<bool>, funs : &mut Vec<(String, Vec<String>)>) -> Exp<bool>  
{
    match p {
        Exp::Num(n, ann) => Exp::Num(n, ann),
        Exp::Bool(b, ann) => Exp::Bool(b, ann),
        Exp::Float(f, ann) => Exp::Float(f, ann),
        Exp::Var(name, ann) => Exp::Var(name, ann),
        Exp::Prim(p, args, ann) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(Box::new(variable_capture_helper(*arg, funs)));
            }
            Exp::Prim(p, new_args, ann)
        }
        Exp::Let { bindings, body, ann} => {
            let mut new_bindings = vec![];
            for (name, exp) in bindings {
                let new_exp = variable_capture_helper(exp, funs);
                new_bindings.push((name.clone(), new_exp));
            }
            let new_body = variable_capture_helper(*body, funs);
            Exp::Let { bindings: new_bindings, body: Box::new(new_body), ann }
        }
        Exp::If {cond, thn, els, ann} => {
            let new_cond = variable_capture_helper(*cond, funs);
            let new_then = variable_capture_helper(*thn, funs);
            let new_else = variable_capture_helper(*els, funs);
            Exp::If {cond: Box::new(new_cond), thn: Box::new(new_then), els: Box::new(new_else), ann: ann}
        }
        Exp::FunDefs { decls, body, ann } => {
            println!("decls: {:?}", decls);
            println!("body: {:?}", body);
            let mut new_decls = vec![];
            let mut bfuns = vec![];
            for decl in decls.iter() {
                funs.push((decl.name.clone(), decl.parameters.clone()));
            }
            for decl in decls.clone() {
                bfuns.push((decl.name.clone(), decl.parameters.clone()));
            }
            let mut map = HashMap::new();
            for decl in decls.clone() {
                let new_body = variable_capture_helper(decl.body.clone(), funs);
                let new_para1 = body_variables(&decl.body, &mut bfuns);
                let mut new_para2 = decl.parameters.clone();
                for para in new_para1 {
                    if !new_para2.contains(&para){
                        new_para2.insert(0,para.clone());
                        map.insert(format!("{}_closure",decl.name.clone()), para.clone());
                        //println!("function: {} insert para: {}", decl.name.clone(), para.clone());
                    }
                }
                if map.contains_key(&decl.name) {
                    let para = map.get(&decl.name).unwrap();
                    new_para2.insert(0, para.clone());
                }
                new_decls.push(FunDecl {name: decl.name.clone(), parameters: new_para2.clone(), body: new_body, ann: ann.clone()});
            }
            let new_body = variable_capture_helper(*body, funs);
            Exp::FunDefs { decls: new_decls, body: Box::new(new_body), ann }
        }
        Exp::DirectCall(name, args, ann ) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(variable_capture_helper(arg,funs));
            }
            Exp::DirectCall(name, new_args, ann)
        }
        Exp::ClosureCall(fun, args, ann) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(variable_capture_helper(arg,funs));
            }
            Exp::ClosureCall(fun, new_args, ann)
        }
        Exp::Lambda { parameters, body, ann } => {
            let new_body = variable_capture_helper(*body,funs);
            Exp::Lambda { parameters, body: Box::new(new_body), ann }
        }
        Exp::MakeClosure { arity, label, env :clo_env, ann } => {
            Exp::MakeClosure { arity: arity.clone(), label: label.clone(), env: clo_env, ann: ann.clone() }
        }
        Exp::ExternalCall { fun, args, is_tail, ann } => {
            match fun {
                VarOrLabel::Label(name) => {
                    //let mut expected_args = vec![];
                    //for fun in funs.clone(){
                    //    if fun.0==*name {
                    //        expected_args = fun.1.clone();
                    //    }
                    //}
                    //println!("expected_args: {:?}", expected_args);
                    //println!("args: {:?}", args);
                    //let mut new_args = vec![];
                    //for arg in args {
                    //    new_args.push(variable_capture_helper(arg,funs));
                    //}
                    //for para in expected_args {
                    //    if !new_args.contains(&Exp::Var(para.clone(), ann.clone())) {
                    //        new_args.insert(0, Exp::Var(para.clone(), ann.clone()));
                    //    }
                    //}
                    //println!("new_args: {:?}", new_args);
                    Exp::ExternalCall { fun: VarOrLabel::Label(name.clone()), args: args, is_tail: is_tail.clone(), ann: ann.clone() }
                }
                _ => {
                    Exp::ExternalCall { fun: fun.clone(), args, is_tail: is_tail.clone(), ann: ann.clone() }
                }
            }
        }
        Exp::Semicolon {..} | Exp::Call(..) | Exp::InternalTailCall(..) => panic!("internal exp uniquify")
    }
}
fn variable_capture(mut p: Exp<bool>) -> Exp<bool> 
{
    //let mut ori_funs = fun.clone();
    let mut new_p = variable_capture_helper(p.clone(), &mut vec![]);
    while new_p != p {
        p = new_p;
        new_p = variable_capture_helper(p.clone(), &mut vec![]);
    }
    p
}
fn lambda_lift_helper(p: &Exp<bool>, should_lift: HashSet<String>, lambda_count:&mut i32, env:&mut HashSet<String>, funs : &mut Vec<(String, Vec<String>)>) -> (Vec<FunDecl<Exp<()>,()>>, Exp<()>) 
{
    match p {
        Exp::Num(n, _) => (vec![], Exp::Num(*n, ())),
        Exp::Bool(b, _) => (vec![], Exp::Bool(*b, ())),
        Exp::Float(f, _) => (vec![], Exp::Float(*f, ())),
        Exp::Var(name, _) => (vec![], Exp::Var(name.clone(), ())),
        Exp::Prim(p, args, _) => {
            let mut new_args = vec![];
            let mut lifted_funs = vec![];
            for arg in args {
                let (lifted_in_args, new_arg) = lambda_lift_helper(&arg, should_lift.clone(), lambda_count, env, funs);
                lifted_funs.extend(lifted_in_args);
                new_args.push(Box::new(new_arg));
            }
            (lifted_funs, Exp::Prim(*p, new_args, ()))
        }
        Exp::Let { bindings, body, ..} => {
            let mut new_bindings = vec![];
            let mut lifted_funs = vec![];
            for (name, exp) in bindings {
                env.insert(name.clone());
                let (lifted_in_exp, new_exp) = lambda_lift_helper(&exp, should_lift.clone(), lambda_count, env, funs);
                lifted_funs.extend(lifted_in_exp);
                new_bindings.push((name.clone(), new_exp));
            }
            let (lifted_in_body, new_body) = lambda_lift_helper(&body, should_lift.clone(), lambda_count, env, funs);
            lifted_funs.extend(lifted_in_body);
            (lifted_funs, Exp::Let {bindings: new_bindings, body: Box::new(new_body), ann: ()})
        }
        Exp::If {cond, thn, els, ..} => {
            let (lifted_in_cond, new_cond) = lambda_lift_helper(&cond, should_lift.clone(), lambda_count, env, funs);
            let (lifted_in_then, new_then) = lambda_lift_helper(&thn, should_lift.clone(), lambda_count, env, funs);
            let (lifted_in_else, new_else) = lambda_lift_helper(&els, should_lift.clone(), lambda_count, env, funs);
            let mut lifted_funs = vec![];
            lifted_funs.extend(lifted_in_cond);
            lifted_funs.extend(lifted_in_then);
            lifted_funs.extend(lifted_in_else);
            (lifted_funs, Exp::If {cond: Box::new(new_cond), thn: Box::new(new_then), els: Box::new(new_else), ann: ()})
        }
        Exp::FunDefs { decls, body, .. } => {
            let mut new_decls = vec![];
            let mut lifted_funs = vec![];
            for decl in decls {
                funs.push((decl.name.clone(), decl.parameters.clone()));
            }
            for decl in decls {
                env.insert(decl.name.clone());
                let mut new_env = env.clone();
                new_env.extend(decl.parameters.clone());
                let (lifted_in_body, new_body) = lambda_lift_helper(&decl.body, should_lift.clone(), lambda_count, &mut new_env, funs);
                lifted_funs.extend(lifted_in_body);
                if should_lift.contains(&decl.name) {
                    lifted_funs.push(FunDecl {name: decl.name.clone(), parameters: decl.parameters.clone(), body: new_body.clone(), ann: ()});
                }else{
                    new_decls.push(FunDecl {name: decl.name.clone(), parameters: decl.parameters.clone(), body: new_body, ann: ()});
                }
            }
            let (lifted_in_body, new_body) = lambda_lift_helper(&body, should_lift.clone(), lambda_count, env, funs);
            lifted_funs.extend(lifted_in_body);
            //(lifted_funs, Exp::FunDefs {decls: new_decls, body: Box::new(new_body), ann: ()})
            (lifted_funs, Exp::FunDefs {decls: new_decls, body: Box::new(new_body), ann: ()})
        }
        Exp::DirectCall(name, args, is_tail) => {
            if should_lift.contains(name) {
                let mut new_args = vec![];
                let mut lifted_funs = vec![];
                for arg in args {
                    let (lifted_in_arg, new_arg) = lambda_lift_helper(&arg, should_lift.clone(), lambda_count, env, funs);
                    lifted_funs.extend(lifted_in_arg);
                    new_args.push(new_arg);
                }
                (lifted_funs, Exp::ExternalCall {fun: VarOrLabel::Label(name.clone()), args: new_args, is_tail: *is_tail, ann: ()})
            }else {
                let mut new_args = vec![];
                let mut lifted_funs = vec![];
                for arg in args {
                    let (lifted_in_arg, new_arg) = lambda_lift_helper(&arg, should_lift.clone(), lambda_count, env, funs);
                    lifted_funs.extend(lifted_in_arg);
                    new_args.push(new_arg);
                }
                (lifted_funs, Exp::InternalTailCall(name.clone(), new_args, ()))
            }
        }
        Exp::ClosureCall(fun, args, ann) => {
            let mut lifted_funs = vec![];
            let (lifted_in_fun, new_fun) = lambda_lift_helper(&fun, should_lift.clone(), lambda_count, env, funs);
            let mut new_args = vec![];
            lifted_funs.extend(lifted_in_fun);
            for arg in args {
                let (lifted_in_arg, new_arg) = lambda_lift_helper(&arg, should_lift.clone(), lambda_count, env, funs);
                lifted_funs.extend(lifted_in_arg);
                new_args.push(new_arg);
            }
            
            new_args.insert(0, Exp::Var("env".to_string(), ()));
            (lifted_funs, 
                Exp::Let { bindings: 
                    vec![
                        ("untagged".to_string(), Exp::Prim(Prim::CheckArityAndUntag(args.len()), vec![Box::new(new_fun.clone())], () )),
                        ("code_ptr".to_string(), Exp::Prim(Prim::GetCode, vec![Box::new(Exp::Var("untagged".to_string(), ())), Box::new(Exp::Num(0, ()))], ())),
                        ("env".to_string(), Exp::Prim(Prim::GetEnv, vec![Box::new(Exp::Var("untagged".to_string(), ())), Box::new(Exp::Num(1, ()))], ()))
                        ,],
                    body: Box::new(    Exp::ExternalCall {fun: VarOrLabel::Var("code_ptr".to_string()), 
                    args: new_args, is_tail: *ann, ann: ()}    
                ), 
                    ann: () }
            )
        }
        Exp::Lambda { parameters, body, .. } => {
            let tag = lambda_count.clone();
            *lambda_count += 1;
            let mut lifted_funs = vec![];
            for para in parameters{
                env.insert(para.clone());
            }
            let (lifted_funs_in_body, new_body) = lambda_lift_helper(&body, should_lift.clone(), lambda_count, env, funs);
            println!("lambda_tag: {}", tag);
            println!("body: {:?}", body);
            let vars = body_variables(&body, funs);
            let mut lambda_env = vec![];
            println!("vars: {:?}", vars);
            println!("env: {:?}", env);
            for var in vars{
                if env.contains(&var){
                    if !parameters.contains(&var){
                        lambda_env.push(var.clone());
                    }
                }
            }
            lifted_funs.extend(lifted_funs_in_body);
            let mut new_parameters = parameters.clone();
            for para in lambda_env.clone() {
                if new_parameters.contains(&para) {
                    new_parameters.remove(new_parameters.iter().position(|x| x == &para).unwrap());
                }
            }
            new_parameters.insert(0, "env".to_string());
            let mut count = 0;
            let mut insert_body = new_body.clone();
            for var in lambda_env.clone(){
                insert_body = Exp::Let {
                    bindings: vec![(var.clone(), Exp::Prim(Prim::ArrayGet, vec![Box::new(Exp::Var("env".to_string(), ())), Box::new(Exp::Num(count, ()))], ()))], 
                    body: Box::new(insert_body.clone()), ann: ()};
                    count = count + 1;
            }
            let mut lam_env = vec![];
            for var in lambda_env.clone(){
                lam_env.push(Box::new(Exp::Var(var, ())));
            }
            let insert_env = Exp::Prim(Prim::MakeArray, lam_env, ());
            println!("lambda_env: {:?}", lambda_env);
            println!("new_parameters: {:?}", new_parameters);
            println!("insert_body: {:?}", insert_body);
            println!("insert_env: {:?}", insert_env);
            lifted_funs.push(FunDecl {name: format!("lambda{}", tag), parameters: new_parameters.clone(), body: insert_body, ann: ()});
            (lifted_funs , Exp::MakeClosure { arity: new_parameters.len()-1, label: format!("lambda{}", tag), env: Box::new(insert_env), ann: () })
        }
        Exp::ExternalCall { fun, args, is_tail, .. } => {
            let mut new_args = vec![];
            let mut lifted_funs = vec![];
            for arg in args {
                let (lifted_in_arg, new_arg) = lambda_lift_helper(&arg, should_lift.clone(), lambda_count, env, funs);
                lifted_funs.extend(lifted_in_arg);
                new_args.push(new_arg);
            }
            (lifted_funs, Exp::ExternalCall {fun: fun.clone(), args: new_args, is_tail: *is_tail, ann: ()})
        }
        Exp::MakeClosure { arity, label, env:clo_env, .. } => {
            let mut lifted_funs = vec![];
            let (lifted_in_env, new_env) = lambda_lift_helper(&clo_env, should_lift.clone(), lambda_count, env, funs);
            lifted_funs.extend(lifted_in_env);
            (lifted_funs, Exp::MakeClosure {arity: *arity, label: label.clone(), env: Box::new(new_env), ann: ()})
        }
        Exp::Semicolon {..} | Exp::Call(..) | Exp::InternalTailCall(..) => panic!("internal exp uniquify")
    }
}
fn recur_helper(p: &Exp<bool>, should_lift: HashSet<String>) -> Exp<bool>
{
    match p {
        Exp::Bool(b, ann) => Exp::Bool(*b, ann.clone()),
        Exp::Float(f, ann) => Exp::Float(*f, ann.clone()),
        Exp::Num(n, ann) => Exp::Num(*n, ann.clone()),
        Exp::Var(name, ann) => Exp::Var(name.clone(), ann.clone()),
        Exp::Prim(p, args, ann) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(Box::new(recur_helper(arg, should_lift.clone())));
            }
            Exp::Prim(*p, new_args, ann.clone())
        }
        Exp::Let { bindings, body, ann } => {
            let mut new_bindings = vec![];
            for (name, exp) in bindings {
                let new_exp = recur_helper(exp, should_lift.clone());
                new_bindings.push((name.clone(), new_exp));
            }
            let new_body = recur_helper(body, should_lift.clone());
            Exp::Let { bindings: new_bindings, body: Box::new(new_body), ann: ann.clone() }
        }
        Exp::If {cond, thn, els, ann} => {
            let new_cond = recur_helper(cond, should_lift.clone());
            let new_then = recur_helper(thn, should_lift.clone());
            let new_else = recur_helper(els, should_lift.clone());
            Exp::If {cond: Box::new(new_cond), thn: Box::new(new_then), els: Box::new(new_else), ann: ann.clone()}
        }
        Exp::FunDefs { decls, body, ann } => {
            let mut new_decls = vec![];
            let mut map = HashMap::new();
            let mut funs = vec![];
            for decl in decls.clone() {
                funs.push((decl.name.clone(), decl.parameters.clone()));
            }
            for decl in decls {
                if decl.name.contains("closure"){
                    continue;
                }
                let new_body = recur_helper(&decl.body, should_lift.clone());
                let new_para1 = body_variables(&decl.body, &mut funs);
                let mut new_para2 = decl.parameters.clone();
                for para in new_para1 {
                    if !new_para2.contains(&para){
                        new_para2.insert(0,para.clone());
                    }
                }
                let arity = decl.parameters.len();
                map.insert(decl.name.clone(), arity);
                //println!("decl.name: {}", decl.name.clone());
                //println!("new_para2: {:?}", new_para2);
                //println!("decl.parameters: {:?}", decl.parameters);
                new_decls.push(FunDecl {name: decl.name.clone(), parameters: new_para2.clone(), body: new_body, ann: ann.clone()});
                //closure
                let mut new_args = vec![];
                for para in decl.parameters.clone() {
                    new_args.push(Exp::Var(para, ann.clone()));
                }
                let mut count =0;
                for para in new_para2 {
                    if !decl.parameters.contains(&para) {
                        new_args.insert(0, Exp::Prim(Prim::ArrayGet, vec![Box::new(Exp::Var("env".to_string(), ann.clone())),
                        Box::new(Exp::Num(count, ann.clone()))], ann.clone()));
                        count = count + 1;
                    }
                }
                let mut new_para = decl.parameters.clone();
                new_para.insert(0, "env".to_string());
                new_decls.push(FunDecl {name: format!("{}_closure", decl.name.clone()), parameters: new_para.clone(), 
                body: Exp::ExternalCall { fun: VarOrLabel::Label(decl.name.clone()), args: new_args, is_tail: false, ann: ann.clone() }, 
                ann: ann.clone()});
            }
            let mut new_body = recur_helper(body, should_lift.clone());
            for decl in decls.clone() {
                if decl.name.contains("closure"){
                    continue;
                }
                let mut lambda_env_to_exp = vec![];
                for var in new_decls.clone().iter().find(|x| x.name == decl.name.clone()).unwrap().parameters.clone(){
                    if !decl.parameters.contains(&var) {
                        lambda_env_to_exp.push(Box::new(Exp::Var(var, ann.clone())));
                    }
                }
                let arity_num = match map.get(&decl.name) {
                    Some(n) => n,
                    None => {
                            panic!("unbound var")
                        },
                };
                new_body = Exp::Let {
                    bindings: vec![(decl.name.clone(), Exp::MakeClosure {
                        arity: arity_num.clone(),
                        label: format!("{}_closure", decl.name.clone()), 
                        env: Box::new(Exp::Prim(Prim::MakeArray, lambda_env_to_exp, ann.clone())), 
                        ann: ann.clone()})], body: Box::new(new_body.clone()), ann: ann.clone()};
            }
            Exp::FunDefs { decls: new_decls, body: Box::new(new_body), ann: ann.clone() }
        }
        Exp::DirectCall(name, args, is_tail) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(recur_helper(arg, should_lift.clone()));
            }
            Exp::DirectCall(name.clone(), new_args, is_tail.clone())
        }
        Exp::ClosureCall(fun, args, is_tail) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(recur_helper(arg, should_lift.clone()));
            }
            let new_fun = recur_helper(fun, should_lift.clone());
            Exp::ClosureCall(Box::new(new_fun), new_args, is_tail.clone())
        }
        Exp::Lambda { parameters, body, ann } => {
            let new_body = recur_helper(body, should_lift.clone());
            Exp::Lambda { parameters: parameters.clone(), body: Box::new(new_body), ann: ann.clone() }
        }
        Exp::ExternalCall { .. } | Exp::Semicolon { .. } | Exp::MakeClosure { .. } | Exp::Call(..) | Exp::InternalTailCall(..)=> {
            panic!("internal exp recur")
        }
    }
}
// Lift some functions to global definitions
fn lambda_lift<Ann>(p: &Exp<Ann>) -> (Vec<FunDecl<Exp<()>, ()>>, Exp<()>)
where Ann: PartialEq + Clone
{
    let should_lift = should_lift(p);
    let mut new_should_lift = should_lift.clone();
    for string in should_lift.clone() {
        new_should_lift.insert(format!("{}_closure", string));
    }
    for string in new_should_lift.clone() {
        println!("should lift: {}", string);
    }
    let re_p = recur_helper(&mark_tail(&p, true), new_should_lift.clone());
    let var_cap = variable_capture(re_p);
    lambda_lift_helper(&var_cap, new_should_lift, &mut 0, &mut HashSet::new(), &mut vec![])
}

//fn sequentialize(e: &Exp<u32>) -> SeqExp<()> {
//    panic!("NYI: sequentialize")
//}
//fn seq_prog(decls: &[FunDecl<Exp<u32>, u32>], p: &Exp<u32>) -> SeqProg<()> {
//    panic!("NYI: seq_prog")
//}
fn sequentialize_funs (decls: &Vec<FunDecl<Exp<u32>, u32>>) -> Vec<FunDecl<SeqExp<()>, ()>>{
    let mut new_decls = vec![];
    let mut global_funs = vec![];
    for decl in decls {
        global_funs.push((decl.name.clone(), decl.parameters.clone()));
    }
    for decl in decls {
        let new_body = sequentialize(&decl.body, &mut global_funs);
        new_decls.push(FunDecl {name: decl.name.clone(), parameters: decl.parameters.clone(), body: new_body, ann: ()});
    }
    new_decls
}
fn seq_prog(decls: &Vec<FunDecl<Exp<()>, ()>>, p: &Exp<u32>) -> SeqProg<()> {
    let new_decls = tag_exp(&Exp::FunDefs { decls: decls.clone(), body: Box::new(Exp::Num(1, ())), ann: () });
    let mut global_funs = vec![];
    for decl in decls {
        global_funs.push((decl.name.clone(), decl.parameters.clone()));
    }
    match new_decls {
        Exp::FunDefs { decls, body: _, ann: _ } => {
            SeqProg { funs: sequentialize_funs(&decls), main: sequentialize(p, &mut global_funs), ann: () }
        }
        _ => panic!("seq_prog error")
    }
}
fn desugar_semicolon<Ann>(p: &SurfProg<Ann>) -> SurfProg<Ann> 
where Ann: Clone
{
    match p {
        Exp::Bool(b, ann) => Exp::Bool(*b, ann.clone()),
        Exp::Num(n, ann) => Exp::Num(*n, ann.clone()),
        Exp::Float(f, ann) => Exp::Float(*f, ann.clone()),
        Exp::Var(name, ann) => Exp::Var(name.clone(), ann.clone()),
        Exp::Prim(p, args, ann) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(Box::new(desugar_semicolon(arg)));
            }
            Exp::Prim(*p, new_args, ann.clone())
        }
        Exp::Let { bindings, body, ann } => {
            let mut new_bindings = vec![];
            for (name, exp) in bindings {
                let new_exp = desugar_semicolon(exp);
                new_bindings.push((name.clone(), new_exp));
            }
            let new_body = desugar_semicolon(body);
            Exp::Let { bindings: new_bindings, body: Box::new(new_body), ann: ann.clone() }
        }
        Exp::If {cond, thn, els, ann} => {
            let new_cond = desugar_semicolon(cond);
            let new_then = desugar_semicolon(thn);
            let new_else = desugar_semicolon(els);
            Exp::If {cond: Box::new(new_cond), thn: Box::new(new_then), els: Box::new(new_else), ann: ann.clone()}
        }
        Exp::FunDefs { decls, body, ann } => {
            let mut new_decls = vec![];
            for decl in decls {
                let new_body = desugar_semicolon(&decl.body);
                new_decls.push(FunDecl {name: decl.name.clone(), parameters: decl.parameters.clone(), body: new_body, ann: ann.clone()});
            }
            let new_body = desugar_semicolon(body);
            Exp::FunDefs { decls: new_decls, body: Box::new(new_body), ann: ann.clone() }
        }
        Exp::Call(name, args, is_tail) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(desugar_semicolon(arg));
            }
            Exp::Call(name.clone(), new_args, is_tail.clone())
        }
        Exp::Lambda { parameters, body, ann } => {
            let new_body = desugar_semicolon(body);
            Exp::Lambda { parameters: parameters.clone(), body: Box::new(new_body), ann: ann.clone() }
        }
        Exp::Semicolon { e1, e2, ann } => {
            Exp::Let { bindings: vec![("()".to_string(),desugar_semicolon(e1))], body: Box::new(desugar_semicolon(e2)), ann: ann.clone() }
        }
        Exp::InternalTailCall(..) | Exp::ExternalCall{..} | Exp::ClosureCall(..) | Exp::DirectCall(..) | Exp::MakeClosure { .. } => {
            panic!("internel exp found in desugar_semicolon")
        }
    }
}
pub fn compile_to_string<Span>(p: &SurfProg<Span>) -> Result<String, CompileErr<Span>>
where
    Span: Clone,
{
    let p = desugar_semicolon(p);
    let () = check_prog(&p)?;
    let tagged_exp = tag_exp(&p);
    println!("tagged_exp: {:?}", tagged_exp);
    let well_scoped_with_unique_names = uniquify(&tagged_exp);
    let elimin_exp = eliminate_closures(&well_scoped_with_unique_names);
    println!("well_scoped_with_unique_names:{:?}", well_scoped_with_unique_names);
    //println!("elimin_exp: {:?}", elimin_exp);
    let (lifted_funs, ast) = lambda_lift(&elimin_exp);
    for fun in lifted_funs.clone() {
        println!("name: {}", fun.name);
        println!("lifted_funs: {:?}", fun);
    }
    println!("ast: {:?}", ast);
    let seq_prog = seq_prog(&lifted_funs, &tag_exp(&ast));
    //println!("\nmain:");
    //let () = print_seq_exp(&seq_prog.main.clone());
    for fun in seq_prog.funs.clone() {
        println!("name: {}", fun.name);
        println!("seq_prog_fun: {:?}", fun);
    }
    let is = compile_to_instrs(&tag_prog(seq_prog));
Ok(format!(
        "\
        section .data
        HEAP:    times 1024 dq 0
section .text
        global start_here
        extern snake_error
        extern print_snake_val
start_here:
        push R15
        sub RSP, 8
        lea r15, [rel HEAP]
        call main
        add rsp, 8
        pop r15
        ret
{}
error_ari_not_number:
        mov rsi, rax
        mov rdi, 1
        call snake_error
error_com_not_number:
        mov RSI, RAX
        mov RDI, 2
        call snake_error
error_overflow:
        mov rsi, rax
        mov rdi, 3
        call snake_error
error_if_not_boolean:
        mov RSI, RAX
        mov RDI, 4
        call snake_error
error_logic_not_boolean:
        mov RSI, RAX
        mov RDI, 5
        call snake_error
error_index_not_number:
        mov RSI, RAX
        mov RDI, 6
        call snake_error
error_index_out_of_bound:
        mov RSI, RAX
        mov RDI, 7
        call snake_error
error_index_into_nonarray:
        mov RSI, RAX
        mov RDI, 8
        call snake_error
error_length_into_nonarray:
        mov RSI, RAX
        mov RDI, 9
        call snake_error
error_not_closure:
        mov RSI, RAX
        mov RDI, 10
        call snake_error
error_wrong_arity:
        mov RSI, R10
        mov RDI, 11
        call snake_error
error_fari_not_float:
        mov RSI, R10
        mov RDI, 12
        call snake_error
error_fcom_not_float:
        mov RSI, R10
        mov RDI, 13
        call snake_error
",
        instrs_to_string(&is)
    ))
}
