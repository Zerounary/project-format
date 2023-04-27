
SELECT p.name p_name, a.name AS s_name
FROM `test_a` p
LEFT JOIN `test_a` a ON p.id = a.p_id
where 1 = 1
\{{#if id }}
and p.id = @ id @
\{{/if}}
\{{#if name }}
and p.name like @ name @ || '%'
\{{/if}}