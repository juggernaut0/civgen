use std::collections::HashSet;

use rand::Rng;

#[derive(Debug, PartialEq)]
struct Ethic(&'static str);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Government(&'static str);

impl Ethic {
    fn get_govs(&self, fanatic: bool) -> HashSet<Government> {
        let mut govs = HashSet::new();
        if self.0 == "Egalitarian" {
            govs.insert(Government("Democratic"));
            if !fanatic {
                govs.insert(Government("Oligarchic"));
                govs.insert(Government("Corporate"));
            }
        } else if self.0 == "Authoritarian" {
            govs.insert(Government("Dictatorial"));
            govs.insert(Government("Imperial"));
            if !fanatic {
                govs.insert(Government("Corporate"));
            }
        } else {
            govs.insert(Government("Democratic"));
            govs.insert(Government("Oligarchic"));
            govs.insert(Government("Dictatorial"));
            govs.insert(Government("Imperial"));
            govs.insert(Government("Corporate"));
        }
        govs
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut ethics = vec!(
        (Ethic("Militarist"), Ethic("Pacifist")), 
        (Ethic("Materialist"), Ethic("Spiritualist")), 
        (Ethic("Egalitarian"), Ethic("Authoritarian")), 
        (Ethic("Xenophile"), Ethic("Xenophobe"))
    );
    let primary = choose_ethic(&mut rng, &mut ethics);
    let fanatic = rng.gen_bool(0.75);
    println!("{}", fanatic);
    let secondary = choose_ethic(&mut rng, &mut ethics);

    if fanatic {
        let valid_govs = (&primary.get_govs(true) & &secondary.get_govs(false)).into_iter().collect();
        let gov = choose(&mut rng, &valid_govs);
        println!("{}: Fanatic {}, {}", gov.0, primary.0, secondary.0)
    } else {
        let tertiary = choose_ethic(&mut rng, &mut ethics);
        let valid_govs = (&(&primary.get_govs(false) & &secondary.get_govs(false)) & &tertiary.get_govs(false)).into_iter().collect();
        let gov = choose(&mut rng, &valid_govs);
        println!("{}: {}, {}, {}", gov.0, primary.0, secondary.0, tertiary.0)
    }
}

fn choose<'a, R: Rng + ?Sized, T>(rng: &mut R, items: &'a Vec<T>) -> &'a T {
    let i: usize = rng.gen_range(0, items.len());
    &items[i]
}

fn choose_and_remove<R: Rng + ?Sized, T>(rng: &mut R, items: &mut Vec<T>) -> T {
    let i: usize = rng.gen_range(0, items.len());
    items.remove(i)
}

fn choose_ethic<R: Rng + ?Sized>(rng: &mut R, ethics: &mut Vec<(Ethic, Ethic)>) -> Ethic {
    let pair = choose_and_remove(rng, ethics);
    if rng.gen::<bool>() { pair.0 } else { pair.1 }
}
