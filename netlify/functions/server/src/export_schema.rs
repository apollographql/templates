use async_graphql::SDLExportOptions;

use schema::SCHEMA;
mod schema;

fn main() {
    print!("{}", SCHEMA.sdl_with_options(SDLExportOptions::new().federation()));
}