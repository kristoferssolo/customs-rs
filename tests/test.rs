use customs_rs::process::process;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn customs_1() {
        let input = include_str!("../data/input/customs.i1");
        let output = include_str!("../data/output/customs.o1");
        assert_eq!(process(input), output);
    }
    #[test]
    fn customs_2() {
        let input = include_str!("../data/input/customs.i2");
        let output = include_str!("../data/output/customs.o2");
        assert_eq!(process(input), output);
    }
    #[test]
    fn customs_3() {
        let input = include_str!("../data/input/customs.i3");
        let output = include_str!("../data/output/customs.o3");
        assert_eq!(process(input), output);
    }
    #[test]
    fn customs_4() {
        let input = include_str!("../data/input/customs.i4");
        let output = include_str!("../data/output/customs.o4");
        assert_eq!(process(input), output);
    }
    #[test]
    fn customs_5() {
        let input = include_str!("../data/input/customs.i5");
        let output = include_str!("../data/output/customs.o5");
        assert_eq!(process(input), output);
    }
    #[test]
    fn customs_6() {
        let input = include_str!("../data/input/customs.i6");
        let output = include_str!("../data/output/customs.o6");
        assert_eq!(process(input), output);
    }
    #[test]
    fn customs_7() {
        let input = include_str!("../data/input/customs.i7");
        let output = include_str!("../data/output/customs.o7");
        assert_eq!(process(input), output);
    }
}
