CREATE TABLE activity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    activity_type VARCHAR(255) NOT NULL,
    activity_splits TEXT,
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
CREATE TABLE raw_activity (
    id BIGINT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    listed TEXT NOT NULL,
    activity TEXT,
    synced BOOLEAN NOT NULL,
    created_at DATETIME NOT NULL
);
