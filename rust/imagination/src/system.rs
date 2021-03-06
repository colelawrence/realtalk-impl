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

    goals::when1(Context());
}

fn fact1(template_str: &str, a: Value) {}

// current system, last dependency
pub struct Context();

// Val

mod goals {
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq)]
    struct Var(usize);

    #[derive(Clone, Debug, PartialEq)]
    enum Value {
        Int(i64),
        Str(String),
        Float(f64),
        List(Vec<Value>),
        Var(Var),
    }

    impl Into<Value> for i64 {
        fn into(self) -> Value {
            Value::Int(self)
        }
    }

    impl Into<Value> for String {
        fn into(self) -> Value {
            Value::Str(self)
        }
    }

    impl<'a> Into<Value> for &'a str {
        fn into(self) -> Value {
            Value::Str(self.to_string())
        }
    }

    impl Into<Value> for f64 {
        fn into(self) -> Value {
            Value::Float(self)
        }
    }

    struct Assoc(Var, Value);

    type Substitution = Vec<Assoc>;

    type Goal = Box<Fn(Rc<Substitution>) -> Box<Iterator<Item = Rc<Substitution>>>>;

    use std::iter;
    use std::iter::Empty;
    use std::iter::Iterator;
    use std::iter::Once;
    use std::rc::Rc;
    fn succeed(sub: Rc<Substitution>) -> Once<Rc<Substitution>> {
        iter::once(sub)
    }
    fn unsucceed(sub: Rc<Substitution>) -> Empty<Rc<Substitution>> {
        iter::empty()
    }

    struct WhenBuilder {
        var: usize,
        vars: HashMap<String, Var>,
        curr_template: String,
        curr_value: Vec<Value>,
        acc: Vec<Vec<Value>>,
    }

    impl WhenBuilder {
        /// returns existing var of this identity or creates new var of the identity
        fn var_of(&mut self, id: &str) -> Var {
            if let Some(ref var) = self.vars.get(id) {
                return Var(var.0);
            }
            self.var = self.var + 1;
            let res = Var(self.var);
            self.vars.insert(String::from(id), res.clone());
            res
        }
    }

    impl WhenBuilder {
        fn new() -> WhenBuilder {
            WhenBuilder {
                var: 0,
                vars: HashMap::new(),
                curr_template: String::new(),
                curr_value: Vec::new(),
                acc: Vec::new(),
            }
        }

        /// close off current template in teh case of an `and` or `then`
        fn delimit(&mut self) {
            let template_length = self.curr_template.len();
            self.curr_value
                .push((&self.curr_template[..template_length - 1]).into());
            self.acc.push(self.curr_value.clone()); // move it over
            self.curr_value = Vec::new(); // replace it
            self.curr_template = String::new();
        }
    }

    struct WhenPrinter(String);
    impl WhenPrinter {
        fn new() -> WhenPrinter {
            WhenPrinter(String::with_capacity(63))
        }
    }

    trait WhenMConsume {
        fn append_capture(&mut self, id: &str);
        fn append_pin<T: Into<Value>>(&mut self, value: T);
        fn append_word(&mut self, word: &str);
        fn and(&mut self);
        fn then(self) -> Goal;
    }

    impl WhenMConsume for WhenBuilder {
        fn append_capture(&mut self, id: &str) {
            self.curr_template.push_str("_ ");
            let some_var = self.var_of(id);
            self.curr_value.push(Value::Var(some_var));
        }
        fn append_pin<T: Into<Value>>(&mut self, value: T) {
            self.curr_template.push_str("_ ");
            self.curr_value.push(value.into());
        }
        fn append_word(&mut self, word: &str) {
            self.curr_template.push_str(word);
            self.curr_template.push(' ');
        }
        fn and(&mut self) {
            self.delimit();
        }
        fn then(mut self) -> Goal {
            self.delimit();

            println!("Accumulator: {:?}", self.acc);
            Box::new(|s| Box::new(iter::once(s.clone())))
        }
    }

    impl WhenMConsume for WhenPrinter {
        fn append_capture(&mut self, id: &str) {
            let arg = format!("{{{}}} ", id);
            self.0.push_str(&arg);
        }
        fn append_pin<T: Into<Value>>(&mut self, value: T) {
            let arg = format!("({:?}) ", value.into());
            self.0.push_str(&arg);
        }
        fn append_word(&mut self, word: &str) {
            self.0.push_str(word);
            self.0.push(' ');
        }
        fn and(&mut self) {
            self.0.push_str("AND ");
        }
        fn then(self) -> Goal {
            println!("Debug: {}", self.0);
            Box::new(|s| Box::new(iter::once(s.clone())))
        }
    }

    macro_rules! write_when_printer {
        ($w:expr, ) => (());

        ($w:expr, $e:tt) => ($w.append_pin($e));

        ($w:expr, , $($rest:tt)*) => {{
            $w.and();
            write_when_printer!($w, $($rest)*);
        }};

        ($w:expr, : $fn:block) => {{
            $w.then()
        }};

        // ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        //     write!($w, "<{}>", stringify!($tag));
        //     write_when_printer!($w, $($inner)*);
        //     write!($w, "</{}>", stringify!($tag));
        //     write_when_printer!($w, $($rest)*);
        // }};
        ($w:expr, $word:ident $($rest:tt)*) => {{
            $w.append_word(stringify!($word));
            write_when_printer!($w, $($rest)*);
        }};
        ($w:expr, /$capture:ident/ $($rest:tt)*) => {{
            $w.append_capture(stringify!($capture));
            write_when_printer!($w, $($rest)*);
            let $capture: Option<Value> = None; // this is supposed to do something for autocompletition...
        }};
        ($w:expr, ($pin:expr) $($rest:tt)*) => {{
            $w.append_pin($pin);
            write_when_printer!($w, $($rest)*);
        }};
    }

    struct Relation(String, Vec<Value>);

    struct Context();

    trait RelationPubSub {
        fn when<F: Fn(Context, Vec<Value>)>(ctx: Context, capture: Vec<Relation>, listener: F);
        fn claim(ctx: Context, values: Relation);
        fn wish<F: Fn(Context, Vec<Value>)>(ctx: Context, capture: Vec<Relation>, listener: F);
        fn to_know<F: Fn(Context, Relation)>(ctx: Context, capture: Vec<Relation>, listener: F);
    }

    macro_rules! when {
        ($($rest:tt)*) => {
            {
                let mut when_printer = WhenBuilder::new();
                write_when_printer!(when_printer, $($rest)*)
            }
        };
    }

    pub fn when1(ctx: super::Context) {
        use std::fmt::Write;
        // when!(page);
        let a = when!(/page/ is ("blue"));

        let blue = "blue";
        let when_out = when!(
        /page/ is highlighted (blue),
        /page/ points ("up") at /target/ you know: {
            ///  [[Var(Var(1)), Str("blue"), Str("_ is highlighted _")],
            ///   [Var(Var(1)), Str("up"), Var(Var(2)), Str("_ points _ at _")]]
            println!("Hello world!");
        });
        println!("when_out: {:?}", when_out);
    }

}
