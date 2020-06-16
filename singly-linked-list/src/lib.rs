mod singly_list {
    use std::rc::Rc;

    pub enum List<T>{
        Node(T, Rc<List<T>>),
        Nil,
    }

    impl<T> List<T> {
        pub fn new() -> List<T>  {
            List::Nil
        }


        pub fn cons(self, t:T) -> List<T> {
            List::Node(t, Rc::new(self))
        }

        pub fn head(&self) -> Option<&T> {
            match self {
                List::Node(t, _ ) => Some(t),
                List::Nil => None
            }

        }

        pub fn rest(&self) -> &List<T> {
            match self {
                List::Node(_, rest) =>  rest,
                List::Nil => &List::Nil
            }
        }
    }


    #[test]
    fn test_cons(){
        let list = List::new().cons("a").cons("b");
        assert_eq!(list.head(), Some(&"b"));
        assert_eq!(list.rest().head(), Some(&"a"));
        assert_eq!(list.rest().rest().head(), None);
    }

}


