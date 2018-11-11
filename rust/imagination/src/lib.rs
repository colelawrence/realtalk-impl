use std::collections::BTreeMap;

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    input * 2
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct TemplateId(usize);
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct EntityId(usize);

#[derive(Clone, Debug)]
pub enum Value {
    EntityId(EntityId),
    Int(isize),
    Str(String),
    Float(f64),
}

#[derive(Debug)]
pub enum Subject {
    EntityId(EntityId),
    Global,
}

#[derive(Debug)]
pub enum QueryArgument {
    CapturesTo(u8),  // argument position when calling next function
    PinnedTo(Value), // This argument must equal this
}

#[derive(Debug)]
struct QueryStatement {
    template_id: TemplateId,
    origin: EntityId,
    /// Arg positions in template function callback and relations between [QueryStatement]s
    query_args: Vec<QueryArgument>, // ex: [1, 3]
}

#[derive(Debug)]
pub struct Query {
    statements: Vec<QueryStatement>,
}

#[derive(Debug)]
pub struct FactStatement {
    template_id: TemplateId,
    origin: EntityId,
    subject: Subject,
    args: Vec<Value>,
}

pub struct System {
    name: String,
    // entities: Vec<EntityId>, // this should actually be entities of some kind I guess
    facts: BTreeMap<TemplateId, Vec<FactStatement>>,
}

impl System {
    pub fn fact(
        &mut self,
        origin: EntityId,
        template: &TemplateId,
        subject: Subject,
        values: Vec<Value>,
    ) {
        let fact = FactStatement {
            template_id: template.clone(),
            origin,
            subject,
            args: values,
        };
        if self.facts.contains_key(&template) {
            self.facts.get_mut(&template).unwrap().push(fact);
        } else {
            self.facts.insert(template.clone(), vec![fact]);
        };
        // just put the fact in the table
    }

    pub fn query(&mut self, query: Query) -> Vec<Vec<Value>> {
        let mut facts: Vec<(QueryStatement, &Vec<FactStatement>)> =
            Vec::with_capacity(query.statements.len());
        let Query { statements: stmts } = query;
        for stmt in stmts {
            let stmt_facts: &Vec<FactStatement> = match self.facts.get(&stmt.template_id) {
                None => return Vec::new(), // abort! There are no special things
                Some(ref fact_stmts) => fact_stmts,
            };
            facts.push((stmt, stmt_facts));
        }

        println!("Facts {:?}", facts);

        return Vec::new();
    }

    pub fn register() {}
}

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn test_free(s: *mut c_char) {
    unsafe { CString::from_raw(s); }
}

#[no_mangle]
// pub extern "C" fn test(source: *const c_char, dest: *mut c_char) -> size_t {
pub extern "C" fn test(source: *const c_char) -> *mut c_char {
    let t1 = unsafe {
        CStr::from_ptr(source)
    };
    println!("t1 {:?}", t1);
    let res = reverse(t1.to_str().expect("valid utf8"));
    let res_cstr = CString::new(res.as_bytes()).expect("no nul");
    println!("cstr: {:?}", res_cstr);
    // let res_ptr = res_cstr.into_raw();
    res_cstr.into_raw()
}

fn reverse(src: &str) -> String {
    src.chars().rev().collect::<String>()
}
