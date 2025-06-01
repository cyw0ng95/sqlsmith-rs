-- Enable foreign key constraints for the current connection.
-- This MUST be executed for every new database connection.
PRAGMA foreign_keys = ON;

-- 1. WAREHOUSE Table
CREATE TABLE IF NOT EXISTS warehouse (
    w_id        INTEGER PRIMARY KEY,
    w_name      TEXT    NOT NULL,
    w_street_1  TEXT    NOT NULL,
    w_street_2  TEXT    NOT NULL,
    w_city      TEXT    NOT NULL,
    w_state     TEXT    NOT NULL,
    w_zip       TEXT    NOT NULL,
    w_tax       REAL    NOT NULL,
    w_ytd       REAL    NOT NULL
);

-- 2. DISTRICT Table
CREATE TABLE IF NOT EXISTS district (
    d_id            INTEGER NOT NULL,
    d_w_id          INTEGER NOT NULL,
    d_name          TEXT    NOT NULL,
    d_street_1      TEXT    NOT NULL,
    d_street_2      TEXT    NOT NULL,
    d_city          TEXT    NOT NULL,
    d_state         TEXT    NOT NULL,
    d_zip           TEXT    NOT NULL,
    d_tax           REAL    NOT NULL,
    d_ytd           REAL    NOT NULL,
    d_next_o_id     INTEGER NOT NULL,
    PRIMARY KEY (d_id, d_w_id),
    FOREIGN KEY (d_w_id) REFERENCES warehouse(w_id)
);

-- 3. CUSTOMER Table
CREATE TABLE IF NOT EXISTS customer (
    c_id            INTEGER NOT NULL,
    c_d_id          INTEGER NOT NULL,
    c_w_id          INTEGER NOT NULL,
    c_first         TEXT    NOT NULL,
    c_middle        TEXT    NOT NULL,
    c_last          TEXT    NOT NULL,
    c_street_1      TEXT    NOT NULL,
    c_street_2      TEXT    NOT NULL,
    c_city          TEXT    NOT NULL,
    c_state         TEXT    NOT NULL,
    c_zip           TEXT    NOT NULL,
    c_phone         TEXT    NOT NULL,
    c_since         TEXT    NOT NULL, -- Stored as ISO8601 string (YYYY-MM-DD HH:MM:SS.SSS)
    c_credit        TEXT    NOT NULL, -- 'GC' or 'BC'
    c_credit_lim    REAL    NOT NULL,
    c_discount      REAL    NOT NULL,
    c_balance       REAL    NOT NULL,
    c_ytd_payment   REAL    NOT NULL,
    c_payment_cnt   INTEGER NOT NULL,
    c_delivery_cnt  INTEGER NOT NULL,
    c_data          TEXT    NOT NULL,
    PRIMARY KEY (c_id, c_d_id, c_w_id),
    FOREIGN KEY (c_d_id, c_w_id) REFERENCES district(d_id, d_w_id)
);

-- 4. HISTORY Table
CREATE TABLE IF NOT EXISTS history (
    h_c_id      INTEGER NOT NULL,
    h_c_d_id    INTEGER NOT NULL,
    h_c_w_id    INTEGER NOT NULL,
    h_d_id      INTEGER NOT NULL,
    h_w_id      INTEGER NOT NULL,
    h_date      TEXT    NOT NULL, -- Stored as ISO8601 string (YYYY-MM-DD HH:MM:SS.SSS)
    h_amount    REAL    NOT NULL,
    h_data      TEXT    NOT NULL,
    FOREIGN KEY (h_c_id, h_c_d_id, h_c_w_id) REFERENCES customer(c_id, c_d_id, c_w_id),
    FOREIGN KEY (h_d_id, h_w_id) REFERENCES district(d_id, d_w_id)
);

