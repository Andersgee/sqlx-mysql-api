# TLDR;

ignore this?
START TRANSACTION works fine (and must end with COMMIT or ROLLBACK)
except for data definition stuff like CREATE and ALTER where ROLLBACK is not possible.

## NOTES:

multiple queries in sequence, with rollback if one fails.
cant really use results of earlier queries in later queries here like in a real transaction

also "data definition" stuff like CREATE and ALTER etc all do implicit
commits and can not be rolled back, see: https://dev.mysql.com/doc/refman/8.0/en/implicit-commit.html
so they are commited even on error / rollback

btw, apparently, this auto commit behaviour also applies to regular queries unless explicitly doing "SET autocommit=0"
tldr from mysql8 reference manual (pseudo quotes):
"autocommit=1 is not recommended for transactions"
"all other databases has autocommit=0 as default"
"mysql has autocommit=1 as default because... no reason"
ok I understand the reason now, it because (atleast with InnoDB), each SQL statement is its own transaction
so the reason for autocommit=1 is to avoid having to COMMIT, even when not inside a real transaction

turns out setting variable does it for entire connection/session, not just this transaction,
docs: https://dev.mysql.com/doc/refman/8.0/en/innodb-autocommit-commit-rollback.html

so testing a bit
doing SET autocommit=1; inside a transaction commits it do db, so no point in transaction
doing SET autocommit=0 behaves as you would expect.
except this session now will NOT DO ANYTHING even after transaction ends.

TLDR; do it like this
START TRANSACTION; SET autocommit=0;
...
...
COMMIT; SET autocommit=1;
or
ROLLBACK; SET autocommit=1;

alternatively one could make sure to set autocommit=1 before every other regular query...

they have a postgres example of transaction here: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/transaction/src/main.rs
but tx.rollback() takes ownership of transaction so cant set autocommit after...
just use the connection instead. it looks like sqlx transaction is just an abstraction on connection that has .commit() etc as methods on it

the above was wrong?

seems like just doing "START TRANSACTION" works fine...
it has implicit autocommit=0... see https://dev.mysql.com/doc/refman/8.0/en/commit.html
quote:
"With START TRANSACTION, autocommit remains disabled until you end the transaction
with COMMIT or ROLLBACK. The autocommit mode then reverts to its previous state."

sidetracked I guess by the fact that "data definition" querys dont allow rollback.
