# sql-generator

This is a tool to convert markdown to sql.

## Usage

### Source code

```console
cargo run test.md
```

### Binary

```console
./sql-generator test.md
```

## Markdown format

- The third level title represents the table name

```markdown
### user

| field |    type     | comment | primary | auto_increment | not_null | default |
|:-----:|:-----------:|:-------:|:-------:|:--------------:|:--------:|:-------:|
|  id   |   int(11)   |   主键    |  true   |      true      |   true   |         |
| name  | varchar(50) |   名称    |         |                |   true   |  true   |  
|  age  |   int(11)   |   年龄    |         |                |   true   |         |  

|  type  |   columns   |
|:------:|:-----------:|
| unique |  name,age   |
```

