SELECT
    COUNT(*) as total_messages,
    sum(array_length(words, 1)) as total_words,
    count(DISTINCT words) as total_unique_words,
    COUNT(source) filter (where source = 'MESSAGE') as total_message_message,
    COUNT(source) filter (where source = 'MESSAGE_CREATE') as total_message_creates,
    COUNT(source) filter (where source = 'MESSAGE_DELETE') as total_message_deletes,
    COUNT(source) filter (where source = 'MESSAGE_UPDATE') as total_message_updates,
    COUNT(source) filter (where source = 'CACHED_MESSAGE') as total_message_cached,
    COUNT(source) filter (where source = 'CUSTOM') as total_custom_messages
FROM
    message_words
WHERE
    author_id = $1