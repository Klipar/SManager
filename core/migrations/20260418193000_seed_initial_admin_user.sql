-- Seed a bootstrap admin user for first login.
-- Default credentials:
--   email: admin@localhost
--   password: admin123
-- IMPORTANT: change password immediately after first login.

INSERT INTO users (name, email, password, is_admin)
VALUES (
    'admin',
    'admin@localhost',
    'jGl25bVBBBW96Qi9Te4V37Fnqchz/Eu4qB9vKrRIqRg=',
    TRUE
)
ON CONFLICT (email) DO NOTHING;
