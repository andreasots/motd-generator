use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::pg::{Pg, PgConnection};
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use rand;

table! {
    quotes {
        id -> Integer,
        quote -> Text,
        attrib_name -> Nullable<Text>,
        attrib_date -> Nullable<Date>,
        deleted -> Bool,
        context -> Nullable<Text>,
        game_id -> Nullable<Integer>,
        show_id -> Nullable<Integer>,
    }
}

#[derive(Debug)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub attrib_name: Option<String>,
    pub attrib_date: Option<NaiveDate>,
    pub deleted: bool,
    pub context: Option<String>,
    pub game_id: Option<i32>,
    pub show_id: Option<i32>,
}

impl Queryable<quotes::SqlType, Pg> for Quote {
    type Row = (i32,
     String,
     Option<String>,
     Option<NaiveDate>,
     bool,
     Option<String>,
     Option<i32>,
     Option<i32>);

    fn build((id, quote, attrib_name, attrib_date, deleted, context, game_id, show_id): Self::Row)
             -> Quote {
        Quote {
            id: id,
            quote: quote,
            attrib_name: attrib_name,
            attrib_date: attrib_date,
            deleted: deleted,
            context: context,
            game_id: game_id,
            show_id: show_id,
        }
    }
}

impl Display for Quote {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        try!(write!(formatter, "Quote #{}: \"{}\"", self.id, self.quote));
        if let Some(ref name) = self.attrib_name {
            try!(write!(formatter, " —{}", name));
        }
        if let Some(ref date) = self.attrib_date {
            try!(write!(formatter, " [{}]", date));
        }
        Ok(())
    }
}

pub fn get_quote() -> Result<String, Box<Error>> {
    use diesel_full_text_search::{plainto_tsquery, to_tsvector, TsVectorExtensions};
    use self::quotes::dsl::*;

    let conn = try!(PgConnection::establish("postgres:///lrrbot"));
    let res = try!(quotes.filter(deleted.eq(false).and(to_tsvector(quote)
                                                           .matches(plainto_tsquery("butts"))))
                         .load::<Quote>(&conn));

    match rand::sample(&mut rand::thread_rng(), res, 1).get(0) {
        Some(q) => Ok(format!("{}", q)),
        None => Err(String::from("No matching quotes.").into()),
    }
}
