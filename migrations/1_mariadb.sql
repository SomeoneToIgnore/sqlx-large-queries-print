create table if not exists test_table
(
    id serial primary key,
    a int            not null,
    b int            not null,
    c int            not null,
    d int            not null,
    e date           not null,
    f date           not null,
    g time           not null,
    h time           not null,
    i tinytext       not null,
    j decimal(10, 2) not null
);
