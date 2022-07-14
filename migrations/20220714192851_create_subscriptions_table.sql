CREATE TABLE subscriptions
(
    id            BIGSERIAL PRIMARY KEY,
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at TIMESTAMPTZ NOT NULL
);