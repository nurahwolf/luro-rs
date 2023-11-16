CREATE FUNCTION public.update_unique_words() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    INSERT INTO unique_words (word, count)
    SELECT word, COUNT(*) AS count
    FROM (
        SELECT
            UNNEST(
                STRING_TO_ARRAY(
                    REGEXP_REPLACE(NEW.content, '[^\w\s]', '', 'g'),
                    ' '
                )
            ) AS word
        ) AS words
    GROUP BY word
    ON CONFLICT (word) DO UPDATE
    SET count = unique_words.count + 1;
    RETURN NEW;
END;
$$;