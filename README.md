Create the file `.env` with the following content,

```
DATABASE_URL=sqlite:frases.db
```

To create the database run,

```
sqlx database create
```

and to remove the database run,

```
sqlx database drop
```

In order to create migrations,

```
sqls migrate add <name>
```

And if you want to create a migration with revert,

```
sqls migrate add -r <name>
```

To run a migration,


```
sqlx migrate run
```

To revert a migration

```
sqlx migrate revert
```

To enable guilding in "offline mode" with "query!()",

```
cargo sqlx prepare
```
