use std::collections::HashMap;



fn main() {
    let mut map = HashMap::<&str, &str>::new();



    // INsertion
    /* 1) Replacing insertion */
    map.insert("somekey", "25");
    map.insert("somekey", "26");
    println!("1) Value after replacing-insertion: {}", map.get("somekey").unwrap());


    /* 2) Insert only new */
    map.entry("doeskey").or_insert("doesval");
    map.entry("somekey").or_insert("newkey");
    println!("2) Only new insertion : existing: {} | new key: {}",
             map.get("somekey").unwrap(), map.get("doeskey").unwrap());

    /* 3) Updating insertion */
    let sentence = "World has to have its heroes to be able to prolong and to thrive. Has to have";
    let mut wc = HashMap::new();

    for w in sentence.split_whitespace() {
        let c = wc.entry(w).or_insert(0);
        *c += 1;
    }
    println!("3) Updating insert: \n The sentence has such words: \n {:#?}", wc);
    
}