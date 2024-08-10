#![allow(unused)]
#![forbid(unsafe_code)]
#![deny(clippy::enum_glob_use)]

#[derive(Debug)]
struct User {
    name: String,
}

fn find_user<'a, 'b>(x: &'a User, y: &'a User, z: &'b User, k: &'b User) -> &'b User {
    println!("x: {:p}", x);
    println!("y: {:p}", y);
    println!("z: {:p}", z);
    println!("k: {:p}", k);
    separate();

    k
}

fn separate() {
    println!("---------------------");
}

fn borrow_1() {
    let mut input = String::from("Merhaba dünya.");
    let borrow_value = &mut input;
    borrow_value.push_str(" Naber?");
    borrow_value.push_str(" İyilik sağlık çok şükür. Emoji: 😎");

    let another_borrow = &mut input;
    println!("another_borrow: {:?}", another_borrow);
}

fn main() {
    borrow_1();

    let ref4;

    {
        let user1 = User {
            name: "user1".to_string(),
        };
        let user2 = User {
            name: "user2".to_string(),
        };
        let user3 = User {
            name: "user3".to_string(),
        };

        let ref1 = &user1;
        let ref2 = &user2;
        let ref3 = &user3;
        ref4 = find_user(ref1, ref2, ref2, ref3);
        let ref5 = ref4;

        println!("user1: {:p}", &user1);
        println!("user2: {:p}", &user2);
        println!("user3: {:p}", &user3);
        separate();

        println!("ref1: {:p}", ref1);
        println!("ref2: {:p}", ref2);
        println!("ref3: {:p}", ref3);
        println!("ref4: {:p}", ref4);
        println!("ref5: {:p}", ref5);
        separate();

        println!("ref1: {:p}", &ref1);
        println!("ref2: {:p}", &ref2);
        println!("ref3: {:p}", &ref3);
        println!("ref4: {:p}", &ref4);
        println!("ref5: {:p}", &ref5);
    }

    separate();
    lifetime_example();
}

#[derive(Default)]
struct Foo {
    pub name: String,
}

impl Foo {
    pub fn new() -> Self {
        Foo {
            name: "".to_string(),
        }
    }

    pub fn from_name(name: String) -> Self {
        Foo { name }
    }

    pub fn from_name_str(name: &str) -> Self {
        Foo {
            name: name.to_string(),
        }
    }
}

fn lifetime_example() {
    println!("Lifetime example starting...");

    let f1 = Foo::new();
    let f2 = Foo::from_name_str("test");

    //drop(f1);
    // Borrow checker is not move operation. When we move a value to another
    // variable then already all references will be invalid. This isn't our
    // problem. Our problem is only about borrow checker, not about move.
    //let f3 = f1;

    let f1_ref1 = &f1;
    let f2_ref2 = &f2;

    // ref'leri drop edemezsin, ownerları drop edebilirsin.
    //drop(f1_ref1);

    let result_ref1 = lifetime_handle_1(&f1, &f1, &f1);
    let result_ref2 = lifetime_handle_1(&f2, &f2, &f1);
    let result_ref3 = lifetime_handle_1(&f1, &f2, &f1);

    println!("f1_ref1: {:p}", f1_ref1);
    println!("f1_ref2: {:p}", f2_ref2);
    println!("result_ref1: {:p}", &result_ref1);
    println!("result_ref2: {:p}", &result_ref2);
    println!("result_ref3: {:p}", &result_ref3);
}

fn lifetime_handle_1<'a>(param1: &Foo, param2: &'a Foo, param3: &Foo) -> &'a Foo {
    //
    param2
}

fn lifetime_handle_2<'a>(param1: &Foo, param2: &'a Foo) -> Option<&'a Foo> {
    if param1.name.len() > param2.name.len() {
        Some(param2)
    } else {
        None
    }
}

/*
Normalde bir fonksiyondan ref dönderemezsin zaten. Çünkü bir fonksiyondan owned val
dönderebilirsin sadece. Eğer ref dönderiyorsan bu parametreden gelen reflerden birisi
olmak zorunda. Fonksiyonda owned olmuş bir değer fonksiyon bittiğinde kaybolur.
Dolayısıyla şu kurallarla çelişecek şekilde yazılmış olan fonkisyonlardan ref dönderilemez:
    1- Hiçbir parametre almayan fonkisyondan ref dönderilemez.
    2- Parametrelerinin hepsi ownerlı olan fonksiyondan ref dönderilemez.
    3- Parametredeki refler ile dönüşteki refin türü farklı olduğunda o ref dönderilemez.

Neden bu kurallar var? Çünkü null pointer dönderme problemini engellemenin en iyi yolu bu.
Eğer böyle olmazsa mecburen null döndermek gerekirdi ki amaç zaten bunu engellemek.
Galiba rust dalgasını öğrenmeden önce temel C bilgisini de iyice oturtmam gerekiyor.
Çünkü rustın olayı C'deki problemleri gidermek olduğu için önce C hakkında bilgi
edinip oradaki problemleri görmek. Problemleri gördükten sonra rustın bunlara getirdiği
çözümleri daha kolay anlayabiliriz inş.

Fonksiyonu tanımlarken hangi ref'i döndereceğimizi açıkça belirtmemiz gerekiyor. Tabi
burada önemli bir durum var, o fonksiyonun parametredeki reflerden birini dönderecekse
böyle birşey yapılmalı. Fonksiyon ref alıp owner dönderiyorsa o zaman lifetime specifier
yazmamıza gerek kalmaz (yani lifetime ellision hala mevcut), ref almadan ref zaten
dönderemez (çünkü sadece owner dönderebilir). Vay be. Çok ilginç ya.

*/
// Aşağıdaki fonksiyon 1. kurala göre geçersizdir.
//fn lifetime_handle_3<'a>() -> &'a Foo {
//    let param1 = Foo::default();
//    &param1
//}

