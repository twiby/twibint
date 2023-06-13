use crate::BigInt;

#[test]
fn add_assign() {
    let mut bg = BigInt::new(0);
    bg += 100u32;

    assert_eq!(bg.val, vec![100]);

    bg += &BigInt::new(100);

    assert_eq!(bg.val, vec![200]);
}

#[test]
fn add_assign_overflow() {
    let mut bg = BigInt::new(u32::MAX);
    bg += 1u32;

    assert_eq!(bg.val, vec![0, 1]);

    bg += &BigInt::new(100);

    assert_eq!(bg.val, vec![100, 1]);
}

#[test]
fn add() {
    let b1 = BigInt::new(100);
    let b2 = BigInt::new(50);

    assert_eq!(&b1 + &b2, BigInt::new(150));
    assert_eq!(&b1 + 50, BigInt::new(150));
}

#[test]
fn add_assign_full_test() {
    let mut b1 = bigintvec![u32::MAX, u32::MAX, u32::MAX];
    b1 += 1;

    assert_eq!(b1.val, vec![0, 0, 0, 1]);

    let b = &BigInt::new(u32::MAX) + &BigInt::new(u32::MAX);
    assert_eq!(b.val, vec![4294967294, 1]);
}

#[test]
fn sub() {
    let mut b1 = bigintvec![u32::MAX, u32::MAX];
    b1 -= u32::MAX;

    assert_eq!(b1, bigintvec![0, u32::MAX]);
    b1 -= 1;
    assert_eq!(b1, bigintvec![u32::MAX, u32::MAX - 1]);
    b1 -= &bigintvec![u32::MAX, u32::MAX - 1];
    assert_eq!(b1, BigInt::new(0));
}

#[test]
fn sub_full() {
    let n1 = bigint!("12345678910111213");
    let n2 = bigint!("987654321");

    let n3 = &n1 - &n2;
    assert_eq!(String::from(&n3), "12345677922456892");
}

#[test]
fn fibonacci() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        assert_eq!(&n2 - &n1, temp);
        assert_eq!(&n2 - &(&n1 + &temp), BigInt::new(0));
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

#[test]
fn fibonacci_5() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }

    n2 *= 5u32;

    assert_eq!(
        String::from(&n2),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
}

