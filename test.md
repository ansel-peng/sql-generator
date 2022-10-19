## Template

### user

| field |    type     | comment | primary | auto_increment | not_null | default |
|:-----:|:-----------:|:-------:|:-------:|:--------------:|:--------:|:-------:|
|  id   |   int(11)   |   主键    |  true   |      true      |   true   |         |
| name  | varchar(50) |   名称    |         |                |   true   |  true   |  
|  age  |   int(11)   |   年龄    |         |                |   true   |         |  

|  type  |   columns   |
|:------:|:-----------:|
| unique |  name,age   |