/* use std::io; */

//const TOTAL:i32 = 30;
const SECONDS_IN_MINUTE:u32 = 60;
const MINUTES_IN_HOUR:u32 = 60;
const SECONDS_IN_HOUR:u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {

    println!("Hello, world!");

    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;
    //let mut total = 30;
    //let _total:i32 = 30;

    {
        //total = 44;
        //let total = "trinta";
        let total = total + 20;
        println!("Trabalhou {} horas", total);
    }

    println!("Trabalhou {} horas", total_em_segundos); 

    tipos();
    say_hello("Jorge", "Azul");

    let subescopo = {
        say_hello("Camaro", "Amarelo")
    };
    println!("{:?}", subescopo); //qualquer bloco de codigo em rust retorna um unit type ()

    let aa = add_numbers(5, 10);
    println!("{aa}");
    let bb = add_numbers(0, 10);
    println!("{bb}");

    /* fn convert_to_number(s:&str) -> i32 {
        s.parse().unwrap()
    } */

    /* fn double(n:i32) -> i32 {
        n * 2
    } */

    let input = "56 77 23 88 19";
    let result: Vec<i32> = input
        .split(' ')
        /* .map(convert_to_number) //podemos usar uma funcao anonima ou clousure em funcoes pequenas, dentro do map
        .map(double) */
        .map(|s| s.parse/* ::<i32> */().unwrap())
        .map(|n:i32| n * 2)
        .collect();
    println!("{:?}", result);

    let aaa:i32 = 1;
    let bbb = aaa; //copy, local na memoria stack diferente de 'aaa'
    println!("{aaa}");
    let aaa = 2;
    println!("{bbb}"); //nao muda o valor

    let ccc = &aaa; //referencia de aaa; o valor esta no mesmo local da memoria stack de aaa
    println!("{}", /* * */ccc); //esse asterisco é usado para derreferenciar; no println! nao precisa

    let ddd = String::from("Joao");  //esta na memoria heap. ddd é dono da string. tem ownership.  ddd é dono de Joao.
    let eee = &ddd; //eee pegou emprestado (borrowing) o valor de ddd; nao pode mover se o valor tiver emprestado.
    println!("{eee}");
    let fff = ddd; //fff agora é o dono de Joao. ddd nao existe mais. a propriedade foi movida "move semantics"
    //println!("{ddd}");
    println!("{fff}");

    fn diz_oi(text:&str) {
        println!("Oi, {text}!")
    }

    fn diz_adeus(text:&str) {
        println!("Adeus, {text}!")
    }

    let name = "Bruno"; //tipo copy, esta na memoria stack, nao tem problema de ownership
    diz_oi(name);
    diz_adeus(name);

    let mut nome = "Renato".to_string(); //tipo no copy, esta na memoria heap, tem problema de ownership
    //diz_oi(nome.clone()); //pode duplicar o valor na memoria heap, mas tem que usar com moderacao, pq pode deixar menos performatico o sistema; tem ownership dessa forma
    diz_oi(&nome);
    diz_adeus(&nome); //sem o '&', nome seria movido de diz_oi; seria como se 'nome' em diz_oi nao existisse mais, por isso da erro... para corrigir, tem que pegar emprestado usando '&'

    fn to_uppercase(text:&mut String) {
        *text = text.to_uppercase(); //o * é usado para dereferenciar pq foi definido que é mutavel. as vezzes nao precisa usar *. mut borrow (emprestimo com poderes de alteracao)
        println!("{text}")
    }

    fn add_prefix(text:&mut String) {
        *text = format!("FOO_{text}"); // mut borrow (emprestimo com poderes de alteracao)
        //text.push_str("_FOO"); //adiciona sufixo. nao precisou do '*'
        println!("{text}")
    }

    to_uppercase(&mut nome);
    add_prefix(&mut nome);
    
    curso_udemy();
    imprimir_frase("frase impressa com sucesso!");
    println!("{}",gcd(20, 5));
    
    
}

