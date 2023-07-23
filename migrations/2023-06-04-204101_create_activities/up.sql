-- Your SQL goes here
CREATE TABLE activity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    activity_type VARCHAR(255) NOT NULL,
    distance FLOAT NOT NULL,
    moving_time INT NOT NULL,
    elapsed_time INT NOT NULL,
    total_elevation_gain FLOAT NOT NULL,
    sport_type VARCHAR(255) NOT NULL,
    average_heartrate FLOAT,
    max_heartrate FLOAT,
    start_date DATETIME,
    summary_polyline TEXT,
    average_cadence FLOAT,
    average_speed FLOAT,
    kudos INT NOT NULL,
    location_country VARCHAR(255),
    location_state VARCHAR(255),
    location_city VARCHAR(255),
    athletes INT NOT NULL
);
CREATE TABLE activity_split (
    activity_id BIGINT NOT NULL,
    distance FLOAT NOT NULL,
    moving_time INT NOT NULL,
    elapsed_time INT NOT NULL,
    average_speed FLOAT NOT NULL,
    elevation_difference FLOAT NOT NULL,
    split INT NOT NULL,
    FOREIGN KEY (activity_id) REFERENCES activity (id),
    PRIMARY KEY (activity_id, split)
);
CREATE TABLE raw_activity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    listed TEXT NOT NULL,
    activity TEXT,
    synced BOOLEAN NOT NULL,
    created_at DATETIME NOT NULL
);
