// Clone Copy Debug PartialEq
// 层级 打个比方，结构体里面有个枚举，那么结构体实现Clone，里面的枚举也实现Clone

// 给枚举Race实现Debug，Clone
#[derive(Debug, Clone, Copy)]
enum Race {
    White,
    Yellow,
    Black,
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        match(self, other) {
            (Race::White, Race::White) => true,
            (Race::Yellow, Race::Yellow) => true,
            (Race::Black, Race::Black) => true,
            _ => false,
        }
    }
}

// 给结构体User实现Debug, Clone
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    race: Race,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.race == other.race
    }
}

fn main() {
    let user = User{
        id: 3,
        name: "John".to_owned(),
        race: Race::Yellow,
    };

    println!("{:#?}", user);

    let user2 = user.clone();
    println!("{:#?}", user2);

    println!("{:#?}", user == user2);
}
