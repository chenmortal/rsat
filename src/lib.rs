pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod assign;
pub mod cnf;
mod db;
// mod error;
mod graph;
pub mod lit;
pub mod solver;
// mod test;
mod cdcl;
mod decision;
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