fn tipos() {
    //let x = 5;
    let x:u8 = 5;
    println!("{}", x);
    
    let mut tupla = (1,'a',false);
    println!("{:?}", tupla);
    
    tupla.0 = 22;
    tupla.1 = 'b';
    tupla.2 = true;
    let (_a,_b,c) = tupla;
    println!("{:?}", tupla);
    println!("{:?}", c);

    let mut array/* :[i32;3] */ = [4,5,6];
    println!("{:?}", array);
    
    array[1] = 98;
    println!("{:?}", &array[1..3]); //slice
    println!("{:?}", &array[1..]); //slice apartir do elemento 1
    println!("{:?}", &array[..2]); //slice ate o elemento 2
    println!("{:?}", array[1]);

    let l0 = 't';
    println!("{l0}");
    let string/* :&str */ = "Verissimo"; //fica na memoria estatica, tamanho definido
    println!("{string}");

    let mut s = String::new(); //permite ser criada uma string de tamanho dinamico, na memoria heap
    s.push('L');
    s.push('a');
    s.push('i');
    s.push('s');
    println!("{s}");

    let mut p = String::new();
    p.push_str("Pedro");
    println!("{p}");

    let o = "Pedrão".to_string(); //cria uma String alocada na memoria heap
    println!("{o}");

    let u = String::from("Pedrinho"); //outra forma
    println!("{u}");
    let v = String::from(""); //string owned
    println!("{v}");
    let a = "Neto".to_owned(); //string owned. metodo considerando o ownership
    println!("{a}");

    let nome = ['L','u','i','s','a'];
    let s = String::from_iter(nome);
    println!("{s}");

    let r:String = "Augusto".into(); //tem que ter a declaracao pra poder funcionar
    println!("{r}");

    /* let mut texto = String::new();
    println!("Digite um texto:");

    io::stdin()
        .read_line(&mut texto)
        .expect("Deu ruim");
    
    println!("Voce digitou: {texto}"); */

}

fn say_hello(name:&str, color:&str) {
    println!("Hello {name}!! Your color is {color}");
    
}

fn add_numbers(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y  // early return. se for true, encerra a leitura da funcao
    }
    x + y //nao é obrigatorio digitar o return aqui, pq uma expressao aritimetica ja retorna um valor
}

fn curso_udemy() {

    //Vetores sao arrays que podem mudar de tamanho e que ficam na memoria heap. Sao muito uteis quando precisamos criar um array dinamico

    let mut nums = vec![1,2,3]; //criando um vetor atraves de macro
    nums.push(4);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new(); //criando um novo vetor zerado atraves do construtor
    vec.push("oi");
    vec.push("sumido");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec); // mostra a ordem inversa dos elementos

    let vect = Vec::<i32>::with_capacity(2); // definindo a capacidade de um vetor
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect(); // criando vetor com 5 elementos
    println!("{:?}", v);

    //slices sao regioes de arrays ou vetores; nao podem ser armazenados diretamente em variaveis, nem como argumento de funcoes. acredito que ela so pega emprestado o valor de determinada variavel.
    // slice nao fica alocada na memoria heap?
    let sv = &v;
    println!("{:?}", sv);
    // podemos pegar apenas um pedaço do valor
    let sv = &v[2..4];
    println!("{:?}", sv);

    //string sao similares os vetores e sao armazenadas como vetores de bytes...sempre tem a garantia de ser uma sequencia UTF-8 valida...é alocada na memoria heap, é global e não nula
    let name = String::from("Tyler");
    let course = "Rust".to_string();
    let new_name = name.replace("Tyler", "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    //string slice nao fica alocada na memoria heap
    //&str - string slice
    let str1 = "hello";
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    println!("{}", "ONE".to_lowercase() == "one");

    //string literal voce usa quando nao quer usar uma sequencia UTF-8
    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);


}

fn imprimir_frase(frase:&str){
    println!("{}", frase)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a!=0 {
        if a<b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

