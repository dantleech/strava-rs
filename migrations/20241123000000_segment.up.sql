CREATE TABLE segment (
    id BIGINT PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    activity_type VARCHAR(255) NOT NULL,
    distance FLOAT NOT NULL,
    average_grade FLOAT,
    maximum_grade FLOAT,
    elevation_high FLOAT,
    elevation_low FLOAT,
    start_lat FLOAT,
    start_long FLOAT,
    end_lat FLOAT,
    end_long FLOAT,
    climb_category INT,
    city VARCHAR(255),
    state VARCHAR(255),
    country VARCHAR(255),
    hazardous BOOLEAN
);
CREATE TABLE segment_effort (
    id BIGINT NOT NULL,
    activity_id BIGINT NOT_NULL
    segment_id BIGINT NOT_NULL
    elapsed_time INT NOT NULL,
    moving_time INT NOT NULL,
    start_date DATETIME NOT NULL,
    average_cadence FLOAT,
    device_watts BOOLEAN,
    average_watts FLOAT,
    pr_rank INT,
    kom_rank INT
);
