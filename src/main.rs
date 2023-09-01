use std::collections::HashMap;
use std::fmt;
use rand::Rng;
use rand::seq::SliceRandom;
extern crate rand;


struct  Init {
    constant_value: String, //bitvec
    output: i32, 
}

struct Load {
    space: usize,
    output: i32,
    endianness: usize,
    address: i32,
    access_size: usize,
    stmt_no: usize,
}


struct Store {
    space: usize,
    value: i32,
    endianness: usize,
    address: i32,
    access_size: usize,
    stmt_no: usize,
}

enum ValueExpr {
    Load(Load),
    Store(Store),

}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Store(space={}, value={}, endianness={}, address={}, access_size={}, stmt_no={})", self.space, self.value, self.endianness, self.address, self.access_size, self.stmt_no)
    }
}
impl fmt::Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Load(space={}, output={}, endianness={}, address={}, access_size={}, stmt_no={})", self.space, self.output, self.endianness, self.address, self.access_size, self.stmt_no)
    }
}
impl fmt::Display for Init {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Init(constant_value={}, output={})", self.constant_value, self.output)
    }
}

impl fmt::Display for ValueExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueExpr::Load(load) => write!(
                f,
                "Load(space={}, output={}, endianness={}, address={}, access_size={}, stmt_no={})",
                load.space, load.output, load.endianness, load.address, load.access_size, load.stmt_no
            ),
            ValueExpr::Store(store) => write!(
                f,
                "Store(space={}, value={}, endianness={}, address={}, access_size={}, stmt_no={})",
                store.space, store.value, store.endianness, store.address, store.access_size, store.stmt_no
            ),
            
        }
    }
}

