use disjoint_impls::disjoint_impls;

pub trait Dispatch {
    type Group;
}

pub enum GroupA {}
impl Dispatch for String {
    type Group = GroupA;
}
impl<T> Dispatch for Vec<T> {
    type Group = GroupA;
}

pub enum GroupB {}
impl Dispatch for i32 {
    type Group = GroupB;
}
impl Dispatch for u32 {
    type Group = GroupB;
}

pub struct Wrapper<T>(T);

disjoint_impls! {
    impl<T> Wrapper<T> where T: Dispatch<Group = GroupA> {
        pub const NAME: &'static str = "Blanket A";
    }
    impl<T> Wrapper<T> where T: Dispatch<Group = GroupB> {
        pub const NAME: &'static str = "Blanket B";
    }
}

/*
const _: () = {
    trait _Wrapper<T0> {
        const NAME: &'static str;
    }

    impl<T0> _Wrapper<GroupA> for Wrapper<T0> where T0: Dispatch<Group = GroupA> {
        const NAME: &'static str = "Blanket A";
    }
    impl<T0> _Wrapper<GroupB> for Wrapper<T0> where T0: Dispatch<Group = GroupB> {
        const NAME: &'static str = "Blanket B";
    }

    impl<T0> Wrapper<T0> where T0: Dispatch, Self: _Wrapper<<T0 as Dispatch>::Group> {
        const NAME: &'static str = <Self as _Wrapper<<T0 as Dispatch>::Group>>::NAME;
    }
};
*/

#[test]
fn main() {
    assert_eq!("Blanket A", <Wrapper<String>>::NAME);
    assert_eq!("Blanket A", <Wrapper<Vec::<u32>>>::NAME);
    assert_eq!("Blanket B", <Wrapper<u32>>::NAME);
    assert_eq!("Blanket B", <Wrapper<i32>>::NAME);
}
