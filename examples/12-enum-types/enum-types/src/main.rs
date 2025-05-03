#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    SeaOfGalilee,
    GolanHeights,
}
impl WineRegions {
    fn copy(&self) -> WineRegions {
        match self {
            WineRegions::Bordeaux => WineRegions::Bordeaux,
            WineRegions::Burgundy => WineRegions::Burgundy,
            WineRegions::Champagne => WineRegions::Champagne,
            WineRegions::Tuscany => WineRegions::Tuscany,
            WineRegions::Rioja => WineRegions::Rioja,
            WineRegions::NapaValley => WineRegions::NapaValley,
            WineRegions::SeaOfGalilee => WineRegions::SeaOfGalilee,
            WineRegions::GolanHeights => WineRegions::GolanHeights,
        }
    }
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}
fn region_popularity(w: WineRegions) {
    match w {
        WineRegions::Bordeaux => println!("Bordeaux is very popular!"),
        WineRegions::Burgundy => println!("Burgundy is also popular!"),
        WineRegions::Champagne => println!("Champagne is a classic!"),
        WineRegions::Tuscany => println!("Tuscany is known for its Chianti!"),
        WineRegions::Rioja => println!("Rioja is famous for its Tempranillo!"),
        WineRegions::NapaValley => println!("Napa Valley is known for its Cabernet Sauvignon!"),
        WineRegions::SeaOfGalilee => println!("Sea of Galilee is known for its unique wines!"),
        WineRegions::GolanHeights => println!("Golan Heights is known for its high-quality wines!"),
    }
}
fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Carmel"),
        region: WineRegions::GolanHeights,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    // clone the wine1 object
    let wine1_region = wine1.region.copy();
    supported_regions(wine1_region);
    supported_regions(WineRegions::Rioja);
    region_popularity(wine1.region);
    region_popularity(wine2.region);
    region_popularity(wine3.region);
}
