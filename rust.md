extensao .rs

rustc --version (saber a versao do rust)
rustup update (procura atualizacoes no rust)

.\[nome do projeto] (no terminal, executa o programa criado)

tem que ter funcao "main" para poder executar
tem que ter aspas duplas se for para representar uma string
se for caractere, pode ser aspas simples
usar ponto e virgula nas funcoes...se nao usar, sera considerada como o 'return', sem precisar digitar

digitar cargo new [nome do projeto] cria um novo projeto

cargo build (cria a 'producao' nao otimizada do programa - modo debug)
dentro da pasta target/debug e so digitar o .\[nome do projeto] para rodar o programa

cargo build --release (versao otimizada para producao e distribuicao)
dentro da pasta target/release e so digitar o .\[nome do projeto] para rodar o programa

cargo run (roda o programa)
cargo fmt (formata o programa para o padrao rust, caso nao esteja)

cargo install cargo-watch (baixa ferramenta para 'escutar' as modificacoes instantaneamente do programa)
cargo watch -x run para usar a ferramenta

nao se usa let no escopo global, pode usar dentro de funcoes
quando definir uma variavel, definir que tipo ela sere. ex: i32
se voce quiser ignorar alguma variavel no progrmama, usar _. ex: let _total:i32 = 30;
se usar constante, colocar em uppercase e snakecase. ex: const SECONDS_IN_MINUTE:i32 = 60;

println! é uma macro. Podemos usar para interpolar. ex: println!("Trabalhou {} horas", TOTAL);
o compilador as vezes ja reconhecer o tipo da variavel criada...se ele reclamar, declarar para corrigir o erro

toda variavel em rust, por padrao, é imutavel...tem que digitar mut antes da variavel para poder modificar, desde que seja do mesmo tipo. ex: let mut total = 30;

se quiser modificar o tipo (variable shadowing), pode fazer assim: 
    let total = 30; (nao precisa ter o mut)
    let total = "trinta";

podemos fazer um subescopo usando {}, dentro da funcao, ou criando uma outra funcao dentro da funcao original; dentro desse subescopo, podemos acessar variaveis da funcao original;

o tipo u32 nao tem valores negativos ('u' de unsigned?)

em rust nao existe null

tipos primitivos: escalares e compostos

escalares: representam um unico valor contido em uma escala conhecida; permitem a comparacao direta entre valores. ex: inteiro, flutuante, booleano, caractere (char) (ex: letra, letra japonesa, emoji)
existem varios tipos de inteiros: signed - armazena sinal (i) e unsigned - sempre positivo(u), 8,16,32,64,128 bits e isize, usize (variam conforme arquitetura); isso é importante para evitar alocacao desnecessaria de memoria
se nao declarar o tipo do inteiro, o compilador assume que sera i32
numeros grandes podem ser colocados _ para separar as casas. ex: 1_000_000_000
hexadecimal. ex: let h = 0x[restante]; 0xff;
octal. ex: let o = 0o[restante]; 0o77;
binario. ex: let b = 0b[restante]; let b = 0b1111_0000;
byte. ex: letby = b[restante]; b'A';
o tipo flutuante é representado por padrao como f64; ex: let x = 41.2; tambem existe o f32
o booleano é representado pelo bool
caractere é representado por char. cabe 4 bites da tabela unicode nele

compostos: servem para agregar multiplos valores; ex: tupla (5, true, 54.9, "ola"), array [1,2,3,4]
a tupla tem tamanho fixo e tipagem heterogenea. Se quiser substituir, tem que trocar o elemento da posicao pelo mesmo tipo. ex: string por string.
mesmo que a tupla seja mutavel, nao da pra aumentar, nem diminuir ela.
conseguindo printar a tupla pelo "modo debug". 
    ex: 
    let numbers:(i32, i32, i32) = (1,2,3)  //declara os tipos se quiser
    println!("{:?}", numbers.0)  //busca o elemento na posicao zero

as tuplas podem ser desestruturadas pelo pattern match.
ex: let (a,b,c) = numbers

os arrays tem que ter os mesmos tipos. eles ficam na memoria stack.
let numbers:[i32;3] //declarando um array de inteiros com 3 elementos, acho que nao é obrigatorio declarar = [1,2,3]
slice:  println!("{:?}", &array[1..3]); pegar do indice 1 ao 2 (colocar um a mais)
slice:  println!("{:?}", &array[..3]); 
slice:  println!("{:?}", &array[1..]); 

'' = caractere. char
"" = string literal; fica na memoria estatica; str (nao tem tamanho fixo); &str = string slice
a tabela unicode tem 7 bits em cada posicao. em hexadecimal o valor muda.

String é alocado na memoria heap (memoria dinamica), para que possa aumentar, caso necessario. Chamada tambem de String dinamica ou string heap.

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
    let mut v = String::from(""); //string owned
    println!("{v}");
    let mut a = "Neto".to_owned(); //string owned. metodo considerando o ownership
    println!("{a}");

    let nome = ['L','u','i','s','a'];
    let s = String::from_iter(nome);
    println!("{s}");

    let r:String = "Augusto".into(); //tem que ter a declaracao pra poder funcionar
    println!("{r}");

use std::io;  // importa uma biblioteca padrao do rust para receber informacoes de input

