use rand::Rng;

#[derive(Debug)]
struct Civic(&'static str);

enum GovernmentType { Democratic, Authoritarian }

struct Government(&'static str, GovernmentType);

fn main() {
    let mut rng = rand::thread_rng();
    let mut civics = vec!(
        (Civic("Militarist"), Civic("Pacifist")), 
        (Civic("Materialist"), Civic("Spiritualist")), 
        (Civic("Egalitarian"), Civic("Authoritarian")), 
        (Civic("Xenophile"), Civic("Xenophobe"))
    );
    let primary = choose_civic(&mut rng, &mut civics);
    let fanatic = rng.gen_bool(0.75);
    let secondary = choose_civic(&mut rng, &mut civics);

    let mut govs = vec!("Democracy", "Oligarchy", "Dictatorship", "Imperial", "Corporate");
    let gov = choose_and_remove(&mut rng, &mut govs);

    if fanatic {
        println!("{}: Fanatic {}, {}", gov, primary.0, secondary.0)
    } else {
        let tertiary = choose_civic(&mut rng, &mut civics);
        println!("{}: {}, {}, {}", gov, primary.0, secondary.0, tertiary.0)
    }
}

fn choose_and_remove<R: Rng + ?Sized, T>(rng: &mut R, items: &mut Vec<T>) -> T {
    let i: usize = rng.gen_range(0, items.len());
    items.remove(i)
}

fn choose_civic<R: Rng + ?Sized>(rng: &mut R, civics: &mut Vec<(Civic, Civic)>) -> Civic {
    let pair = choose_and_remove(rng, civics);
    if rng.gen::<bool>() { pair.0 } else { pair.1 }
}
