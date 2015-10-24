#![feature(box_syntax)]

struct Mu<A> {
    unMu : Box<Fn(Mu<A>) -> A>,
}


// y f = (\h -> h $ Mu h) (\x -> f . (\(Mu g) -> g) x $ x)
fn fact9<F> (f: F) -> i32 
  where F: Fn(Mu<i32>)->i32 + 'static {

    let m = Mu{unMu:box f};
    5
}

fn fix<F, T> (f : &F) -> T
    where F: Fn(T) -> T + 'static {
        f (fix(f))
}

fn main(){
    println!("{}", fact2(3, fact));
    println!("{}", fact3(fact3(Box::new(fact)))(4));
    println!("{}", fix(&fact3)(5));
}

 
fn fact(x : i32) -> i32 {
    if x == 1 {1}
    else{x * fact(x-1)}
}

fn fact2<F>(x : i32, f:F) -> i32 
where F: Fn(i32)->i32
{
    if x == 1 {1}
    else{x * f(x-1)}
}


fn fact3(f : Box<Fn(i32)->i32>) -> Box<Fn(i32) -> i32>
{
    Box::new(move |x:i32| -> i32 {
        if x == 1{1}
        else{x * f(x-1)}
    })
}

