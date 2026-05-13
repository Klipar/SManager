-- Seed a bootstrap admin user for first login.
-- Default credentials:
--   email: admin@localhost
--   password: admin123
-- IMPORTANT: change password immediately after first login.

INSERT INTO users (name, email, password, is_admin)
VALUES (
    'admin',
    'admin@localhost',
    'pib9szStyiWl7ib38y/09Z/gW1oMokLG7diRpjMThqc=',
    TRUE
)
ON CONFLICT (email) DO NOTHING;