funcao se nomeia com snake case. ex: fn say_hello() {}
podemos reaproveitar as funcoes quantas vezes quisermos
funcao é importante no gerenciamento de alocacao de memoria
pra ter ownership, borrowing e checagem de lifetime tem que ter funcao
podemos passar funcoes e receber funcoes para outras funcoes
importante que elas sejam parametrizadas (seja definido um parametro entre os parenteses). ex: fn say_hello(name: &str) {}
ao parametrizar, somos obrigados a definir o tipo da variavel
parametro é o nome da variavel que colocamos na funcao e argumento é o valor que colocamos ao usar a funcao
nao é permitido declarar um parametro e nao usa-lo (opcional). Tem que usar sempre
para usarmos parametros default, temos que parametrizar usandos objetos (struct) ou macros.
podemos usar objetos (estruturas / struct?) como parametro de uma funcao. fn say_hello(person: Person) {}
em rust, os argumentos sao posicionais, temos que colocar os argumentos ao usar a funcao na sequencia em que eles sao declarados.
se quisermos ter parametros nomeados, temos que usar struct ou macro
statement é uma delcaracao de algo, so que nao produz nenhum valor de retorno; let x = 5
expressoes sao todos os elementos que esperamos receber um valor de retorno
expressao literal sao expressoes. ex: true, 5, "bruno".
chamada de uma macro ou funcao ou operacao aritimetica ou expressao de comparacao tambem é expressao
ex: let x = vec![]; let x = say_hello("Bruno"); let x = 5 * 5; let x = 10 > 5;
pegar um bloco de codigo dentro de uma funcao (subescopo) e nomea-lo tambem é uma funcao.
qualquer bloco de codigo em rust retorna um unit type ()
toda vez que tivermos um retorno na funcao temos que anotar a assinatura. ->. ex: fn add_numbers(x:i32, y:i32) ->
funcao pode ser declarada com especificacao lifetime e tradebounds?
a ultima linha da funcao sera o que vai ser retornado. Se colocar ponto e virgula vai dar erro.

ex: fn add_numbers(x: i32, y: i32) -> i32 {
    x + y //nao é obrigatorio digitar o return aqui, pq uma expressao aritimetica ja retorna um valor
}

o return serve para guard clausules (clausulas guarda). usamos dentro de if, loops para encerrar a leitura da funcao que estiver para baixo do retorno. é chamado de early return, dá um block no resto da funcao, apos atender o requisito.

fn add_numbers(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y  // early return. se for true, encerra a leitura da funcao
    }
    x + y //nao é obrigatorio digitar o return aqui, pq uma expressao aritimetica ja retorna um valor
}

collect() : pega os resultados do map e coloca dentro da variavel declarada.
vetor é um array dinamico
podemos usar uma funcao anonima ou clousure em funcoes pequenas, dentro do map

um dos desafios na linguagem de baixo nivel é a gestao de memoria na memoria heap. no rust, temos o borrow checker, que resolve os seguintes problemas:
- se tiver problema de "pointer exception" - valor null, o programa nao compila. Chamado de problema de um bilhao de dolares. variavel que aponta para um local nulo na memoria.
- segmentation fault. quando tentamos derreferenciar uma variavel com valor nulo.
- memory leak. quando nao fazemos a gestao correta da memoria heap.
- dangling pointers. quando tentamos acessar um valor que ja foi limpo pela memoria, que nao existe mais
- double free. evita a limpeza da memoria duas vezes, desnecessariamente.
- use after free. evita problemas na gestao de limpeza de memoria (algumas linguagens permitem o programador fazer a propria gestao de limpeza de memoria)
- data races. acontece em linguagens de alto nivel. quando um valor mutavel esta sendo modificado por varios processos ao mesmo tempo, comprometendo a integridade.
rust nao tem garbage colletor, nem gestao de memoria manual.
fazemos a limpeza de memoria pela tipagem (semantinca OBRM - ownership, borrowship, resource management e semantica MOOB?)
o borrow checker tem uma linguagem de erro muito amigavel

no rust existe a categoria de tipo copy, que podem ser copiados automaticamente
os valores primitivos ficam na memoria stack. todos os valores primitivos escalares sao tipo copy.

let a:i32 = 1;
let b = a  // é criado uma copia com o valor de a, em um local diferente. Se o 'a' alterar, nao impacta no 'b'.

se usarmos o '&', estamos criando uma referencia, e nao uma copia.
let b = &a; // o valor é o mesmo de 'a' na memoria.

se quisermos imprimiro 'b', temos que derreferenciar:
println!("{}", *b) //tem que colocar esse asterisco

a String por padrao é 'no copy'
copiar valores na memoria heap custa caro. É a diferença do rust para outros tipos de linguagens.

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

regras de ownership:
- cada valor tem um dono;
- so existe um dono;
- quando o dono sai de escopo, o valor é limpo;
- a posse pode ser movida para outro dono.

o que esta na memoria stack nao esta submetida a regras de ownership
a maior parte das vezes queremos so fazer o borrowing (emprestimo) do valor

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
        println!("{text}")
    }

    to_uppercase(&mut nome);
    add_prefix(&mut nome);

regras de borrowing:
- podemos ter uma unica referencia caso ela seja mutavel
- podemos ter varias quando todas sao imutaveis

toda vez que chamamos o metodo de uma variavel, o rust faz a derefencia implicita, nao precisa usar '*'

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
    println!("{:?}", v)

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

    a funcao é representada por fn;
    a funcao main sempre deve existir e é a principal funcao do programa;
    se nomeia em snake_case;
    ela pode ter ou nao parametros e pode ou nao retornar algo;
    colocamos uma seta -> e depois o tipo, para especificar o tipo que sera retornado na funcao
    se quer retornar uma valor, nao colocar semicolon (ponto e virgula), ou colocar return
    nao existe parametro opcional em rust, tem que colocar todos

