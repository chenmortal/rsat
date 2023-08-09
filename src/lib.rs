pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod assign;
mod cnf;
mod db;
mod error;
mod graph;
mod lit;
mod solver;
// mod test;
mod prop;
mod watch;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
