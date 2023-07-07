create table if not exists accounts (
    id serial primary key,
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc'),
    updated_at timestamp with time zone not null default (current_timestamp at time zone 'utc'),
    email varchar(50),
    password varchar(100)
);

create trigger update_accounts_updated_at 
  before update on accounts 
  for each row 
  execute procedure updateRecord();
