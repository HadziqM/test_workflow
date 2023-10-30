drop table if exists users cascade;
drop table if exists sessions cascade;
drop table if exists ships cascade;
drop table if exists user_ship cascade;
drop table if exists sensors cascade;
drop table if exists flow_sens cascade;
drop table if exists sonic_sens cascade;
drop table if exists vibration_sens cascade;
drop table if exists gps_sens cascade;
drop table if exists temp_sens cascade;


create table if not exists users(
  id serial,
  username text not null unique,
  password text not null,
  status int not null default(1),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id)
);

create table if not exists sessions(
  id serial,
  key text not null unique,
  user_id int,
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
    CONSTRAINT fk_users
      FOREIGN KEY(user_id) 
	      REFERENCES users(id)
);
create table if not exists ships(
  id serial,
  name text not null,
  machine_type int not null default(1),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id)
);
create table if not exists user_ship(
  id serial,
  ship_id int,
  user_id int,
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_users
      FOREIGN KEY(user_id) 
	      REFERENCES users(id),
  CONSTRAINT fk_ships
    FOREIGN KEY(ship_id) 
      REFERENCES ships(id)
);
create table if not exists sensors(
  id serial,
  ship_id int,
  status boolean default(false),
  name text not null,
  identifier int not null ,
  table_name text not null,
  factor real not null default(1),
  zero_value real not null default(0),
  min_value real not null default(0),
  max_value real not null default(0),
  warning_value real not null,
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_ships
    FOREIGN KEY(ship_id) 
      REFERENCES ships(id)
);
create table if not exists flow_sens(
  id serial,
  sens_id int,
  data real not null default(0),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_sensors
    FOREIGN KEY(sens_id) 
      REFERENCES sensors(id)

);
create table if not exists sonic_sens(
  id serial,
  sens_id int,
  data real not null default(0),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_sensors
    FOREIGN KEY(sens_id) 
      REFERENCES sensors(id)
);

create table if not exists vibration_sens(
  id serial,
  sens_id int,
  data real not null default(0),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_sensors
    FOREIGN KEY(sens_id) 
      REFERENCES sensors(id)
);

create table if not exists temp_sens(
  id serial,
  sens_id int,
  data real not null default(0),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_sensors
    FOREIGN KEY(sens_id) 
      REFERENCES sensors(id)
);

create table if not exists gps_sens(
  id serial,
  sens_id int,
  latitude real not null default(0),
  longitude real not null default(0),
  speed real not null default(0),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_sensors
    FOREIGN KEY(sens_id) 
      REFERENCES sensors(id)
);

create table if not exists ship_log(
  id serial,
  ship_id int,
  message text not null,
  severity int not null default(0)
  global boolean default(false),
  created_at timestamp without time zone default now(),
  PRIMARY KEY(id),
  CONSTRAINT fk_ships
    FOREIGN KEY(ship_id) 
      REFERENCES ships(id)
);
