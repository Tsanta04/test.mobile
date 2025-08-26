# Agricultural Management System Backend

This is a backend application for an agricultural management system built with Rust, Actix-web, and Diesel ORM. The system allows for management of agricultural grounds, sensors, and monitoring of various agricultural metrics.

## Features

- User authentication and authorization with JWT
- Role-based access control (admin, seller, buyer, supplier)
- CRUD operations for all entities
- Sensor data management
- Ground monitoring
- Alert system
- Planning system
- Messaging system

## Database Schema

The database consists of the following tables:
- Users: User management with role-based access
- Person: Personal information linked to users
- Culture_type: Types of cultures
- Location: Geographical data
- Sensor_type and Sensor: Sensor management
- Sensor_pack and Pack: Sensor packaging system
- Ground: Agricultural grounds management
- Discussion, Message, and Participant: Communication system
- State: Monitoring states
- Alert: Alert system with custom types
- Planning: Scheduling system

## Project Structure

```
server/
├── migrations/        # Database migrations
├── src/
│   ├── dto/           # Data Transfer Objects
│   ├── handlers/      # HTTP request handlers
│   ├── models/        # Database models
│   ├── routes/        # API routes
│   ├── services/      # Business logic
│   ├── errors.rs      # Error handling
│   ├── main.rs        # Application entry point
│   └── schema.rs      # Database schema
└── Cargo.toml         # Project dependencies
```

## Setup and Installation

### Prerequisites

- Rust and Cargo
- PostgreSQL

### Steps

1. Clone the repository:
   ```
   git clone <repository-url>
   cd agricultural-management-system
   ```

2. Set up the database:
   ```
   createdb agricultural_db
   ```

3. Configure environment variables:
   Create a `.env` file in the project root with the following content:
   ```
   DATABASE_URL=postgres://username:password@localhost/agricultural_db
   JWT_SECRET=your_jwt_secret_key
   RUST_LOG=debug
   ```

4. Run database migrations:
   ```
   cd server
   diesel migration run
   ```

5. Build and run the application:
   ```
   cargo run
   ```

The server will start at `http://localhost:8080`.

## API Endpoints

### Authentication
- `POST /api/auth/register`: Register a new user
- `POST /api/auth/login`: Login and get JWT token

### Users
- `GET /api/users`: Get all users (admin only)
- `GET /api/users/{id}`: Get user by ID
- `PUT /api/users/{id}`: Update user
- `DELETE /api/users/{id}`: Delete user
- `GET /api/users/me`: Get current user

### Persons
- `POST /api/persons`: Create a new person
- `GET /api/persons`: Get all persons (admin only)
- `GET /api/persons/{id}`: Get person by ID
- `PUT /api/persons/{id}`: Update person
- `DELETE /api/persons/{id}`: Delete person
- `GET /api/persons/user/{user_id}`: Get person by user ID

### Culture Types
- `POST /api/culture-types`: Create a new culture type (admin only)
- `GET /api/culture-types`: Get all culture types
- `GET /api/culture-types/{id}`: Get culture type by ID
- `PUT /api/culture-types/{id}`: Update culture type (admin only)
- `DELETE /api/culture-types/{id}`: Delete culture type (admin only)

### Locations
- `POST /api/locations`: Create a new location (admin or seller)
- `GET /api/locations`: Get all locations
- `GET /api/locations/{id}`: Get location by ID
- `PUT /api/locations/{id}`: Update location (admin or seller)
- `DELETE /api/locations/{id}`: Delete location (admin only)

### Sensor Types
- `POST /api/sensor-types`: Create a new sensor type (admin or supplier)
- `GET /api/sensor-types`: Get all sensor types
- `GET /api/sensor-types/{id}`: Get sensor type by ID
- `PUT /api/sensor-types/{id}`: Update sensor type (admin or supplier)
- `DELETE /api/sensor-types/{id}`: Delete sensor type (admin only)

### Sensors
- `POST /api/sensors`: Create a new sensor (admin or supplier)
- `GET /api/sensors`: Get all sensors
- `GET /api/sensors/{id}`: Get sensor by ID
- `PUT /api/sensors/{id}`: Update sensor (admin or supplier)
- `DELETE /api/sensors/{id}`: Delete sensor (admin only)
- `GET /api/sensors/type/{type_id}`: Get sensors by type

## Security

- JWT-based authentication
- Password hashing with Argon2
- Role-based access control
- Input validation
- Error handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.

