fn main() {
    {
        let v = vec![1, 2, 3, 4, 5];

        let third = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        // v.push(6);

        println!("The first element is: {}", first);
    }

    {
        let mut v = vec![100, 32, 57];

        for i in &v {
            println!("{}", i);
        }

        for i in &mut v {
            *i += 50;
        }

        println!("{:?}", v);
    }

    {
        let mut s = String::new();

        let data = "initial contents";

        let s = data.to_string();

        let s = "initial contents".to_string();

        let s = String::from("initial contents");

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        s1.push('b');
        // let s3 = s1 + s2;

        println!("{:?} {:?}", s1, s2);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s = s1 + "-" + &s2 + "-" + &s3;
        let s = format!("{}-{}-{}", s1, s2, s3);

        println!("{:?}", s);
    }

    {
        let hello = String::from("Здравствуйте");
        let s = &hello[0..4];

        println!("{:?}", s);

        for c in "Здравствуйте".chars() {
            println!("{}", c);
        }
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        println!("The score for {} is {}", &team_name, score.unwrap());

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    {
        use std::collections::HashMap;

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let _scores = teams
            .into_iter()
            .zip(initial_scores.into_iter())
            .collect::<HashMap<_, _>>();
    }

    {
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();

        map.insert(field_name, field_value);

        // println!("{:?} {:?}", field_name, field_value);
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    {
        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
