SELECT category as "category: CharacterFetishCategory",
    character_name,
    character_fetish.fetish_id,
    user_id,
    name,
    description
FROM user_characters_fetishes character_fetish
    JOIN fetishes fetish_details ON character_fetish.fetish_id = fetish_details.fetish_id
WHERE (
        user_id = $1
        and character_name = $2
        and character_fetish.fetish_id = $3
    )
