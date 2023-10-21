create table user_character_images
(
    user_id        bigint not null
        constraint user_character_images_users_user_id_fk
            references users,
    character_name text   not null,
    favourite      boolean,
    img_id         bigint not null,
    constraint user_character_images_pk
        primary key (user_id, character_name, img_id),
    constraint user_character_images_user_characters_user_id_character_name_fk
        foreign key (user_id, character_name) references user_characters
);