#[test]
fn fibonacci_5_bis() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }

    let n3 = &n2 * &BigInt::new(5);
    assert_eq!(
        String::from(&n3),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
    let n3 = &BigInt::new(5) * &n2;
    assert_eq!(
        String::from(&n3),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
}

#[test]
fn fibonacci_square() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
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

    let n3 = &n1 * &bigintvec![1, 2];
    assert_eq!(
        String::from(&n3),
        "1197636379730135883426986149904\
		088164401882181843047237880919768151177099109732530421\
		226027034957340830083465166125"
    );

    assert_eq!(
        String::from(&n1 * 505575602),
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

#[test]
fn factorial_100() {
    let mut n = BigInt::new(1);

    for i in 1..=100u32 {
        n *= i;
    }

    assert_eq!(
        String::from(n),
        "933262154439441526816992388562667\
		004907159682643816214685929638952175999932299156089414\
		639761565182862536979208272237582511852109168640000000\
		00000000000000000"
    );
}

#[test]
fn fact_mod() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let n3 = &n2 * &n2;

    assert_eq!(&n3 % 13, 9);
    assert_eq!(&n3 % 4294967295, 637285095);
}

#[test]
fn fact_div() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let mut n3 = &n2 * &n2;

    assert_eq!(
        &n3 / 13,
        bigint!(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );
    assert_eq!(
        &n3 / 4294967295,
        bigint!(
            "45259565933858024969189\
    	89092294192835654543867968879846044465914135558921337249217357\
    	80617389059621687664461274568142421215633665995869120916045822\
    	7133103309984645193164261759358644035107871882721534"
        )
    );

    n3 /= 13;
    assert_eq!(
        n3,
        bigint!(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );
}

#[test]
fn mul_test() {
    let n1 = BigInt::new(4294967295);
    let n2 = BigInt::new(4294967295);
    let n3 = &n1 * &n2;

    assert_eq!(String::from(&n3), "18446744065119617025");
}

#[test]
fn pure_mul_test() {
    let n1 = bigintvec![4294967295, 4294967295, 4294967295];
    let n2 = &n1 * 4294967295;
    assert_eq!(String::from(&n2), "340282366841710300949110269833929293825");
}

#[test]
fn shl_assign_test() {
    let b = BigInt::new(2147483648);
    let b2 = &b << 33;
    assert_eq!(b2.val, vec![0, 0, 1]);
}

#[test]
fn shr_assign_test() {
    let b = bigintvec![u32::MAX, u32::MAX, u32::MAX];
    let b2 = &b >> 33;
    assert_eq!(b2.val, vec![u32::MAX, u32::MAX >> 1]);
}

#[test]
fn bit_and() {
    let mut n1 = bigintvec![u32::MAX, 15];
    let n2 = bigintvec![15, u32::MAX, u32::MAX];

    assert_eq!(&n1 & &n2, bigintvec![15, 15]);
    assert_eq!(&n2 & &n1, bigintvec![15, 15]);

    n1 &= &n2;
    assert_eq!(n1, bigintvec![15, 15]);
}

#[test]
fn bit_or() {
    let mut n1 = bigintvec![u32::MAX, 15];
    let n2 = bigintvec![15, u32::MAX, u32::MAX];

    assert_eq!(&n1 | &n2, bigintvec![u32::MAX, u32::MAX, u32::MAX]);
    assert_eq!(&n2 | &n1, bigintvec![u32::MAX, u32::MAX, u32::MAX]);

    n1 |= &n2;
    assert_eq!(n1, bigintvec![u32::MAX, u32::MAX, u32::MAX]);
}

#[test]
fn bit_xor() {
    let mut n1 = bigintvec![u32::MAX, 15];
    let n2 = bigintvec![15, u32::MAX, u32::MAX];

    assert_eq!(&n1 ^ &n2, bigintvec![4294967280, 4294967280, u32::MAX]);
    assert_eq!(&n2 ^ &n1, bigintvec![4294967280, 4294967280, u32::MAX]);

    n1 ^= &n2;
    assert_eq!(n1, bigintvec![4294967280, 4294967280, u32::MAX]);
}

#[test]
fn mul_assign_u32() {
    let mut b = BigInt::new(2147483648);
    b *= 5;

    assert_eq!(b.to_string(), "10737418240");
}

#[test]
#[should_panic]
fn div_by_zero() {
    let n = BigInt::new(12345);
    let _ = &n / 0u32;
}

#[test]
fn div_1() {
    let mut a = 16u32;

    a /= &BigInt::new(3);
    assert_eq!(a, 5u32);

    a /= &BigInt::new(5);
    assert_eq!(a, 1u32);

    a /= &BigInt::new(10);
    assert_eq!(a, 0u32);
}

#[test]
fn div_2() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let mut n3 = &n2 * &n2;

    assert_eq!(
        &n3 / &bigint!(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        ),
        BigInt::from(13)
    );
    assert_eq!(
        &n3 / &bigint!(13),
        bigint!(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );
    assert_eq!(
        &n3 / 4294967295,
        bigint!(
            "45259565933858024969189\
    	89092294192835654543867968879846044465914135558921337249217357\
    	80617389059621687664461274568142421215633665995869120916045822\
    	7133103309984645193164261759358644035107871882721534"
        )
    );

    n3 /= 13;
    assert_eq!(
        n3,
        bigint!(
            "1495295042090895003199722801220\
    	40728827381352546685669358043201668880798182222792721662490998\
    	53225891950704354677416822677102005678101626764883209609807969\
    	48326176773601497063020348946641164733333774212270432"
        )
    );

    n3 /= &bigintvec![256, 256, 1024];
    assert_eq!(
        n3,
        bigint!(
            "79160260402704383443505518173623327171271715\
		894251231563668951724860277618637301958917651208758398\
		250711971981987758781540730998305599662194105412632969\
		281358142708870468507852092297047"
        )
    );

    let mut n4 = &n3 * &bigintvec![256, 256, 1024];
    assert_eq!(&n4 % &n3, BigInt::new(0));

    n4 /= &bigintvec![256, 256, 1024];
    assert_eq!(n3, n4);
}

#[test]
fn rem() {
    let a = bigintvec![256, 256, 256, 1024];
    let b = bigintvec![109, 75, 31];
    println!("{:?}", a.to_string());
    println!("{:?}", b.to_string());
    assert_eq!((&a % &b).to_string(), "268965438589694318452");
}

#[test]
fn product() {
    let mut values = Vec::<u32>::with_capacity(20);
    for i in 0..20 {
        values.push(i + 1);
    }

    let n1: BigInt = values.iter().product();
    assert_eq!(n1, bigint!("2432902008176640000"));

    let big_values = values.iter().map(|n| BigInt::new(*n)).collect::<Vec<_>>();
    let n2: BigInt = big_values.iter().product();
    assert_eq!(n2, bigint!("2432902008176640000"));
}

#[test]
fn sum() {
    let values = vec![u32::MAX; 10];

    let n1: BigInt = values.iter().sum();
    println!("{:?}", n1.to_string());
    assert_eq!(n1, bigint!("42949672950"));

    let big_values = values.iter().map(|n| BigInt::new(*n)).collect::<Vec<_>>();
    let n2: BigInt = big_values.iter().sum();
    assert_eq!(n2, bigint!("42949672950"));
}