-- 5. ITEM Table
CREATE TABLE IF NOT EXISTS item (
    i_id        INTEGER PRIMARY KEY,
    i_im_id     INTEGER NOT NULL,
    i_name      TEXT    NOT NULL,
    i_price     REAL    NOT NULL,
    i_data      TEXT    NOT NULL
);

-- 6. STOCK Table
CREATE TABLE IF NOT EXISTS stock (
    s_i_id          INTEGER NOT NULL,
    s_w_id          INTEGER NOT NULL,
    s_quantity      INTEGER NOT NULL,
    s_dist_01       TEXT    NOT NULL,
    s_dist_02       TEXT    NOT NULL,
    s_dist_03       TEXT    NOT NULL,
    s_dist_04       TEXT    NOT NULL,
    s_dist_05       TEXT    NOT NULL,
    s_dist_06       TEXT    NOT NULL,
    s_dist_07       TEXT    NOT NULL,
    s_dist_08       TEXT    NOT NULL,
    s_dist_09       TEXT    NOT NULL,
    s_dist_10       TEXT    NOT NULL,
    s_ytd           INTEGER NOT NULL,
    s_order_cnt     INTEGER NOT NULL,
    s_remote_cnt    INTEGER NOT NULL,
    s_data          TEXT    NOT NULL,
    PRIMARY KEY (s_i_id, s_w_id),
    FOREIGN KEY (s_i_id) REFERENCES item(i_id),
    FOREIGN KEY (s_w_id) REFERENCES warehouse(w_id)
);

-- 7. ORDER Table
CREATE TABLE IF NOT EXISTS "order" ( -- "order" is a keyword, so quote it
    o_id            INTEGER NOT NULL,
    o_d_id          INTEGER NOT NULL,
    o_w_id          INTEGER NOT NULL,
    o_c_id          INTEGER NOT NULL,
    o_entry_d       TEXT    NOT NULL, -- Stored as ISO8601 string (YYYY-MM-DD HH:MM:SS.SSS)
    o_carrier_id    INTEGER,          -- Can be NULL
    o_ol_cnt        INTEGER NOT NULL,
    o_all_local     INTEGER NOT NULL, -- 0 or 1 for boolean
    PRIMARY KEY (o_id, o_d_id, o_w_id),
    FOREIGN KEY (o_c_id, o_d_id, o_w_id) REFERENCES customer(c_id, c_d_id, c_w_id)
);

-- 8. NEW_ORDER Table
CREATE TABLE IF NOT EXISTS new_order (
    no_o_id     INTEGER NOT NULL,
    no_d_id     INTEGER NOT NULL,
    no_w_id     INTEGER NOT NULL,
    PRIMARY KEY (no_o_id, no_d_id, no_w_id),
    FOREIGN KEY (no_o_id, no_d_id, no_w_id) REFERENCES "order"(o_id, o_d_id, o_w_id)
);

-- 9. ORDER_LINE Table
CREATE TABLE IF NOT EXISTS order_line (
    ol_o_id         INTEGER NOT NULL,
    ol_d_id         INTEGER NOT NULL,
    ol_w_id         INTEGER NOT NULL,
    ol_number       INTEGER NOT NULL,
    ol_i_id         INTEGER NOT NULL,
    ol_supply_w_id  INTEGER NOT NULL,
    ol_delivery_d   TEXT,             -- Can be NULL
    ol_quantity     INTEGER NOT NULL DEFAULT 0,
    ol_amount       REAL    NOT NULL,
    ol_dist_info    TEXT    NOT NULL,
    PRIMARY KEY (ol_o_id, ol_d_id, ol_w_id, ol_number),
    FOREIGN KEY (ol_o_id, ol_d_id, ol_w_id) REFERENCES "order"(o_id, o_d_id, o_w_id),
    FOREIGN KEY (ol_i_id) REFERENCES item(i_id),
    FOREIGN KEY (ol_supply_w_id, ol_i_id) REFERENCES stock(s_w_id, s_i_id)
);