-- CREATE TYPE Status AS ENUM ('Processing', 'Approved', 'Declined', 'Failed');

CREATE TABLE users
(
    id    uuid PRIMARY KEY,
    alias character varying(255)

--     amount      integer                        NOT NULL,
--     card_number character varying(255)         NOT NULL,
--     status      Status                         NOT NULL,
--     hold_ref    uuid,
--     inserted_at timestamp(0) without time zone NOT NULL,
--     updated_at  timestamp(0) without time zone NOT NULL
);
CREATE UNIQUE INDEX users_id_index ON users (id uuid_ops);
CREATE UNIQUE INDEX users_alias_index ON users (alias text_ops);
-- CREATE INDEX payments_approved_status_ix ON payments (status) WHERE status = 'Approved';

-- CREATE TABLE refunds
-- (
--     id          uuid PRIMARY KEY,
--     payment_id  uuid REFERENCES payments (id)  NOT NULL,
--     amount      integer                        NOT NULL,
--     inserted_at timestamp(0) without time zone NOT NULL,
--     updated_at  timestamp(0) without time zone NOT NULL
-- );
--
-- -- CREATE UNIQUE INDEX refunds_pkey ON refunds(id uuid_ops);
-- CREATE UNIQUE INDEX refunds_id_index ON refunds (id uuid_ops);
-- CREATE INDEX refunds_payment_id_index ON refunds (payment_id uuid_ops);
