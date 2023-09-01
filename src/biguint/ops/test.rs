use crate::traits::{Digit, Pow, TrueDiv};
use crate::BigUint;

use typed_test_gen::test_with;

#[test_with(u32, u64)]
fn add_assign<T: Digit>() {
    let mut bg = BigUint::<T>::from(0u32);
    bg += T::ONE;

    assert_eq!(bg.to_string(), "1");

    bg += &BigUint::<T>::from(100u32);

    assert_eq!(bg.to_string(), "101");
}

#[test_with(u32, u64)]
fn add_assign_overflow<T: Digit>() {
    let mut bg = BigUint::<T>::new(T::MAX);
    bg += T::ONE;

    assert_eq!(bg.val, vec![T::ZERO, T::ONE]);

    bg += T::ONE;

    assert_eq!(bg.val, vec![T::ONE, T::ONE]);
}

#[test_with(u32, u64)]
fn add<T: Digit>() {
    let b1 = BigUint::<T>::from(100u32);
    let b2 = BigUint::<T>::from(50u32);

    assert_eq!(&b1 + &b2, BigUint::<T>::from(150u32));
    assert_eq!(&b1 + T::ONE, BigUint::<T>::from(101u32));
}

#[test_with(u32, u64)]
fn add_assign_full_test<T: Digit>() {
    let mut b1 = BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]);
    b1 += T::ONE;

    assert_eq!(b1.val, vec![T::ZERO, T::ZERO, T::ZERO, T::ONE]);

    let b = &BigUint::<T>::new(T::MAX) + &BigUint::<T>::new(T::MAX);
    assert_eq!(b.val, vec![T::MAX - T::ONE, T::ONE]);
}

#[test_with(u32, u64)]
fn sub<T: Digit>() {
    let mut b1 = BigUint::<T>::from(vec![T::MAX, T::MAX]);
    b1 -= T::MAX;

    assert_eq!(b1.val, vec![T::ZERO, T::MAX]);
    b1 -= T::ONE;
    assert_eq!(b1.val, vec![T::MAX, T::MAX - T::ONE]);
    b1 -= &BigUint::<T>::from(vec![T::MAX, T::MAX - T::ONE]);
    assert_eq!(b1, BigUint::<T>::default());
}

#[test_with(u32, u64)]
fn sub_full<T: Digit>() {
    let n1 = BigUint::<T>::from("12345678910111213");
    let n2 = BigUint::<T>::from("987654321");

    let n3 = &n1 - &n2;
    assert_eq!(String::from(&n3), "12345677922456892");
}

#[test_with(u32, u64)]
fn fibonacci<T: Digit>() {
    let mut n1 = BigUint::<T>::new(T::ZERO);
    let mut n2 = BigUint::<T>::new(T::ONE);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        assert_eq!(&n2 - &n1, temp);
        assert_eq!(&n2 - &(&n1 + &temp), BigUint::<T>::new(T::ZERO));
        n1 = temp;
        n -= 1;
    }

    assert_eq!(
        String::from(&n2),
        "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125"
    );
}

#[test_with(u32, u64)]
fn fibonacci_5<T: Digit>() {
    let mut n1 = BigUint::<T>::new(T::ZERO);
    let mut n2 = BigUint::<T>::new(T::ONE);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }

    n2 *= BigUint::<T>::from(5u32);

    assert_eq!(
        String::from(&n2),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
}

