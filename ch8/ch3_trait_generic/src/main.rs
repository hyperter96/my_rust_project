trait Overview {
    fn overview(&self) -> String {
        String::from("Course")
    }
}

trait Another {
    fn hell(&self) {
        println!("welcome to hell")
    }
}

struct Course {
    headline: String,
    author: String,
}

// Course实现两种特质
impl Overview for Course {}
impl Another for Course {}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for AnotherCourse {}

fn call_overview(item: &impl Overview) {
    println!("Overview {}", item.overview());
}

fn call_overview_generic<T: Overview>(item: &T) {
    println!("Overview generic {}", item.overview());
}

fn call_overviewT(item: &impl Overview, item1: &impl Overview) {
    println!("OverviewT {}", item.overview());
    println!("OverviewT {}", item1.overview());
}

// 只允许传入同类型结构体的引用
fn call_overviewTT<T: Overview>(item: &T, item1: &T) {
    println!("OverviewTT {}", item.overview());
    println!("OverviewTT {}", item1.overview());
}

// 多绑定
fn call_mul_bind(item: &(impl Overview + Another)) {
    println!("Overview {}", item.overview());
    item.hell();
}

// 多绑定泛型
fn call_mul_bind_generic<T>(item: &T)
where 
    T: Overview + Another,
{
    println!("Overview {}", item.overview());
    item.hell();
}

fn main() {
    let c0 = Course {
        headline: "ff".to_owned(),
        author: "yy".to_owned(),
    };
    let c1 = Course {
        headline: "ff".to_owned(),
        author: "yy".to_owned(),
    };

    let c2 = AnotherCourse {
        headline: "ff".to_owned(),
        author: "yz".to_owned(),
    };
    call_overview(&c1); // Overview Course
    call_overview_generic(&c1); // Overview generic Course
    call_overviewT(&c1, &c2); 
    // OverviewT Course
    // OverviewT Course
    call_overviewTT(&c0, &c1); // 应用范围宅一些
    // OverviewTT Course
    // OverviewTT Course
    call_overviewT(&c0, &c1);
    // OverviewT Course
    // OverviewT Course
    call_mul_bind(&c1);
    // Overview Course
    // welcome to hell
    call_mul_bind_generic(&c1);
    // Overview Course
    // welcome to hell
}