// Aşağıdaki fonksiyon 2. kurala göre geçersizdir.
//fn lifetime_handle_4<'a>(param1: Foo, param2: Foo) -> &'a Foo {
//    &param2
//}

// Aşağıdaki fonksiyon 3. kurala göre geçersizdir.
//fn lifetime_handle_5<'foo, 'bar, 'baz>(param1: &'foo Foo, param2: &'bar Foo) -> &'baz str {
//    param2.name.as_str()
//}

// 3. kuralı şu şekilde valid hale getirebiliriz:
fn lifetime_handle_5<'foo, 'bar>(param1: &'foo Foo, param2: &'bar Foo) -> &'bar str {
    param2.name.as_str()
}

// Aşağıdaki fonksiyonda lifetime specifier yazılmaz (ama lifetime ellision hala mevcuttur)
// çünkü dönüş türü owner (yani ref değil).
fn lifetime_handle_4(param1: &Foo, param2: &Foo) -> Foo {
    Foo {
        name: param2.name.clone(),
    }
}

/*
Sonuç olarak aga eğer mutlaka parametredeki reflerden biri dönderilecekse parametredeki
lifetime specifierların hangisinin uzun veya kısa yaşadığının bir önemi yok, dönderilecek
olan refin lifetime... döndürülme
ihtimali olan parametrelerin lifetime specifierıyla returndeki refin lifetime
specifierı aynı olmalı.

null döndürülme ihtimalinden dolayı mı acaba? Olabilir ha. Ama hala böyle birşey için
lifetime specifier gerekli olup olmadığını anlayabilmiş değilim. Gerçi bi dakka ya
anlamıştım lan ben onu. Hah tamam, hangi refin döneceğini biliyor olmamız gerekiyordu.
Bu bir koruma önlemiydi, null dönmesin diye. Peki null nasıl dönebilir ki?
Parametredeki refler zaten hepsi valid, dolayısıyla dönen değer de valid olur
otomatik olarak. Hay sikiyim amk yine kafam karıştı.

*/

/*
The borrow checker relies on lifetimes and their constraints. The compiler can't
guarantee the references are valid without lifetimes.

Bak adam ne diyor. Demekki aga borrow checker bizim lifetime specifier'larımıza
göre çalışıyor. Bu yüzden derleyici bize borrow checkerları kullanmayı
mecbur bırakıyor. Böylece hem derleyici herşeyi kendisi çıkarsamak zorunda
kalmıyor hem de yazılımcıya esneklik kazandırmış oluyor. Dili neden böyle
tasarladıklarını şimdi daha iyi anladım galiba ve çok şükür kafamda herhangi
bir soru işareti kalmadı.

Benim düştüğüm hata ise borrow checker ile ownership (yani move) işlemini
birbirine karıştırmış oluşum. Çünkü ben şöyle düşünüyordum "zaten ownership
check var daha neden ekstradan lifetime specifier tanımlıyoruz ki?"
Halbuki konumuz ownership check değil borrow check. Yani aslında olay
ownershiple ilgili değil, borrowla ilgili. Çünkü ownership değerin kendisini
kontrol eder ama borrow referansları kontrol eder. Yani bu ikisinin uğraştığı
konular farklı.

İşte ingilizceyi iyi anlayamamanın sonucu amına koyim. Ownership başkaaaa
borrowing başka. Kafanı sikiyim. Asıl problem sendeymiş yani.

Borrowing & Ownership. İşte bütün mesele bu ikisini doğru anlamakta. Ha gerçi
dökümanlarda "borrow checker relies on lifetime and their constraints" tarzında
açıklayıcı birşey yazmıyor. Hiçbir açıklama yapmadan doğrudan lifetime specifier
kullanımını gösteriyor. Ulan amına koyim yazsana adam gibi niye böyle birşeyin
var olduğunu, derleyicinin bu lifetime specifierlara baktığını falan. Kafanızı
sikiyim sizin. Neyse aga mevzuyu anladık en azından çok şükür.

Bir de çılgınlar gibi C diliyle yazılmış örnekler veriyorlar, bu lifetime
mevzusunun ne kadar önemli olduğuyla ilgili. Lifetime olayı pointerlarla
ilgili. Rustta pointer yok ref var ama aslında onlar da pointer. Bütün amaç
pointerın her zaman valid bir memory adresini göstermesi. Eğer C gibi bir
dil kullanıyorsan bunu sağlamak tamamen yazılımcıya düşer ve çoğu zaman
bu mümkün olmaz. Ama rustta bu mevzu bazı kurallara bağlı kalınarak güvence
altına alınmış. Bundan dolayı rustta tüm referanslar valid bir memory adresini
işaret eder her zaman.

---

Stream mux is means "stream multiplex". Multiplexing means that one connection
is using with different protocols and apps. Multiplexer is creating virtual
connection over the current connection and sending streams over these
virtual connections. I can imagine what's going on behind the scenes. No problem.

*/
