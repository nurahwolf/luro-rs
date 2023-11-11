WITH words AS (
    SELECT message_id,
        messages.content,
        UNNEST (
            STRING_TO_ARRAY (
                REGEXP_REPLACE(content, '[^\w\s]', '', 'g'),
                ' '
            )
        ) AS word
    FROM messages
)
SELECT COUNT(DISTINCT(words.content)) as total_messages,
    COUNT(words.word) as total_words,
    COUNT(DISTINCT(words.word)) as total_unique_words
FROM words