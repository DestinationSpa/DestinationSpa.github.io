mod template;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Website {
    home: Home,
    introduction: Introduction,
    infos: Infos,
    benefits: Benefits,
}

#[derive(Deserialize)]
struct Home {
    title: Title,
    subtitle: Title,
    note: Note,
}

#[derive(Deserialize)]
struct Introduction {
    title: Title,
    text: Text,
    images: Vec<Image>,
}

#[derive(Deserialize)]
struct Infos {
    title: Title,
    contacts: Contacts,
    hours: Hours,
}

#[derive(Deserialize)]
struct Contacts {
    phone: Phone,
    email: Email,
    location: Location,
    socials: Socials,
}

#[derive(Deserialize)]
struct Location {
    address: String,
    postcode: String,
    city: String,
    latitude: String,
    longitude: String,
    query: Vec<String>,
}

#[derive(Deserialize)]
struct Socials {
    facebook: Social,
    instagram: Social,
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Social(Option<String>);

#[derive(Deserialize)]
struct Hours {
    monday: Day,
    tuesday: Day,
    wednesday: Day,
    thursday: Day,
    friday: Day,
    saturday: Day,
    sunday: Day,
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Day(Option<(u8, u8, u8, u8)>);

#[derive(Deserialize)]
struct Benefits {
    title: Title,
    categories: Vec<Category>,
}

#[derive(Deserialize)]
struct Category {
    title: Title,
    image: Option<Image>,
    description: Text,
    benefits: Vec<Benefit>,
}

#[derive(Deserialize)]
struct Benefit {
    title: Title,
    image: Option<Image>,
    price: Price,
    book: Book,
    more: bool,
    description: Text,
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Book(String);

#[derive(Deserialize)]
#[serde(transparent)]
struct Email(String);

#[derive(Deserialize)]
#[serde(transparent)]
struct Image((String, String));

#[derive(Deserialize)]
#[serde(transparent)]
struct Inline(String);

#[derive(Deserialize)]
#[serde(transparent)]
struct Note(Option<Inline>);

#[derive(Deserialize)]
#[serde(transparent)]
struct Phone(String);

#[derive(Deserialize)]
#[serde(transparent)]
struct Price((u32, u32));

#[derive(Deserialize)]
#[serde(transparent)]
struct Text(String);

#[derive(Deserialize)]
#[serde(transparent)]
struct Title(Inline);
