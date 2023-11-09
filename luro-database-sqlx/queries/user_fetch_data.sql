SELECT 
    users.gender "gender: DbGender",
    users.sexuality as "sexuality: DbSexuality",
    users.user_id,
    users.user_permissions as "user_permissions: DbUserPermissions"
FROM users
WHERE
    user_id = $1