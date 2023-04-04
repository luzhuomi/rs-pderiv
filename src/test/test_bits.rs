use bitvec::prelude::*;
use crate::regex::re::*;
use crate::regex::pderiv::*;
use crate::regex::pderiv::bits::*;
use std::rc::Rc;

#[test]
fn test_empcode_star_a_a() {
    use RE::*;
    let r = star!(Lit('a'));
    let result = emp_code(&r);
    assert_eq!(result,bitvec![1]); 
}


#[test]
fn test_empcode_eps_star_a_a() {
    use RE::*;
    let r = seq!(Eps,star!(Lit('a')));
    let result = emp_code(&r);
    assert_eq!(result,bitvec![1]); 
}


#[test]
fn test_empcode_choice_star_a_eps_a() {
    use RE::*;
    let r = choice!(star!(Lit('a')), Eps);
    let result = emp_code(&r);
    assert_eq!(result,bitvec![0,1]); 
}

#[test]
fn test_empcode_star_choice_a_b() {
    use RE::*;
    let r = star!(choice!(Lit('a'), Lit('b')));
    let result = emp_code(&r);
    assert_eq!(result,bitvec![1]); 
}

#[test]
fn test_pderiv_bc_same_as_pderiv_star_a() {
    use RE::*;
    let r = star!(Lit('a'));
    let lhs = pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}


#[test]
fn test_pderiv_bc_same_as_pderiv_choice_star_a_eps() {
    use RE::*;
    let r = choice!(star!(Lit('a')), Eps);
    let lhs = pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}




#[test]
fn test_pderiv_bc_same_as_pderiv_star_choice_a_b() {
    use RE::*;
    let r = star!(choice!(Lit('a'), Lit('b')));
    let lhs = pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}

#[test]
fn test_pderiv_bc_same_as_pderiv_abaac() {
    use RE::*;
    let x = choice!(Lit('a'),seq!(Lit('a'),Lit('b')));
    let y = choice!(seq!(Lit('b'), seq!(Lit('a'), Lit('a'))), Lit('a'));
    let z = choice!(seq!(Lit('a'), Lit('c')), Lit('c')); 
    let r = seq!(seq!(x,y),z);
    let lhs = pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}


#[test]
fn test_cached_pderiv_bc_same_as_pderiv_star_a() {
    use RE::*;
    let r = star!(Lit('a'));
    let mut cached = PDCached::new();
    let lhs = cached.pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}





#[test]
fn test_cached_pderiv_bc_same_as_pderiv_choice_star_a_eps() {
    use RE::*;
    let r = choice!(star!(Lit('a')), Eps);
    let mut cached = PDCached::new();
    let lhs = cached.pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}




#[test]
fn test_cached_pderiv_bc_same_as_pderiv_star_choice_a_b() {
    use RE::*;
    let r = star!(choice!(Lit('a'), Lit('b')));
    let mut cached = PDCached::new();
    let lhs = cached.pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}

#[test]
fn test_cached_pderiv_bc_same_as_pderiv_abaac() {
    use RE::*;
    let x = choice!(Lit('a'),seq!(Lit('a'),Lit('b')));
    let y = choice!(seq!(Lit('b'), seq!(Lit('a'), Lit('a'))), Lit('a'));
    let z = choice!(seq!(Lit('a'), Lit('c')), Lit('c')); 
    let r = seq!(seq!(x,y),z);
    let mut cached = PDCached::new();
    let lhs = cached.pderiv_bc(&r, &'a');
    let rhs = pderiv(&r, &'a');
    assert_eq!(lhs.len(), rhs.len());
    for (idx, p) in lhs.iter().enumerate() {
        assert_eq!(&p.0, rhs[idx].as_ref());
    }
}