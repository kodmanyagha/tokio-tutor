// I made this constraint explicit, but if you had a field that looked
// like `&'short &'long ()` it would be implicitly present.
//
// If you didn't have to declare the lifetime, things like reordering fields
// would be a breaking change.
struct S<'long: 'short, 'short> {
    a: &'long str,
    b: &'short str,
}

// This only compiles because of the `'long: 'short` constraint on `S`
fn example<'a, 'b>(s: S<'a, 'b>) -> &'b str {
    s.a
}

fn example_2(ref1: &String) -> &String {
    ref1
}

fn check() {
    let forever = "hello";
    let local = "world".to_string();

    /*
    Burada ref1 ne kadar yaşamak zorunda? local kadar yaşamak zorunda. Çünkü yukarıda
    fonksiyona bu refi gönderdik ve artık onun yaşam zamanı o fonksiyondan dönen
    refin yaşam zamanına atandı. Bu atama işlemi derleme zamanında gerçekleşiyor.
    Bu yüzden derlendikten sonra ortaya çıkan kod hep valid oluyor zaten.

    Null döndüremesin diye olabilir mi? Zaten null diye birşey yok ki olim.
    Belki de C'yi iyice öğrenirsem bu lifetime specifier dalgasının tam olarak hangi
    problemi çözdüğünü daha iyi anlayabilirim. Gerçi şimdi oturup C öğrenmek
    çok gereksiz birşey.
    */
    let ref1 = example_2(&local);
    //let ref1 = &local;

    //drop(local);

    println!("{ref1}");

    let string1 = "".to_string();
    let s: &str = &string1; // Auto deref will be applied.
    println!("{s}");

    let s = S {
        //a: &*local,
        // Auto deref will be applied.
        a: &local,
        b: forever,
    };

    // Due to the constraints, the borrow checker understands that the API
    // of `example` allows returning either the borrow of `'local` or the copy
    // of `forever`
    let borrow = example(s);

    /* drop(local) dediğimiz zaman şöyle birşey oluyor, local yukarıda `'long` olacak
    şekilde işaretlendi. Hatta hem forever hem local eşit şekilde yaşıyor olmaları
    gerekiyor çünkü S isimli struct'ta biz referansların lifetimelarının eşit olması
    gerektiğini belirttik. Borrow checker da bizim oluşturduğumuz lifetime eşitliklerini
    kontrol ediyor. Son satırdaki println satırını silersek drop'taki hata ortadan kalkar,
    drop'u println'in altına alırsak yine hata ortadan kalkar. Çünkü borrow checker
    bizim belirttiğimiz lifetimeların aynı olup olmadığını kontrol ettiğinden dolayı
    local ile forever'ın aynı süre kadar yaşıyor olmaları gerekiyor.

    Lifetime specifierlar her zaman aynı olmak zorunda mı? Değil, reflerle çalışıyoruz ve
    yazılımcının esnekliğe ihtiyaç duyduğu durumlar olabilir. Aslında bu reflerle çalışma
    mevzusuna daha derinlemesine bakmam gerekiyor çünkü performansın önemli olduğu
    yerlerde refleri çok kullanmam gerekecek. Her yerde değerin aynısını klonlayamam.
    Bu performansı çok kötü etkiler aga. Bundan dolayı bol bol reflerle çalışmam
    gerekiyor dolayısıyla da bu lifetime specifier, borrow checker vs gibi mevzulara
    çok iyi hakim olmam gerekiyor.

    Borrow edilen değeri sonradan move yapamıyoruz anladığım kadarıyla. Yani, mantıklı.
     */
    //drop(local);

    // So it prevents using the return value here, after `local` has been
    // dropped.  The borrow checker used the lifetimes to prevent the creation
    // of an invalid reference
    println!("{borrow}");
}

/* This is example about static lifetime specifier. */
//fn alternatively() {
//    let forever = "hello";
//    let local = "world".to_string();
//
//    // If the constraints weren't upheld, this could compile
//    let s: S<'_, 'static> = S {
//        a: &*local,
//        b: forever,
//    };
//
//    // Which would let this compile as per the signature of `example`
//    let too_long: &'static str = example(s);
//    drop(local);
//
//    // Which would make this a use-after-free
//    println!("{too_long}");
//
//    // The borrow checker used the lifetimes to prevent the creation of an
//    // invalid reference
//}

// Here's the last example with some unsafe to defeat the borrow checker.
//
// You can comment out the other two examples and run it.
//
// The result is use-after-free (it crashed when I ran it, or alternatively
// you can run it with Miri under Tools, top-right).
fn main() {
    check();
    //alternatively();

    let forever = "hello";
    let local = "world..................".to_string();

    let s: S<'_, 'static> = unsafe {
        S {
            a: &*{ &*local as *const str },
            b: forever,
        }
    };

    let too_long: &'static str = example(s);
    drop(local);
    println!("{too_long}");
}

fn attempt_1() {
    let str1 = "test1".to_string();
    let str2 = "test2".to_string();
    let ref1 = &str1;
    let ref2 = &str2;
    println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);

    //println!("ref1: {}", ref1);
    let ref3 = lifetime_1(ref1, ref2);

    drop(str1);

    //println!("ref3: {}", ref3);
    println!("ref2: {}", ref2);
}

fn lifetime_1<'a>(ref1: &'a String, ref2: &'a String) -> &'a String {
    if ref1.len() > ref2.len() {
        ref1
    } else {
        ref2
    }
}
