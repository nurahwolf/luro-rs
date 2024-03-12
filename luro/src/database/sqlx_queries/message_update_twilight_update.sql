UPDATE messages
SET 
    message_updates = COALESCE(message_updates || $2, $2),
    source = $3
WHERE message_id = $1