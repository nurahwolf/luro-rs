ALTER TABLE user_warnings
    ADD foreign key (moderator_id) references users(user_id) deferrable initially deferred,
    ADD foreign key (user_id) references users(user_id) deferrable initially deferred;