use std::io;
fn example1 () {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [2, 3];
    let mut check_arr = false;
    let mut count = 0;

    for arr in org_arr  {

        if sub_arr[count] == arr{
            count = count + 1;
            if count == sub_arr.len() {
                check_arr = true;
                break;
            }
        } else {
            count = 0;
        }
        
    }

    println!("{}", check_arr);
}

fn example2() {
    loop {
        let str = "abacDacbb";
        let mut line = String::new();

        println!("Nhap 1 so: ");
        
        io::stdin().read_line(&mut line).unwrap();
        line.pop();

        let mut new_str = String::new(); 
        let mut count = 0;

        let input_line: Vec<char> = line.chars().collect();
        let input_str:Vec<char> = str.chars().collect();

        for i in 0.. input_str.len() {
            if input_line[0] == input_str[i] {
                count = count + 1;
            } else {
                new_str.push(input_str[i]);
            }
        }


        println!("{}", count);
        println!("{}", new_str);
    }
}
fn main() {
    example1();
    example2()
}