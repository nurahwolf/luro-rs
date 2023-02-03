use luro_core::Data;
use poise::{serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton, CreateComponents, ReactionType, CreateSelectMenu, CreateSelectMenuOption, CreateEmbed}, CreateReply};

/// Create a button that responds to "show_menu", used for opening further menus
pub fn open_menu_button() -> CreateButton {
    CreateButton::default()
        .emoji(ReactionType::Unicode("📝".to_string()))
        .custom_id("show_menu")
        .to_owned()
}

/// Create a button that responds to "close_menu", used for closing open menus
pub fn close_menu_button() -> CreateButton {
    CreateButton::default()
        .emoji(ReactionType::Unicode("❎".to_string()))
        .custom_id("close_menu")
        .to_owned()
}

/// Delete the sent Discord message
pub fn remove_favourite_button() -> CreateButton {
    CreateButton::default()
        .custom_id("remove")
        .label("Remove this message")
        .style(ButtonStyle::Primary)
        .to_owned()
}

// Delete a favourite from the database
pub fn delete_favourite_button() -> CreateButton {
    CreateButton::default()
        .custom_id("delete")
        .label("Delete Favourite")
        .style(ButtonStyle::Danger)
        .to_owned()
}

/// Create message components used for showing a more advanced menu. This contains [open_menu_button]
pub fn initial_menu_row() -> CreateComponents {
    CreateComponents::default()
        .create_action_row(|row| row.add_button(open_menu_button()))
        .to_owned()
}

pub async fn create_selection_menu(data: &Data, author_id: &String) -> Result<CreateSelectMenu, String> {
    let favourites = &data.user_favourites.read().await.favs;

    // Get favorites from author
    let user_favourites = match favourites.get(author_id) {
        Some(ok) => ok,
        None => {
            return Err("Looks like you don't have any favorites saved yet!".into());
        }
    };

    let mut menu = CreateSelectMenu::default();
    menu.custom_id("menu");
    menu.placeholder("Move to a new category");
    menu.options(|options| {
        for fav in user_favourites {
            let mut option = CreateSelectMenuOption::default();
            option.label(fav.0);
            option.value(fav.0);
            options.add_option(option);
        }
        options
    });
    Ok(menu)
}

pub async fn favourite_categories_row(data: &Data, author_id: &String) -> Result<CreateActionRow, String> {
    Ok(CreateActionRow::default().add_select_menu(create_selection_menu(data, author_id).await?).to_owned())
}

pub fn reply_builder(embed: CreateEmbed) -> CreateReply<'static> {
    let mut reply_builder = CreateReply::default();
    reply_builder.embed(|e| {
        *e = embed;
        e
    }).components(|c|{
        *c = initial_menu_row();
        c
    });
    reply_builder
}

pub fn favorite_manipulation_row() -> CreateActionRow {
    CreateActionRow::default()
        .add_button(close_menu_button())
        .add_button(remove_favourite_button())
        .add_button(delete_favourite_button())
        .to_owned()
}
