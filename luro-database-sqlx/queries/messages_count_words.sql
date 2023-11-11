WITH words AS (
    SELECT cast(SUM(unique_words.count) as bigint) as total_words,
        COUNT(DISTINCT(unique_words.word)) as total_unique_words
    FROM unique_words
),
messages AS (
    SELECT COUNT(messages) as total_messages
    FROM messages
)
SELECT messages.total_messages,
    words.total_unique_words,
    words.total_words
FROM messages,
    words