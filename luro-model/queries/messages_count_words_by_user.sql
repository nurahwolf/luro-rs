WITH words AS (
    SELECT messages.author_id,
        messages.content,
        UNNEST (
            STRING_TO_ARRAY (
                REGEXP_REPLACE(content, '[^\w\s]', '', 'g'),
                ' '
            )
        ) AS word
    FROM messages
    WHERE author_id = $1
)
SELECT words.author_id,
    COUNT(DISTINCT (words.content)) as total_messages,
    COUNT(words.word) as total_words,
    COUNT(DISTINCT(words.word)) as total_unique_words
FROM words
GROUP BY words.author_id
ORDER BY total_unique_words DESC