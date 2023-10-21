create table images (
    img_id   bigint  not null
        constraint images_pk
            primary key,
    url      text    not null,
    owner_id bigint  not null
        constraint images_users_user_id_fk
            references users,
    nsfw     boolean not null,
    source   text
);