mod template;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Website {
    home: Home,
    introduction: Introduction,
    images: Images,
    contacts: Contacts,
    hours: Hours,
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
}

#[derive(Deserialize)]
struct Images {
    title: Title,
    note: Note,
    images: Vec<Image>,
}

#[derive(Deserialize)]
struct Contacts {
    title: Title,
    note: Note,
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
    title: Title,
    note: Note,
    monday: Hour,
    tuesday: Hour,
    wednesday: Hour,
    thursday: Hour,
    friday: Hour,
    saturday: Hour,
    sunday: Hour,
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Hour((Option<(u8, u8, u8, u8)>, Option<(u8, u8, u8, u8)>));

#[derive(Deserialize)]
struct Benefits {
    title: Title,
    categories: Vec<Category>,
}

#[derive(Deserialize)]
struct Category {
    title: Title,
    image: Image,
    description: Text,
    benefits: Vec<Benefit>,
}

#[derive(Deserialize)]
struct Benefit {
    title: Title,
    price: Price,
    book: Book,
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