#[test_with(u32, u64)]
fn fibonacci_5_bis<T: Digit>() {
    let mut n1 = BigUint::<T>::new(T::ZERO);
    let mut n2 = BigUint::<T>::new(T::ONE);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }

    let n3 = &n2 * &BigUint::<T>::from(5u32);
    assert_eq!(
        String::from(&n3),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
    let n3 = &BigUint::<T>::from(5u32) * &n2;
    assert_eq!(
        String::from(&n3),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
}

#[test_with(u32, u64)]
fn fibonacci_square<T: Digit>() {
    let mut n1 = BigUint::<T>::new(T::ZERO);
    let mut n2 = BigUint::<T>::new(T::ONE);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let n1 = n2.clone();
    assert_eq!(
        String::from(&n2),
        "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125"
    );
    assert_eq!(
        String::from(&n1),
        "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125"
    );

    assert_eq!(
        String::from(&n1 * &BigUint::<T>::from(505575602u32)),
        "704889806905615918937\
		649989837846505682417079166195319138308619361953957652\
		23225155085259704056844689235065938250"
    );

    let n3 = &n2 * &n1;
    n2 *= &n1;
    assert_eq!(
        String::from(&n3),
        "1943883554718163504159639641586\
		529474755957583106913701654561621695450376368896305381\
		612382980919365953591566108064186948023260738153211479\
		434817249275036032824029805681946181926453630633514153\
		3339064759515625"
    );
    assert_eq!(
        String::from(&n2),
        "1943883554718163504159639641586\
		529474755957583106913701654561621695450376368896305381\
		612382980919365953591566108064186948023260738153211479\
		434817249275036032824029805681946181926453630633514153\
		3339064759515625"
    );
}

#[test_with(u32, u64)]
fn factorial_100<T: Digit>() {
    let mut n = BigUint::<T>::new(T::ONE);

    for i in 1..=100u32 {
        n *= BigUint::<T>::from(i);
    }

    assert_eq!(
        String::from(n),
        "933262154439441526816992388562667\
		004907159682643816214685929638952175999932299156089414\
		639761565182862536979208272237582511852109168640000000\
		00000000000000000"
    );
}

#[test_with(u32, u64)]
fn fact_mod<T: Digit>() {
    let mut n1 = BigUint::<T>::new(T::ZERO);
    let mut n2 = BigUint::<T>::new(T::ONE);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let n3 = &n2 * &n2;

    assert_eq!(&n3 % &BigUint::<T>::from(13u32), BigUint::<T>::from(9u32));
    assert_eq!(
        &n3 % &BigUint::<T>::from(4294967295u32),
        BigUint::<T>::from(637285095u32)
    );
}

#[test_with(u32, u64)]
fn fact_div<T: Digit>() {
    let mut n1 = BigUint::<T>::new(T::ZERO);
    let mut n2 = BigUint::<T>::new(T::ONE);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let mut n3 = &n2 * &n2;

    assert_eq!(
        &n3 / &BigUint::<T>::from(13u32),
        BigUint::<T>::from(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );
    assert_eq!(
        &n3 / &BigUint::<T>::from(4294967295u32),
        BigUint::<T>::from(
            "45259565933858024969189\
    	89092294192835654543867968879846044465914135558921337249217357\
    	80617389059621687664461274568142421215633665995869120916045822\
    	7133103309984645193164261759358644035107871882721534"
        )
    );

    n3 /= &BigUint::<T>::from(13u32);
    assert_eq!(
        n3,
        BigUint::<T>::from(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );
}

#[test_with(u32, u64)]
fn mul_test<T: Digit>() {
    let n1 = BigUint::<T>::from("4294967295");
    let n2 = BigUint::<T>::from("4294967295");
    let n3 = &n1 * &n2;

    assert_eq!(String::from(&n3), "18446744065119617025");
}

#[test_with(u32, u64)]
fn shl_assign_test<T: Digit>() {
    let mut b = BigUint::<T>::new(T::ONE << (T::NB_BITS - 1));
    let b2 = &b << (T::NB_BITS + 1);
    assert_eq!(b2.val, vec![T::ZERO, T::ZERO, T::ONE]);
    b <<= T::NB_BITS + 1;
    assert_eq!(b.val, vec![T::ZERO, T::ZERO, T::ONE]);
}

#[test_with(u32, u64)]
fn shr_assign_test<T: Digit>() {
    let mut b = BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]);
    let b2 = &b >> (T::NB_BITS + 1);
    assert_eq!(b2.val, vec![T::MAX, T::MAX >> 1]);
    b >>= T::NB_BITS + 1;
    assert_eq!(b.val, vec![T::MAX, T::MAX >> 1]);
}

#[test_with(u32, u64)]
fn bit_and<T: Digit>() {
    let mut n1 = BigUint::<T>::from(vec![T::MAX, T::ONE]);
    let n2 = BigUint::<T>::from(vec![T::ONE, T::MAX, T::MAX]);

    assert_eq!(&n1 & &n2, BigUint::<T>::from(vec![T::ONE, T::ONE]));
    assert_eq!(&n2 & &n1, BigUint::<T>::from(vec![T::ONE, T::ONE]));

    n1 &= &n2;
    assert_eq!(n1, BigUint::<T>::from(vec![T::ONE, T::ONE]));
}

#[test_with(u32, u64)]
fn bit_or<T: Digit>() {
    let mut n1 = BigUint::<T>::from(vec![T::MAX, T::ONE]);
    let n2 = BigUint::<T>::from(vec![T::ONE, T::MAX, T::MAX]);

    assert_eq!(&n1 | &n2, BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));
    assert_eq!(&n2 | &n1, BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    n1 |= &n2;
    assert_eq!(n1, BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));
}

