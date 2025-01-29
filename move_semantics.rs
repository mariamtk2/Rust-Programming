
 // excerise 1
 /*
  fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec;
    
        vec.push(88);
    
        vec
    }
    
    fn main() {
        
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn move_semantics1() {
            let vec0 = vec![22, 44, 66];
            let vec1 = fill_vec(vec0);
            assert_eq!(vec1, vec![22, 44, 66, 88]);
        }
    }

    */

    // exercise 2
/*
    fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec;
    
        vec.push(88);
    
        vec
    }
    
    fn main() {
       
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        
        #[test]
        fn move_semantics2() {
            let vec0 = vec![22, 44, 66];
    
            let vec1 = fill_vec(vec0.clone());

        
            assert_eq!(vec0, [22, 44, 66]);
            assert_eq!(vec1, [22, 44, 66, 88]);
        }
    }
*/

// excerise 3

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
