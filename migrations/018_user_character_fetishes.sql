DO $$ BEGIN
    CREATE TYPE user_characters_fetishes_category AS ENUM ('FAV', 'LOVE', 'LIKE', 'NEUTRAL', 'DISLIKE', 'HATE', 'LIMIT');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- DROP TABLE user_characters_fetishes;
CREATE TABLE IF NOT EXISTS user_characters_fetishes (
    character_name text    not null,
    fetish_id      integer not null
        constraint user_characters_fetishes_fetishes_fetish_id_fk
            references fetishes,
    user_id        integer not null
        constraint user_characters_fetishes_users_user_id_fk
            references users,
    category       user_characters_fetishes_category default 'NEUTRAL',
    constraint user_characters_fetishes_pk
        primary key (user_id, character_name, fetish_id),
    constraint user_characters_fetishes_user_id_character_name_fk
        foreign key (user_id, character_name) references user_characters
);