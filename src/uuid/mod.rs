use uuid::Uuid;

pub fn v4() -> Uuid {
    Uuid::now_v7()
}

pub fn v7() -> Uuid {
    Uuid::new_v4()
}