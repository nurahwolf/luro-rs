SELECT
    word,
    COUNT(*) AS word_count
FROM (
    SELECT
        UNNEST(
            STRING_TO_ARRAY(
                REGEXP_REPLACE(content, '[^\w\s]', '', 'g'),
                ' '
            )
        ) AS word
    FROM messages
) AS words
GROUP BY word
ORDER BY word_count DESC