use zenlang::rawvec::RawVec;

#[test]
fn rawvec_test_1() {
    let mut vec = RawVec::<i32>::new();
    vec.dealloc();
}

#[test]
fn rawvec_test_2() {
    let mut vec = RawVec::<i32>::new();
    vec.push(123);
    vec.dealloc();
}

#[test]
fn rawvec_test_3() {
    let mut vec = RawVec::<i32>::new();
    vec.push(123);
    vec.push(69);
    vec.push(420);

    assert_eq!(*vec.last(), 420);
    vec.pop();
    assert_eq!(*vec.last(), 69);
    vec.pop();
    assert_eq!(*vec.last(), 123);
    vec.pop();

    vec.dealloc();
}

#[test]
fn rawvec_test_4() {
    let mut vec = RawVec::<i32>::new();
    vec.push(123);
    vec.push(69);
    vec.push(420);

    assert_eq!(*vec.at(1), 69);
    vec.remove(0);
    assert_eq!(*vec.at(1), 420);

    vec.dealloc();
}

#[test]
#[should_panic]
fn rawvec_test_5() {
    let mut vec = RawVec::<i32>::new();
    vec.pop();
    vec.dealloc();
}

#[test]
#[should_panic]
fn rawvec_test_6() {
    let mut vec = RawVec::<i32>::new();
    vec.push(420);
    vec.pop();
    vec.pop();
    vec.dealloc();
}

#[test]
#[should_panic]
fn rawvec_test_7() {
    let mut vec = RawVec::<i32>::new();
    vec.remove(1);
    vec.dealloc();
}

#[test]
fn rawvec_test_8() {
    let mut vec = RawVec::<i32>::new();
    vec.push(123);
    vec.push(69);
    vec.push(420);

    for (index, e) in vec.iter().enumerate() {
        assert_eq!(*vec.at(index), e);
    }

    vec.dealloc();
}

#[test]
fn rawvec_test_9() {
    let mut vec = RawVec::<i32>::new();
    vec.push(123);
    vec.push(69);
    vec.push(420);

    for (index, e) in vec.iter().rev().enumerate() {
        assert_eq!(*vec.at(vec.len() - index - 1), e);
    }

    vec.dealloc();
}
