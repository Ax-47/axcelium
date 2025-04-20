CREATE TABLE users(
    userId int PRIMARY KEY not null AUTO_INCREMENT,
    userName varchar(50),
    password varchar(50),
    isValid boolean
)