#[test_with(u32, u64)]
fn bit_xor<T: Digit>() {
    let mut n1 = BigUint::<T>::from(vec![T::MAX, T::ONE]);
    let n2 = BigUint::<T>::from(vec![T::ONE, T::MAX, T::MAX]);

    assert_eq!(
        &n1 ^ &n2,
        BigUint::<T>::from(vec![T::MAX - T::ONE, T::MAX - T::ONE, T::MAX])
    );
    assert_eq!(
        &n2 ^ &n1,
        BigUint::<T>::from(vec![T::MAX - T::ONE, T::MAX - T::ONE, T::MAX])
    );

    n1 ^= &n2;
    assert_eq!(
        n1,
        BigUint::<T>::from(vec![T::MAX - T::ONE, T::MAX - T::ONE, T::MAX])
    );
}

#[test_with(u32, u64)]
fn mul_assign_u32<T: Digit>() {
    let mut b = BigUint::<T>::from("2147483648");
    b *= T::ONE << 3;

    assert_eq!(b.to_string(), "17179869184");
}

#[test_with(u32, u64)]
#[should_panic]
fn div_by_zero<T: Digit>() {
    let n = BigUint::<T>::from("12345");
    let _ = &n / T::ZERO;
}

#[test_with(u32, u64)]
fn div_1<T: Digit>() {
    let mut a = BigUint::<T>::from(16u32);

    a /= &BigUint::<T>::from(3u32);
    assert_eq!(a, BigUint::<T>::from(5u32));

    a /= &BigUint::<T>::from(5u32);
    assert_eq!(a, BigUint::<T>::from(1u32));

    a /= &BigUint::<T>::from(10u32);
    assert_eq!(a, BigUint::<T>::from(0u32));
}

