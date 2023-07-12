fn main(){
    //create a vector
    let v = vec![1,2,3,1,4,9,445,23,53,2,35,3,224,3,4,4,5,6,264,1,9,0,13,4,44,134];
    //output result
    println!("mediana = {}, moda = {}",median(&v),moda(&v));
    println!("{}",pig_lat(String::from("hello world i am Artem")));
}

//function that find a median of values in vector in descending order
fn median(vec : &Vec<i32>) -> f32{
    //clone our vector
    let mut v = vec.clone();
    //sort our vector 
    v.sort();
    //count parity of length of vector
    let  s = &v.len() % 2;
    //declare a value of mediana index
    let mut r;
    //create mutable value for madiana
    let mut  mediana = 0.0;

    match  s {
        //if the length is not even
        1 => {r = (&v.len() - 1)/2; mediana = (v[r] as f32)/1.0;}
        //if the length is even
        0 => {r = &v.len()/2; mediana = ( v[r] + v[r - 1]) as f32 / 2.0;}
        _ => {}
    }
    //return result
    mediana
}


//function that count a moda of values in vector
fn moda(vec :&Vec<i32>) -> i32{
    use std::collections::HashMap;//use HashMap
    let mut v = vec.clone();//clone vector
    let mut map = HashMap::new();//declare a new HashMap
    //iterate on the cloned vector
    for i in &v{
        //if we have the key - i, then score will be value by key, else score value will be -1
        let score = map.get(i).copied().unwrap_or(-1);
        //if we don't have the key - i
        if score == -1{
            //create new element
            map.insert(i, 1);
            continue;
        }
        //if we have value by key - i, then we add 1 to the value
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    //find a max value in HashMap
    let mut max_key = 0;
    let mut max_val = -100;
    //iterate on HashMap and compare the max value with current value
    for (key,value) in map{
        if value > max_val{max_val = value;max_key = *key;}
    }
    //return result
    max_key
    }


//function that change string to pig latine
    fn pig_lat(s : String) -> String{
        //create a array of vowel letters
        let vowel = ["A","a","E","e","I","i","O","o","U","u","Y","y","А","а","Е","е","Ё","ё","И","и","О","о","У","у","Ы","ы","Э","э","Ю","ю","Я","я"];
        //clone our string
        let st = s.clone();
        //split our string by whitespaces
        let mut r = st.split_whitespace();
        //iterate on r
        for  mut t in r{
            if vowel.contains(&&t[0..1]){//if the array of vowel letters contains first letter ... 
                                         //... in string, then we change our string
                t = &format!("{t}-hay");//
                continue;
            }
            else{
                //change outr string if first letterd is not vowel
                let mut leng = &t.len();
                let mut string_fin = &t[1..*leng];
                let m = &t[0..1];
                t = &format!("{string_fin}-{m}ay");
                continue;
            }
        }
        println!();

        String::from("1")

    }




