pub trait Kita {
    const NAME: &'static str;
}
pub trait Dispatch1 {
    type Group;
}
pub trait Dispatch2 {
    type Group;
}

pub enum GroupA {}
pub enum GroupB {}

impl Dispatch1 for String {
    type Group = GroupA;
}
impl Dispatch2 for String {
    type Group = GroupA;
}
impl<T> Dispatch1 for Vec<T> {
    type Group = GroupA;
}
impl<T> Dispatch2 for Vec<T> {
    type Group = GroupB;
}

impl Dispatch1 for i32 {
    type Group = GroupB;
}
impl Dispatch2 for i32 {
    type Group = GroupA;
}
impl Dispatch1 for u32 {
    type Group = GroupB;
}
impl Dispatch2 for u32 {
    type Group = GroupB;
}

disjoint::impls! {
    impl<T: Dispatch1<Group = GroupA> + Dispatch2<Group = GroupA>> Kita for T {
        const NAME: &'static str = "Blanket AA";
    }
    impl<T: Dispatch1<Group = GroupA> + Dispatch2<Group = GroupB>> Kita for T {
        const NAME: &'static str = "Blanket AB";
    }
    impl<T: Dispatch1<Group = GroupB>> Kita for T {
        const NAME: &'static str = "Blanket B*";
    }
}

// Expands to:
//trait _Kita<T1, T2> {
//    const _NAME: &'static str;
//}
//
//impl<T: Dispatch1 + Dispatch2 + _Kita<<T as Dispatch1>::Group, <T as Dispatch2>::Group>> Kita for T {
//    const NAME: &'static str = <T as _Kita<<T as Dispatch1>::Group, <T as Dispatch2>::Group>>::_NAME;
//}
//
//impl<T> _Kita<GroupA, GroupA> for T {
//    const _NAME: &'static str = "Blanket AA";
//}
//impl<T> _Kita<GroupA, GroupB> for T {
//    const _NAME: &'static str = "Blanket AB";
//}
//impl<T, F> _Kita<GroupB, F> for T {
//    const _NAME: &'static str = "Blanket B*";
//}

fn main() {
    assert_eq!("Blanket AA", String::NAME);
    assert_eq!("Blanket AB", Vec::<u32>::NAME);
    assert_eq!("Blanket B*", u32::NAME);
    assert_eq!("Blanket B*", i32::NAME);
}
