CREATE TYPE gender AS ENUM ('MALE', 'FEMALE', 'TRANS_FEMALE', 'TRANS_MALE', 'ITS_COMPLICATED');
CREATE TYPE sexuality AS ENUM ('STRAIGHT', 'BISEXUAL', 'PANSEXUAL', 'LESBIAN', 'GAY');


-- if EXISTS (SELECT FROM user_marriages WHERE proposer_id = 392992034304294924) then
--    statements;
-- end if;

-- UPDATE users
-- SET user_permissions = 'USER'
-- WHERE user_permissions IS NULL;

    -- user_permissions user_permissions NOT NULL default 'USER',


-- ALTER TABLE users
-- ALTER COLUMN user_permissions SET DEFAULT 'USER';

-- DO $$
--     BEGIN
--     IF EXISTS (SELECT FROM user_marriages WHERE proposer_id = 392992034304294924) THEN
--         insert into user_marriages (divorced, proposer_id, proposee_id, rejected, reason)
--         values
--             (true, 373524896187416576, 392992034304294924, true, 'blah')
--         ON CONFLICT
--             (proposer_id, proposee_id)
--         DO UPDATE SET divorced = true, reason = 'hi', rejected = true
--         RETURNING *;
--     ELSE
--         insert into user_marriages (divorced, proposer_id, proposee_id, rejected, reason)
--         values
--             (true, 392992034304294924, 373524896187416576, true, 'blah')
--         ON CONFLICT
--             (proposer_id, proposee_id)
--         DO UPDATE SET divorced = true, reason = 'hi', rejected = true;
--         -- RETURNING divorced, proposee_id, proposer_id, reason, rejected;
--     END IF;
-- END $$;

-- insert into user_marriages (divorced, proposer_id, proposee_id, rejected, reason)
-- values
--     (true, 373524896187416576, 392992034304294924, true, 'blah')
-- select
--     proposer_id = 373524896187416576, proposee_id = 392992034304294924
-- where not exists (
--     select 1 from user_marriages (proposer_id, proposee_id) where proposee_id = 373524896187416576 and proposer_id = 392992034304294924
-- )
-- ON CONFLICT (proposer_id, proposee_id)
-- DO UPDATE SET divorced = $1, reason = $4, rejected = $5
-- RETURNING *;

-- INSERT INTO user_marriages( divorced, proposee_id, proposer_id, reason, rejected ) from user_marriages
-- WHERE (proposer_id, proposee_id) IN (select proposer_id, proposee_id from user_marriages where
--     (proposer_id = 373524896187416576 AND proposee_id = 392992034304294924)
--         or
--     (proposer_id = 392992034304294924 AND proposee_id = 373524896187416576)
-- )

-- DELETE FROM user_marriages
--             WHERE (proposer_id, proposee_id) IN (select proposer_id, proposee_id from user_marriages where
--                 (proposer_id = $1 AND proposee_id = $2)
--                     or
--                 (proposer_id = $2 AND proposee_id = $1)
--             )
--             RETURNING *


-- INSERT INTO user_marriages( divorced, proposee_id, proposer_id, reason, rejected )
--     select divorced, proposee_id, proposer_id, reason, rejected
--     FROM user_marriages
--     WHERE proposer_id = 373524896187416576 AND proposee_id = 468251651044671488
-- union
--     select divorced, proposee_id as proposer_id, proposer_id as proposee_id, reason, rejected
--     FROM user_marriages
--     WHERE proposer_id = 373524896187416576 AND proposee_id = 468251651044671488
-- ON CONFLICT (proposer_id, proposee_id)
-- DO UPDATE SET divorced = true, reason = 'Yeet', rejected = true
-- RETURNING divorced, proposee_id, proposer_id, reason, rejected

-- update table set colunm3 = 'value'
-- where exists (
--     select pkey, column1, column2 from table
--     where (column1 = @refa and column2 = @refb)
--         or
--     (column1 = @refb and column2 = @refa)
-- )

-- DELETE FROM user_marriages
-- WHERE EXISTS (select proposer_id, proposee_id from user_marriages where
--     (proposer_id = 392992034304294924 AND proposee_id = 373524896187416576)
--         or
--     (proposer_id = 373524896187416576 AND proposee_id = 392992034304294924)
-- );

-- select proposer_id, proposee_id, divorced, rejected, reason from user_marriages WHERE proposer_id = 373524896187416576 AND proposee_id = 392992034304294924
-- union
-- select proposee_id as proposer_id, proposer_id as proposee_id, divorced, rejected, reason from user_marriages WHERE proposer_id = 373524896187416576 AND proposee_id = 392992034304294924;


