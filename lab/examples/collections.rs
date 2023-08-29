fn main() {
    // https://doc.rust-lang.org/std/collections/index.html

    use std::collections::btree_map::BTreeMap;

    let mut count = BTreeMap::new();
    let message = "she sells sea shells by the sea shore";

    // SIMPLY A FANTASTIC API !!!
    for c in message.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    assert_eq!(count.get(&'s'), Some(&8));

    println!("Number of occurrences of each character");
    for (char, count) in &count {
        println!("{char}: {count}");
    }

    // A client of the bar. They have a blood alcohol level.
    struct Person {
        blood_alcohol: f32,
    }

    // All the orders made to the bar, by client ID.
    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    // Our clients.
    let mut blood_alcohol = BTreeMap::new();

    for id in orders {
        // If this is the first time we've seen this customer, initialize them
        // with no blood alcohol. Otherwise, just retrieve them.
        let person = blood_alcohol
            .entry(id)
            .or_insert(Person { blood_alcohol: 0.0 });

        // Reduce their blood alcohol level. It takes time to order and drink a beer!
        person.blood_alcohol *= 0.9;

        // Check if they're sober enough to have another beer.
        if person.blood_alcohol > 0.3 {
            // Too drunk... for now.
            println!("Sorry {id}, I have to cut you off");
        } else {
            // Have another!
            println!("{id} is having another one.");
            person.blood_alcohol += 0.1;
        }
    }

    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};

    #[derive(Debug)]
    struct Foo {
        a: u32,
        b: &'static str,
    }

    // we will compare `Foo`s by their `a` value only.
    impl PartialEq for Foo {
        fn eq(
            &self,
            other: &Self,
        ) -> bool {
            self.a == other.a
        }
    }

    impl Eq for Foo {}

    // we will hash `Foo`s by their `a` value only.
    impl Hash for Foo {
        fn hash<H: Hasher>(
            &self,
            h: &mut H,
        ) {
            self.a.hash(h);
        }
    }

    impl PartialOrd for Foo {
        fn partial_cmp(
            &self,
            other: &Self,
        ) -> Option<Ordering> {
            self.a.partial_cmp(&other.a)
        }
    }

    impl Ord for Foo {
        fn cmp(
            &self,
            other: &Self,
        ) -> Ordering {
            self.a.cmp(&other.a)
        }
    }

    let mut map = BTreeMap::new();
    map.insert(Foo { a: 1, b: "baz" }, 99);

    // We already have a Foo with an a of 1, so this will be updating the value(just the VALUE!).
    map.insert(Foo { a: 1, b: "xyz" }, 100);

    // The value has been updated...
    assert_eq!(map.values().next().unwrap(), &100);

    // ...but the key hasn't changed. b is still "baz", not "xyz".
    assert_eq!(map.keys().next().unwrap().b, "baz");
}
