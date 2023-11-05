SELECT 
    users.user_permissions as "user_permissions: LuroUserPermissions",
    users.gender "gender: Gender",
    users.sexuality as "sexuality: Sexuality"
FROM users
WHERE
    user_id = $1