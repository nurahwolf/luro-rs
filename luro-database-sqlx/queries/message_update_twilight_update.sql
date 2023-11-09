UPDATE messages
SET message_updates = message_updates || $2,
    source = $3
WHERE message_id = $1