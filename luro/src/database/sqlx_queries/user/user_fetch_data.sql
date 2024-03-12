SELECT 
    users.gender "gender: Gender",
    users.sexuality as "sexuality: Sexuality",
    users.user_id,
    users.user_permissions as "user_permissions: UserPermissions"
FROM users
WHERE
    user_id = $1