-- DELETE FROM user_marriages WHERE (proposee_id, proposer_id) IN (
-- select proposee_id, proposer_id
--     FROM user_marriages
--     WHERE proposer_id = 392992034304294924 AND proposee_id = 373524896187416576
-- union
-- select proposee_id as proposer_id, proposer_id as proposee_id
--     FROM user_marriages
--     WHERE proposer_id = 392992034304294924 AND proposee_id = 373524896187416576
-- )


-- INSERT INTO user_marriages (
--                 divorced,
--                 proposee_id,
--                 proposer_id,
--                 reason,
--                 rejected
--             ) VALUES
--                 ($1, $2, $3, $4, $5)
--             ON CONFLICT
--                 (proposer_id, proposee_id)
--             DO UPDATE SET
--                 divorced = $1,
--                 reason = $4,
--                 rejected = $5
            -- RETURNING
            --     divorced,
            --     proposee_id,
            --     proposer_id,
            --     reason,
            --     rejected


-- SELECT COUNT(*) FROM user_marriages WHERE proposer_id = 373524896187416576 AND proposee_id = 468251651044671488;
-- UPDATE user_marriages SET divorced = true WHERE proposer_id = 468251651044671488 AND proposee_id = 373524896187416576;

-- select proposer_id, proposee_id, divorced, rejected, reason from user_marriages
-- union
-- select proposee_id as proposer_id, proposer_id as proposee_id, divorced, rejected, reason from user_marriages

-- INSERT INTO user_marriages (divorced, proposee_id, proposer_id, reason)
-- VALUES (true, 373524896187416576, 468251651044671488, 'Blah')
-- ON CONFLICT (proposer_id, proposee_id)
-- DO UPDATE SET divorced = true
-- RETURNING divorced, proposee_id, proposer_id, reason, rejected;

-- SELECT divorced, proposee_id, proposer_id, reason, rejected
-- FROM user_marriages
-- WHERE NOT EXISTS (
--     SELECT *
--     FROM user_marriages
    -- WHERE (proposer_id = 373524896187416576 AND proposee_id = 468251651044671488)
    --    OR (proposer_id = 468251651044671488 AND proposee_id = 373524896187416576)
-- )
-- ON CONFLICT (proposer_id, proposee_id)
-- DO UPDATE SET divorced = true
-- RETURNING divorced, proposee_id, proposer_id, reason, rejected;

-- {
--   "proposer_id": "373524896187416576",
--   "proposee_id": "468251651044671488",
--   "divorced": true,
--   "rejected": false,
--   "reason": "In a daring move, <@373524896187416576> slides towards <@468251651044671488> with roller skates, but ends up crashing into a pie. They mumble, 'Pie you marry me?'"
-- }

-- WITH arranged AS (
--     SELECT message_id,
--         UNNEST (
--             STRING_TO_ARRAY (
--                 REGEXP_REPLACE(content, '[^\w\s]', '', 'g'),
--                 ' '
--             )
--         ) AS word,
--         content
--     FROM messages
-- )
-- SELECT a.message_id,
--     COUNT(a.word) as total_words,
--     COUNT(DISTINCT(a.word)) as total_unique_words,
--     a.content as message_content
-- FROM arranged a
-- GROUP BY a.message_id,
--     a.content;

-- SELECT * FROM user_marriages WHERE proposer_id = 373524896187416576 or proposee_id = 373524896187416576
-- ALTER TABLE user_marriages RENAME COLUMN active TO divorced;

-- SELECT
--     COUNT(approve) filter (where approve) as approvers,
--     COUNT(disapprove) filter (where disapprove) as disapprovers
-- FROM 
--     user_marriage_approvals
-- WHERE
--     proposer_id = 373524896187416576 and proposee_id = 468251651044671488


-- SELECT divorced, proposee_id, proposer_id, reason, rejected
-- FROM input
-- WHERE NOT EXISTS (
--     SELECT 1
--     FROM user_marriages
--     WHERE (proposer_id = input.proposer_id AND proposee_id = input.proposee_id)
--        OR (proposer_id = input.proposee_id AND proposee_id = input.proposer_id)
-- )
-- ON CONFLICT (proposer_id, proposee_id)
-- DO UPDATE SET
--     divorced = excluded.divorced,
--     reason = excluded.reason,
--     rejected = excluded.rejected
-- RETURNING divorced, proposee_id, proposer_id, reason, rejected;