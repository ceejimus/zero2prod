-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
  '58adc858-88bd-4ec6-aecc-800367858fe9',
  'admin',
  '$argon2id$v=19$m=15000,t=2,p=1$WQFskHQ3dhvvIYpB5KVVlw$HBDCcv999PzA9Q3UJfn+SsGJWwFWR5IJkVZVwwg1nA8'
)

