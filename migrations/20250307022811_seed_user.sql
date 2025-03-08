-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '04bd12e6-03f4-4ae1-8304-a0ad5ed7f1ed',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$dMa2qb5SB4IjzsZ5no0jxA$hirQ3CPiq2s08wtyv7aBybidgp25YSiQT4Ri3raEGLc'
)