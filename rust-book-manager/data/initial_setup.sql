INSERT INTO
  roles (name)
VALUES
  ('Admin'),
  ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
  users (name, email, password_hash, role_id)
SELECT
  'Eleazarr Fig',
  'Eleazar.fig@example.com',
  '$2b$12$yMrTYbqZpPpZAHoZhSaQ4uSKThyYJzq7THRf0LhnCVAZoGczWcpT6',
  role_id
FROM
  roles
WHERE
  name LIKE 'Admin';