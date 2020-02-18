// mode probe_struct

#[derive(Debug, Copy, Clone)]  // compiler creates Debug, Copy, Clone
pub struct DemoCopy {          // Debug and Copy are called implicitly
    pub f:f64,                 // Clone must be called explicitly.
}
impl DemoCopy {
    #[allow(dead_code)]
    pub fn new(f : f64) -> Self {
        Self {
            f,
        }
    }
}

///////////////////////////////////////////////////////////
// DemoClone implements Clone trait
//---------------------------------------------------------
// can't define Copy trait because string field is not blitable
// but String defines clone, so we can implement clone.
//---------------------------------------------------------
#[derive(Debug)]
pub struct DemoClone {
    pub s:String,
}
impl Clone for DemoClone {  // can't define Copy - String is not blitable
    fn clone(&self) -> Self {
        DemoClone::new(self.s.clone()) 
        // note: no semicolon so will be returned
        
        // This works too
        // DemoClone { s : self.s.clone(), }
    }
}
impl DemoClone {
    #[allow(dead_code)]
    pub fn new(s : String) -> Self {
        Self {
            s,
        }
    }
}
///////////////////////////////////////////////////////////
// user-defined traits act like interfaces

trait Speaker {
    fn salutation(&self) -> String;
}
///////////////////////////////////////////////////////////
// The following structs act like classes that implement
// a Speaker interface

#[derive(Debug,Copy,Clone)]
pub struct Presenter;
impl Speaker for Presenter {
    fn salutation(&self) -> String {
      "Hello, today we will discuss ...".to_string()
    }
} 

#[derive(Debug)]
pub struct Friend {
    pub name : String,
}
impl Speaker for Friend {
    fn salutation(&self) -> String {
      let mut s = String::from("Hi good buddy, its me, ");
      let nm = self.name.as_str();
      s.push_str(nm);
      return s;
    }
}
impl Friend {
    #[allow(dead_code)]
    pub fn new(name : String) -> Self {
        Self {
            name,
        }  // note: no semicolon so Self is returned
    }
}
#[derive(Debug,Copy,Clone)]
pub struct TeamLead;
impl Speaker for TeamLead {
    fn salutation(&self) -> String {
      "Hi, I have a task for you ...".to_string()
    }
} 

#[allow(dead_code)]
pub fn run() {

    print!("\n  {}","-- demo struct ownership --\n");

    let dc = DemoCopy::new(3.1415927);
    print!("\n  dc = {:?}", dc);

    let dcc = dc;  // copy of dc as it is blitable
    print!("\n  dcc = {:?} - copy of dc", dcc);

    let dccc = dcc.clone();
    print!("\n  dccc = {:?} - clone of dcc",dccc);

    let mut sc = DemoClone::new("demo clone".to_string());
    sc.s.push_str(" plus more stuff");
    print!("\n  sc = {:?} - cloneable", sc);

    let mut scc = sc.clone();
    print!("\n  scc = {:?} - a clone", scc); 
    scc.s.push_str(", again more");
    print!("\n  scc = {:?} - after mutation",scc);
    print!("\n  sc = {:?} - original",sc);

    let msccr = &mut sc;
    print!("\n  msccr = {:?} - &mut sc",msccr);
    msccr.s = "new string".to_string();
    print!("\n  msccr = {:?} - after mutation", &msccr);

    let sccr = &sc;
    print!("\n  sccr = {:?} - &sc",sccr);
    print!("\n  ref sccr makes use of mut ref msccr illegal");

    ////////////////////////////////////////////////
    // Fails to compile because there is another
    // reference: sccr
    //msccr.s.push_str(" modified");
    //print!("\n  msccr = {:?} - modified",msccr);

    // https://stackoverflow.com/questions/25818082/vector-of-objects-belonging-to-a-trait
    
    print!("\n\n  {}","-- demo polymorphic struct instances --\n");
    let presenter : Presenter = Presenter;
    let joe : Friend = Friend::new("Joe".to_string());
    let sue : Friend = Friend::new("Sue".to_string());
    let team_lead : TeamLead = TeamLead;

    let mut v :Vec<&dyn Speaker> = Vec::new();
    v.push(&presenter);
    v.push(&joe);
    v.push(&sue);
    v.push(&team_lead);

    for speaker in v.iter() {
        print!("\n  {:?}",speaker.salutation());
    }
}