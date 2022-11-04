use modinverse::modinverse;

// In modular arithmetic,   the modular multiplicative inverse of an integer   a   modulo   m   is an integer   x   such that 

/*ax ≡ 1 (mod m).

Such an integer may not exist. If so, this function will return None. Otherwise, the inverse will be returned wrapped up in a Some.*/
// The modinverse crate has an implementation of the extended Euclidean algorithm built in. 

let does_exist = modinverse(3, 26);
let does_not_exist = modinverse(4, 32);

match does_exist {
  Some(x) => assert_eq!(x, 9),
  None => panic!("modinverse() didn't work as expected"),
}

match does_not_exist {
  Some(x) => panic!("modinverse() found an inverse when it shouldn't have"),
  None => {},
}
