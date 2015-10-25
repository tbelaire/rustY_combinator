#![feature(box_syntax)]
fn fix< F: ?Sized> (f : &F, x : i32) -> i32
    where F: Fn(&Fn(i32)->i32, i32) -> i32
{
    let z :&Fn(i32)->i32= 
        &move |y:i32| -> i32 {
            fix(f, y)
        };
    f(z, x)
}

fn fact5(g : &Fn(i32) -> i32, x : i32) -> i32
{
    if x == 1 {1}
    else      {x * g(x-1)}
}
fn main(){
    let x = fix(&fact5, 5);
    println!("{}", x);
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


fn fix_unbounded<F, T> (f : &F) -> T
    where F: Fn(T) -> T + 'static {
        f (fix_unbounded(f))
}


fn fact3(f : Box<Fn(i32)->i32>) -> Box<Fn(i32) -> i32>
{
    Box::new(move |x:i32| -> i32 {
        if x == 1{1}
        else{x * f(x-1)}
    })
}

fn fact4(f : Box<(Fn(i32) -> i32)>) -> Box<Fn(i32) -> i32>
{
    Box::new(move |x:i32| -> i32 {
        if x == 1{1}
        else{x * f(x-1)}
    })
}


struct Mu<A> {
    un_mu : Box<Fn(Mu<A>) -> A>,
}


// y f = (\h -> h $ Mu h) (\x -> f . (\(Mu g) -> g) x $ x)
fn fact9<F> (f: F) -> i32 
  where F: Fn(Mu<i32>)->i32 + 'static {

    let m = Mu{un_mu:box f};
    5
}