#[test_with(u32, u64)]
fn div_2<T: Digit>() {
    let mut n1 = BigUint::<T>::new(T::ZERO);
    let mut n2 = BigUint::<T>::new(T::ONE);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let mut n3 = &n2 * &n2;

    assert_eq!(
        &n3 / &BigUint::<T>::from(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        ),
        BigUint::<T>::from(13u32)
    );
    assert_eq!(
        &n3 / &BigUint::<T>::from(13u32),
        BigUint::<T>::from(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );
    assert_eq!(
        &n3 / &BigUint::<T>::from("4294967295"),
        BigUint::<T>::from(
            "45259565933858024969189\
    	89092294192835654543867968879846044465914135558921337249217357\
    	80617389059621687664461274568142421215633665995869120916045822\
    	7133103309984645193164261759358644035107871882721534"
        )
    );

    n3 /= &BigUint::<T>::from(13u32);
    assert_eq!(
        n3,
        BigUint::<T>::from(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );

    let mut n4 = &n3 * &BigUint::<T>::from(vec![T::ONE, T::ONE, T::ONE]);
    assert_eq!(&n4 % &n3, BigUint::<T>::default());

    n4 /= &BigUint::<T>::from(vec![T::ONE, T::ONE, T::ONE]);
    assert_eq!(n3, n4);
}

#[test_with(u32, u64)]
fn rem<T: Digit>() {
    let a = BigUint::<T>::from("81129638419329048179758161985792");
    let b = BigUint::<T>::from("571849066607118647405");
    assert_eq!((&a % &b).to_string(), "268965438589694318452");
}

#[test_with(u32, u64)]
fn product<T: Digit>() {
    let values = vec![T::ONE; 20];

    let n1: BigUint<T> = values.iter().product();
    assert_eq!(n1, BigUint::<T>::from("1"));

    let big_values = values
        .iter()
        .map(|n| BigUint::<T>::new(*n))
        .collect::<Vec<_>>();
    let n2: BigUint<T> = big_values.iter().product();
    assert_eq!(n2, BigUint::<T>::from("1"));
}

#[test_with(u32, u64)]
fn sum<T: Digit>() {
    let values = vec![T::MAX; 10];
    let mut s = format!("{:?}", T::MAX);
    s.push('0');

    let n1: BigUint<T> = values.iter().sum();
    println!("{:?}", n1.to_string());
    assert_eq!(n1.to_string(), s);

    let big_values = values
        .iter()
        .map(|n| BigUint::<T>::new(*n))
        .collect::<Vec<_>>();
    let n2: BigUint<T> = big_values.iter().sum();
    assert_eq!(n2.to_string(), s);
}

#[test_with(u32, u64)]
fn truediv<T: Digit>() {
    let n1 = BigUint::<T>::from("123456678890123345567789");
    let n2 = BigUint::<T>::from("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 10000000270550.242f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n1 = BigUint::<T>::from("123456678890123345567789") << 15;
    let n2 = BigUint::<T>::from("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 3.2768000886539034e+17f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n1 = BigUint::<T>::from("123456678890123345567789") << 3030;
    let n2 = BigUint::<T>::from("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = f64::INFINITY;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n2 = BigUint::<T>::from("123456678890123345567789");
    let n1 = BigUint::<T>::from("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 9.999999729449765e-14f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n2 = BigUint::<T>::from("12345667889012334556778900000000");
    let n1 = BigUint::<T>::from("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 9.999999729449765e-22f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n1 = BigUint::<T>::from("12345667889012334556778900000000");
    let n2 = BigUint::<T>::from("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 1.0000000270550242e+21f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);
}

#[test_with(u32, u64)]
fn pow<T: Digit>() {
    let n = BigUint::<T>::from(5u32);
    assert_eq!(n.pow(0), BigUint::<T>::from(1u32));
    assert_eq!(n.pow(1), BigUint::<T>::from(5u32));
    assert_eq!(n.pow(2), BigUint::<T>::from(25u32));
    assert_eq!(n.pow(3), BigUint::<T>::from(125u32));

    let n = BigUint::<T>::from(128u32);
    let n2 = n.pow(50);
    assert_eq!(
        n2,
        BigUint::<T>::from(
            "2293498615990071511610820895302086940796564989168281\
            123737588839386922876088484808070018553110125686554624"
        )
    );

    let n = BigUint::<T>::from(128u32);
    let n2 = n.pow(16);
    assert_eq!(n2, BigUint::<T>::from("5192296858534827628530496329220096"));

    let n = BigUint::<T>::from(128u32);
    let n2 = n.pow(15);
    assert_eq!(n2, BigUint::<T>::from("40564819207303340847894502572032"));
}

#[test_with(u32, u64)]
fn long_mul<T: Digit>() {
    let n1 = BigUint::<T>::from(vec![T::ONE; 100]);
    let n2 = BigUint::<T>::from(vec![T::ONE; 100]);

    let mut ret = Vec::<T>::with_capacity(2 * 100 - 1);
    for i in 0..100 {
        ret.push(T::decomposition_from_u32(i + 1)[0]);
    }
    for i in (0..99).rev() {
        ret.push(T::decomposition_from_u32(i + 1)[0]);
    }

    let n3 = n1 * n2;
    assert_eq!(n3.val, ret);
}

#[test]
fn maxed_out_mul() {
    let n1 = BigUint::<u32>::from(vec![u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
    let n2 = BigUint::<u32>::from(vec![u32::MAX, u32::MAX, u32::MAX, u32::MAX]);

    let n3 = &n1 * &n2;

    assert_eq!(
        n3,
        BigUint::<u32>::from(
            "115792089237316195423570985008687907852589419931798687112530834793049593217025"
        )
    );
}