fn shuffle<T>(array: &mut [T]) {
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut delete_stmt = Vec::new();
    let mut store_map = HashMap::new();
    let mut store_map_stmt_addres = HashMap::new();
    let mut load_map_stmt_addres = HashMap::new();
    let mut indexes_to_remove = Vec::new();
    let mut load_list = Vec::new();
    let mut store_list = Vec::new();
    let mut init_list = Vec::new();
    let mut count_list = Vec::new();
    let mut value_init = Vec::new();
    let mut delete_load_stmt: Vec<usize> = Vec::new();
    let mut indexes_to_remove_load: Vec<usize> = Vec::new();
    let mut instructions: Vec<ValueExpr> = Vec::new();
    // let mut init_load_Synergy = Vec::new();
    // let mut init_store_Synergy = Vec::new();


    // Функция для чтения ввода и преобразования его в целое число

    fn read_input() -> usize {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().parse().expect("Invalid input")
    }
    
    println!("Введите кол-во операций Store:");
    let store_count = read_input();
    println!("Введите кол-во операций Load:");
    let load_count = read_input();

    let mut count = store_count + load_count;

    for i in 0..count{
        count_list.push(i); 
    }

    shuffle(&mut count_list);

    // Инициализируем constant_value INIT
    for i in 0..count{
        value_init.push(format!("{:#010x}",rng.gen_range(0..=16)));
    }

    // Инициализация параметров каждой операции Init и добавление её в спикок init_list.
    let mut generated_values_init = Vec::new();
        for i in 0..count {
            let mut output = rng.gen_range(0..20);
           
            let constant_value = &value_init[i];

            while generated_values_init.contains(&output) {
                output = rng.gen_range(0..20);
            }
            generated_values_init.push(output);
            init_list.push(Init {
                constant_value: constant_value.to_string(),
                output,
                
            });
        }

    // Строим по Init Карту HashMap<Var,BitVec> 
    let mut HashInit = HashMap::new();
    for i in &init_list {
        HashInit.insert(i.output,i.constant_value.to_string() );
    }
    
        println!("Init HashMap: {:?}", HashInit);
    


    println!("----------------------------------------------------");

    for i in 0..2*load_count{
        println!("Init List: {}", init_list[i]);
    }
    println!("----------------------------------------------------");

    // Инициализация параметров каждой операции Store и добавление её в спикок store_list.
    let mut generated_values_store = Vec::new();
    for i in 0..store_count {

    let space = 0;
    let mut value = rng.gen_range(0..10);

    while generated_values_store.contains(&value) {
        value = rng.gen_range(0..10);
    }

    generated_values_store.push(value); 
    let endianness = 0;
    let address = rng.gen_range(0..20);
    let access_size = 4;
    let stmt_no = count_list[i];
    store_list.push(Store {
        space,
        value,
        endianness,
        address: address,
        access_size,
        stmt_no,
    });
    }

    store_list.sort_by_key(|x| x.stmt_no);

    for i in 0..store_list.len(){
        println!("Store List: {}", store_list[i]);
    }

    let mut cplusplus = store_list.len();

// Инициализация параметров каждой операции Load и добавление её в спикок load_list.
    let mut  generated_values_load = Vec::new();

for i in 0..load_count {

    let space = 0;
    let mut output = rng.gen_range(0..10);
    while generated_values_load.contains(&output) {
        output = rng.gen_range(0..10);
    }
    generated_values_load.push(output);

    let endianness = 0;
    let address = rng.gen_range(0..20);
    let access_size = 4;
    let stmt_no =count_list[cplusplus + i];
    load_list.push(Load {
        space,
        output,
        endianness,
        address: address,
        access_size,
        stmt_no,
    });
}
    load_list.sort_by_key(|x| x.stmt_no);

    println!("----------------------------------------------------");
    for i in 0..load_list.len(){
        println!("Load List: {}", load_list[i]);
    }
    println!("----------------------------------------------------");

    // //HashMap связи Init и Load
    // for init in 0..init_list.len(){
    //     for load in 0..load_list.len(){
    //         if load_list[load].stmt_no == init_list[init].stmt_no{
    //         init_load_Synergy.push((load_list[load].address.to_string(), init_list[init].constant_value.to_string()));
    //         }
    //     }
    // }
    // //HashMap связи Init и Store
    // for init in 0..init_list.len(){
    //     for store in 0..store_list.len(){
    //         if store_list[store].stmt_no == init_list[init].stmt_no{
    //         init_store_Synergy.push((store_list[store].address.to_string(), init_list[init].constant_value.to_string()));
    //         }
    //     }
    // }

    // //address на constant_value
    // for (store, init) in store_list.iter_mut().zip(init_store_Synergy.iter()) {
    //     store.address = init.1.to_string();
    // }
    // for (load, init) in load_list.iter_mut().zip(init_load_Synergy.iter()) {
    //     load.address = init.1.to_string();
    // }

    // for i in 0..load_list.len(){
    //     println!("load/init List: {:?}", init_load_Synergy[i]);
    // }    

    println!("----------------------------------------------------");

    // for i in 0..load_list.len(){
    //     println!("store/init List: {:?}", init_store_Synergy[i]);
    // }
    // println!("----------------------------------------------------
    // ");

    //Связь Load/Store с Init:              !!!!

    for load in &mut load_list {
        let constant_value = HashInit.get(&load.address).unwrap();
        let hex_number = i32::from_str_radix(&constant_value[2..], 16).unwrap();
        load.address = hex_number;
    }
    for store in &mut store_list {
        let constant_value = HashInit.get(&store.address).unwrap();
        let hex_number = i32::from_str_radix(&constant_value[2..], 16).unwrap();
        store.address = hex_number;
    }
    //                                      !!!!


    for i in 0..load_list.len(){
        println!("Load List: {}", load_list[i]);
    }
    for i in 0..store_list.len(){
        println!("Store List: {}", store_list[i]);
    }

    let Optimize_Var = r#"Dead Store Elimination"#;
    // Оптимизация "Dead Store Elimination"
    println!("|||||||||| Оптимизация {} |||||||||||:
    ", Optimize_Var);

    for load in &load_list {
        load_map_stmt_addres.insert(load.address, load.stmt_no);
    }
    // for store_init in init_store_Synergy {
    //     for store in &store_list {
    //         if store.address == store_init.0 {
    //             store_map.insert(store_init.1, store.stmt_no);
    //         }
    //     }
    // }

    for store in &store_list {
        store_map_stmt_addres.insert(store.stmt_no, store.address);
    }

    for store in &store_list {
        store_map.insert(store.address, store.stmt_no);
    }



    'tens: for store in &store_list {
        if let Some(stmt_no) = store_map.get(&store.address) {
            if *stmt_no > store.stmt_no {
                if let Some(addr) = store_map_stmt_addres.get(&stmt_no) {
                    if let Some(value) = load_map_stmt_addres.get(addr){
                        if value < &store.stmt_no || value > stmt_no
                            {   
                                    println!(
                                    " !!! Замечен мёртвый Store с номером {}: он имеет тот же адрес что и у statemet {}",
                                    *stmt_no, store.stmt_no);
                                    println!("Тут всё ок, потому что номер load с тем же адресом не входит в диапазон индексов двух Store'ов:
                                    ");
                                        if delete_stmt.len() == 0 {
                                            delete_stmt.push(store.stmt_no);
                                        }
                                        else{
                                    for i in 0..delete_stmt.len(){
                                        if &delete_stmt[i] == stmt_no {
                                            delete_stmt.push(store.stmt_no);
                                        }
                                        else{
                                            delete_stmt.push(store.stmt_no);
                                        }
                                    }
                                }
                                continue 'tens; 
                            }
                        
                            if value > &store.stmt_no && value < stmt_no { 
                                println!("Между двумя Store {}, {} (store.stmt_no)  стоит Load {} (load.stmt) с тем же адресом ${}
                                " , stmt_no, store.stmt_no, value, addr);
                                continue 'tens; }
                        }
                    }   
                    if delete_stmt.len() != 0 {
                        println!(
                            " !!! Замечен мёртвый Store с statemet {}: а Load с тем же адресом не существует 
                            ", store.stmt_no);
                    }
                    println!(
" !!! Замечен мёртвый Store с statemet {}: он имеет тот же адрес что и у номером {}, а Load с тем же адресом не существует 
",*stmt_no, store.stmt_no);

                        if delete_stmt.len() == 0 {
                            delete_stmt.push(store.stmt_no);
                        }
                        else{
                    for i in 0..delete_stmt.len(){
                        if &delete_stmt[i] == stmt_no {
                            delete_stmt.push(store.stmt_no);
                        }
                        else{
                            delete_stmt.push(store.stmt_no);
                        }
                    }
                }     
            }
        }
    }


    for (i, store) in store_list.iter().enumerate() {
        for &stmt_no in &delete_stmt {
            if store.stmt_no == stmt_no && !indexes_to_remove.contains(&i) {
                indexes_to_remove.push(i);
            }
        }
    }

    if indexes_to_remove.len() != 0{
        println!("----------------------------------------------------");
    }

    for i in 0..indexes_to_remove.len(){
        println!("indexes__store_list_to_remove: {}", indexes_to_remove[i]);
    }

    for &index in indexes_to_remove.iter().rev() {
        store_list.remove(index);
    }
   

    println!("----------------------------------------------------");

    println!("Optimaze store:");
    for i in 0..store_list.len(){
        println!("Store List: {}", store_list[i]);
    }

   
    for store in &store_list {
        store_map_stmt_addres.insert(store.stmt_no, store.address);
    }
    
     
    println!("----------------------------------------------------");

    let Var_opti = r#"Common Subexpression Elimination"#;
    println!("||||||||| Оптимизация {} |||||||||
    ",Var_opti);



    'tens: for load in &load_list {
        if let Some(stmt_no) = load_map_stmt_addres.get(&load.address) {
            if *stmt_no > load.stmt_no {
                if let Some(addr) = store_map_stmt_addres.get(&stmt_no) {
                    if let Some(value) = load_map_stmt_addres.get(addr){
                        if value < &load.stmt_no || value > stmt_no
                            {   
                                    println!(
                                    "!!! Замечен мёртвый Load с номером {}: он имеет тот же адрес что и у statemet {}, а Store с тем же адресом не существует ",
                                    *stmt_no, load.stmt_no);
                                    println!("Тут всё ок, потому что номер load с тем же адресом не входит в диапазон индексов двух Store'ов:
                                    ");
                                        if delete_load_stmt.len() == 0 {
                                            delete_load_stmt.push(*stmt_no);
                                        }
                                        else{
                                    for i in 0..delete_load_stmt.len(){
                                        if &delete_load_stmt[i] == stmt_no {
                                            delete_load_stmt.push(*stmt_no);
                                        }
                                        else{
                                            delete_load_stmt.push(*stmt_no);
                                        }
                                    }
                                }
                                continue 'tens; 
                            }
                        
                            if value > &load.stmt_no && value < stmt_no { 
                                println!("Между двумя Load {}, {} (load.stmt_no)  стоит Store {} (store.stmt) с тем же адресом ${}
                                " , stmt_no, load.stmt_no, value, addr);
                                continue 'tens; }
                        }
                    }   
                    if delete_stmt.len() != 0 {
                        println!(
                            "!!! Замечен мёртвый Load с statemet {}: а Store с тем же адресом не существует 
                            ", load.stmt_no);
                    

                    }
                    println!(
"!!! Замечен мёртвый Load с номером {}: он имеет тот же адрес что и у statemet {}, а Store с тем же адресом не существует  
",*stmt_no, load.stmt_no);

                        if delete_load_stmt.len() == 0 {
                            delete_load_stmt.push(*stmt_no);
                        }
                        else{
                    for i in 0..delete_load_stmt.len(){
                        if &delete_load_stmt[i] == stmt_no {
                            delete_load_stmt.push(*stmt_no);
                        }
                        else{
                            delete_load_stmt.push(*stmt_no);
                        }
                    }
                }     
            }
        }
    }


    for (i, load) in load_list.iter().enumerate().clone() {
        for &stmt_no in &delete_load_stmt {
            if load.stmt_no == stmt_no && !indexes_to_remove_load.contains(&i) {
                indexes_to_remove_load.push(i);
            }
        }
    }
    if indexes_to_remove_load.len() != 0{
        println!("----------------------------------------------------");
    }
    for i in 0..indexes_to_remove_load.len(){
        println!("indexes__load_list_to_remove: {}", indexes_to_remove_load[i]);
    }
    for &index in indexes_to_remove_load.iter().rev() {
        load_list.remove(index);
    }
    
    println!("----------------------------------------------------");

    println!("Optimaze load:");
    for i in 0..load_list.len(){
        println!("Load List: {}", load_list[i]);
    }

    println!("----------------------------------------------------");
    
    println!("All optimaze:
    ");
    let new_count = load_list.len() + store_list.len();


    for i in load_list{
        instructions.push(ValueExpr::Load(i));
    }
    for i in store_list{
        instructions.push(ValueExpr::Store(i));
    }

    instructions.sort_by_key(|expr| match expr {
        ValueExpr::Load(load) => load.stmt_no,
        ValueExpr::Store(store) => store.stmt_no,
    });

    for i in 0..new_count{
        println!("List: {}
        ", instructions[i]);
    }

}