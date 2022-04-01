CREATE TABLE timesheets (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  time_done INTERVAL NOT NULL,
  time_target INTERVAL NOT NULL,
  start_date TIMESTAMP,
  end_date TIMESTAMP,
  grade REAL,
  ects REAL
)