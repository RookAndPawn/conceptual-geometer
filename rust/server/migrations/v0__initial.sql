

create table plugin (
    file text primary key not null,
    name text not null,
    plugin_version text not null,
    plugin_rustc_version text not null,
    plugin_cg_version text not null
);
