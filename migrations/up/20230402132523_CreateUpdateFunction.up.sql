create or replace function updateRecord()

returns trigger as
$$
  begin
    new.updated_at = (current_timestamp at time zone 'utc');
    return new;
  end;
$$

language plpgsql;
