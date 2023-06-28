-- Your SQL goes here
CREATE TABLE activity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    activity_type VARCHAR(255) NOT NULL,
    distance FLOAT NOT NULL,
    moving_time INT NOT NULL,
    elapsed_time INT NOT NULL,
    total_elevation_gain FLOAT NOT NULL,
    sport_type VARCHAR(255) NOT NULL,
    average_heartrate FLOAT,
    max_heartrate FLOAT,
    start_date DATETIME,
    summary_polyline TEXT
);
CREATE TABLE raw_activity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    data TEXT NOT NULL,
    synced BOOLEAN NOT NULL,
    created_at DATETIME NOT NULL
);
