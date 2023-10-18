WITH arranged AS (
    SELECT message_id,
        UNNEST (
            STRING_TO_ARRAY (
                REGEXP_REPLACE(content, '[^\w\s]', '', 'g'),
                ' '
            )
        ) AS word,
        content
    FROM messages
)
SELECT a.message_id,
    COUNT(a.word) as total_words,
    COUNT(DISTINCT(a.word)) as total_unique_words,
    a.content as message_content
FROM arranged a
GROUP BY a.message_id,
    a.content;