use async_graphql::{Context, Object, Result};

use crate::db::get_db;

use super::{location::Location, person::Person, plant::Plant};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn plants(&self, ctx: &Context<'_>) -> Result<Vec<Plant>> {
        let plants = sqlx::query_as!(
            Plant,
            r#"
            select
                plants.id as "id!",
                plants.name as "name!",
                (locs.id, locs.name) as "location!: Location",
                birthday as "birthday!",
                image as "image!",
                watering_frequency as "watering_frequency!",
                watering_instructions as "watering_instructions!",
                last_watered_date as "last_watered_date!",
                (ppl.id, ppl.name) as "last_watered_by!: Person"
            from plants
            join locations as locs
                on plants.location = locs.id
            join people as ppl
                on plants.last_watered_by = ppl.id
            "#
        )
        .fetch_all(get_db(ctx)?)
        .await?;
        Ok(plants)
    }

    pub async fn locations(&self, ctx: &Context<'_>) -> Result<Vec<Location>> {
        let locations = sqlx::query_as!(
            Location,
            r#"select id as "id!", name as "name!" from locations"#
        )
        .fetch_all(get_db(ctx)?)
        .await?;
        Ok(locations)
    }
}
