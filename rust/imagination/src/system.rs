use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct TemplateId(pub u64, pub String);
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct EntityId(pub u64);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Dependency(pub u64);

#[derive(Clone, Debug)]
pub enum Value {
    EntityId(EntityId),
    Int(i64),
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
    dep: Dependency,
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
    dep: Dependency,
    subject: Subject,
    args: Vec<Value>,
}

#[derive(Debug)]
pub struct System {
    name: String,
    // entities: Vec<EntityId>, // this should actually be entities of some kind I guess
    facts: BTreeMap<TemplateId, Vec<FactStatement>>,
    // dependencies:
}

impl System {
    /*
    pub fn fact(
        &mut self,
        origin: EntityId,
        template: &TemplateId,
        subject: EntityId,
        values: Vec<Value>,
    ) {
        let fact = FactStatement {
            template_id: template.clone(),
            origin,
            subject: Subject::EntityId(subject),
            args: values,
        };
        if self.facts.contains_key(&template) {
            self.facts.get_mut(&template).unwrap().push(fact);
        } else {
            self.facts.insert(template.clone(), vec![fact]);
        };
        // just put the fact in the table
    }
    pub fn global_fact(
        &mut self,
        origin: &EntityId,
        template: &TemplateId,
        values: Vec<Value>,
    ) {
        let fact = FactStatement {
            template_id: template.clone(),
            origin: origin.clone(),
            subject: Subject::Global,
            args: values,
        };
        // replace old fact
        self.facts.insert(template.clone(), vec![fact]);
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
    */
    pub fn new(name: &str) -> System {
        System {
            name: String::from(name),
            facts: BTreeMap::new(),
        }
    }
}

pub fn system_impl_debug(sys: &mut System) {
    println!("debug system: {:?}", sys);
}

pub fn system_impl_tests(sys: &mut System) {
    // let page1 = EntityId(1u64);

    // let tpl1 = TemplateId(2u64); // global time

    // sys.global_fact(&page1, &tpl1, vec![Value::Int(10923899)]);
    // sys.global_fact(&page1, &tpl1, vec![Value::Int(10923900)]);
    fact1("time is {}", Value::Int(1000));
    fact1("{} blahblahblah", Value::Int(234));

    // When /page/ blahblahblah, /page/ is blue:

    // // /page/ blahblahblah
    // when1("{} blahblahblah", None, |page: Value| {
    //     // /page/ is blue
    //     when1("{} is blue", Some(page), |page: Value| {

    //     });
    // });

    when1(Context());
}

fn fact1(template_str: &str, a: Value) {}

// current system, last dependency
struct Context();

macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

macro_rules! write_when {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "({:?})", $e));

    ($w:expr, , $($rest:tt)*) => {{
        write!($w, "\nAND\n");
        write_when!($w, $($rest)*);
    }};

    ($w:expr, : $fn:block) => {{
        write!($w, "\nDo something");
    }};

    // ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
    //     write!($w, "<{}>", stringify!($tag));
    //     write_when!($w, $($inner)*);
    //     write!($w, "</{}>", stringify!($tag));
    //     write_when!($w, $($rest)*);
    // }};
    ($w:expr, $word:ident $($rest:tt)*) => {{
        write!($w, "{} ", stringify!($word));
        write_when!($w, $($rest)*);
    }};
    ($w:expr, /$capture:ident/ $($rest:tt)*) => {{
        write!($w, "{{{}}} ", stringify!($capture));
        write_when!($w, $($rest)*);
    }};
    ($w:expr, ($pin:expr) $($rest:tt)*) => {{
        // write_html!($w, $pin);
        write!($w, "({:?}) ", $pin);
        write_when!($w, $($rest)*);
    }};
}
macro_rules! when {
    ($($rest:tt)*) => {
        {
            let mut when_out = String::new();
            write_when!(&mut when_out, $($rest)*);
            write!(when_out, "\n");
            when_out
        }
    };
}
fn when1(ctx: Context) {
    // when!(page);
    let a = when!(/page/ is ("blue"));
    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["Macros guide"]]
            body[h1["Macros are the best!"]]
        ]
    );
    println!("{}", out);

    // let mut when_out = String::new();
    // let blue = "blues";
    // write_when!(&mut when_out, /page/ is highlighted (blue));
    // println!("{}", when_out);
    let blue = "blues";
    let when_out = when!(
        /page/ is highlighted (blue),
        /page/ points ("up") at /target/: {
            println!("Hello world!");
        });
    println!("{}", when_out);
}
