-- Create the Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    google_id VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create the Itineraries table
CREATE TABLE itineraries (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    city_name VARCHAR(255) NOT NULL,
    itinerary JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create the cities table
CREATE TABLE cities (
    id SERIAL PRIMARY KEY,
    city_name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert 25 popular tourist destination cities
INSERT INTO cities (city_name) VALUES
-- Asia
('Tokyo'),
('Kyoto'),
('Bangkok'),
('Singapore'),
('Dubai'),

-- Europe
('Paris'),
('London'),
('Rome'),
('Barcelona'),
('Amsterdam'),

-- North America
('New York'),
('Los Angeles'),
('Las Vegas'),
('Toronto'),
('Mexico City'),

-- South America
('Rio de Janeiro'),
('Buenos Aires'),
('Lima'),
('Bogotá'),
('Santiago'),

-- Africa
('Cape Town'),
('Marrakech'),
('Cairo'),
('Nairobi'),
('Victoria Falls');