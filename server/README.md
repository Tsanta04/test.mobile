# Agricultural Monitoring System Backend

This is the backend API for an agricultural monitoring system built with Rust and Actix-web. The system allows farmers to monitor their agricultural lands using sensor data, track the health and productivity of their crops, and receive alerts about potential issues.

## Features

- User authentication and authorization with JWT
- Role-based access control (seller, buyer, supplier)
- CRUD operations for all entities
- Real-time sensor data monitoring
- Alert system for crop health, production, and other metrics
- Planning and scheduling for agricultural activities
- Discussion and messaging system for users

## Tech Stack

- **Language**: Rust
- **Web Framework**: Actix-web
- **Database**: PostgreSQL
- **ORM**: SQLx
- **Authentication**: JWT (JSON Web Tokens)
- **Validation**: Validator
- **Logging**: env_logger
- **Configuration**: dotenv

## Project Structure

```
server/
├── migrations/           # Database migration files
├── src/
│   ├── auth/             # Authentication and authorization
│   ├── dto/              # Data Transfer Objects
│   ├── error/            # Error handling
│   ├── handlers/         # Request handlers
│   ├── models/           # Database models
│   ├── routes/           # API routes
│   ├── services/         # Business logic
│   └── main.rs           # Application entry point
├── Cargo.toml            # Project dependencies
└── .env                  # Environment variables
```

## API Endpoints

The API provides the following endpoints:

- **Authentication**
  - `POST /api/auth/login`: User login
  - `POST /api/auth/register`: User registration

- **Users**
  - `POST /api/users`: Create a new user (admin only)
  - `GET /api/users`: Get all users (admin only)
  - `GET /api/users/me`: Get current user
  - `GET /api/users/{id}`: Get user by ID
  - `PUT /api/users/{id}`: Update user
  - `DELETE /api/users/{id}`: Delete user (admin only)

- **Persons**
  - `POST /api/persons`: Create a new person
  - `GET /api/persons`: Get all persons (admin only)
  - `GET /api/persons/{id}`: Get person by ID
  - `GET /api/persons/user/{id}`: Get person by user ID
  - `PUT /api/persons/{id}`: Update person
  - `DELETE /api/persons/{id}`: Delete person

- **Culture Types**
  - `POST /api/culture-types`: Create a new culture type (admin only)
  - `GET /api/culture-types`: Get all culture types
  - `GET /api/culture-types/{id}`: Get culture type by ID
  - `PUT /api/culture-types/{id}`: Update culture type (admin only)
  - `DELETE /api/culture-types/{id}`: Delete culture type (admin only)

- **Locations**
  - `POST /api/locations`: Create a new location (admin only)
  - `GET /api/locations`: Get all locations
  - `GET /api/locations/{id}`: Get location by ID
  - `PUT /api/locations/{id}`: Update location (admin only)
  - `DELETE /api/locations/{id}`: Delete location (admin only)

- **Sensor Types**
  - `POST /api/sensor-types`: Create a new sensor type (admin only)
  - `GET /api/sensor-types`: Get all sensor types
  - `GET /api/sensor-types/{id}`: Get sensor type by ID
  - `PUT /api/sensor-types/{id}`: Update sensor type (admin only)
  - `DELETE /api/sensor-types/{id}`: Delete sensor type (admin only)

- **Sensors**
  - `POST /api/sensors`: Create a new sensor (admin only)
  - `GET /api/sensors`: Get all sensors
  - `GET /api/sensors/{id}`: Get sensor by ID
  - `GET /api/sensors/type/{id}`: Get sensors by type
  - `PUT /api/sensors/{id}`: Update sensor (admin only)
  - `DELETE /api/sensors/{id}`: Delete sensor (admin only)

- **Sensor Packs**
  - `POST /api/sensor-packs`: Create a new sensor pack (admin only)
  - `GET /api/sensor-packs`: Get all sensor packs
  - `GET /api/sensor-packs/{id}`: Get sensor pack by ID
  - `PUT /api/sensor-packs/{id}`: Update sensor pack (admin only)
  - `DELETE /api/sensor-packs/{id}`: Delete sensor pack (admin only)

- **Packs**
  - `POST /api/packs`: Create a new pack (admin only)
  - `GET /api/packs`: Get all packs
  - `GET /api/packs/{id}`: Get pack by ID
  - `GET /api/packs/pack-id/{id}`: Get packs by pack ID
  - `GET /api/packs/sensor/{id}`: Get packs by sensor ID
  - `PUT /api/packs/{id}`: Update pack (admin only)
  - `DELETE /api/packs/{id}`: Delete pack (admin only)

- **Grounds**
  - `POST /api/grounds`: Create a new ground
  - `GET /api/grounds`: Get all grounds (admin only)
  - `GET /api/grounds/{id}`: Get ground by ID
  - `GET /api/grounds/user/{id}`: Get grounds by user ID
  - `GET /api/grounds/culture-type/{id}`: Get grounds by culture type
  - `GET /api/grounds/location/{id}`: Get grounds by location
  - `GET /api/grounds/pack/{id}`: Get grounds by pack
  - `PUT /api/grounds/{id}`: Update ground
  - `DELETE /api/grounds/{id}`: Delete ground

- **Discussions**
  - `POST /api/discussions`: Create a new discussion
  - `GET /api/discussions`: Get all discussions (admin only)
  - `GET /api/discussions/my`: Get discussions by participant
  - `GET /api/discussions/{id}`: Get discussion by ID
  - `PUT /api/discussions/{id}`: Update discussion
  - `DELETE /api/discussions/{id}`: Delete discussion (admin only)

- **Messages**
  - `POST /api/messages`: Create a new message
  - `GET /api/messages/{id}`: Get message by ID
  - `GET /api/messages/discussion/{id}`: Get messages by discussion
  - `PUT /api/messages/{id}`: Update message
  - `DELETE /api/messages/{id}`: Delete message

- **Participants**
  - `POST /api/participants`: Add a participant
  - `GET /api/participants/discussion/{id}`: Get participants by discussion
  - `DELETE /api/participants/{discussion_id}/{participant_id}`: Remove participant

- **States**
  - `POST /api/states`: Create a new state
  - `GET /api/states`: Get all states (admin only)
  - `GET /api/states/{id}`: Get state by ID
  - `GET /api/states/pack/{id}`: Get states by pack ID
  - `GET /api/states/pack/{id}/latest`: Get latest state by pack ID
  - `PUT /api/states/{id}`: Update state
  - `DELETE /api/states/{id}`: Delete state (admin only)

- **Alerts**
  - `POST /api/alerts`: Create a new alert (admin only)
  - `GET /api/alerts`: Get all alerts (admin only)
  - `GET /api/alerts/unseen`: Get unseen alerts
  - `GET /api/alerts/{id}`: Get alert by ID
  - `GET /api/alerts/state/{id}`: Get alerts by state
  - `GET /api/alerts/type/{type}`: Get alerts by type (admin only)
  - `GET /api/alerts/level/{level}`: Get alerts by level (admin only)
  - `PUT /api/alerts/{id}`: Update alert (admin only)
  - `PUT /api/alerts/{id}/seen`: Mark alert as seen
  - `DELETE /api/alerts/{id}`: Delete alert (admin only)

- **Plannings**
  - `POST /api/plannings`: Create a new planning
  - `GET /api/plannings`: Get all plannings (admin only)
  - `GET /api/plannings/date-range`: Get plannings by date range (admin only)
  - `GET /api/plannings/{id}`: Get planning by ID
  - `GET /api/plannings/ground/{id}`: Get plannings by ground
  - `PUT /api/plannings/{id}`: Update planning
  - `DELETE /api/plannings/{id}`: Delete planning

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL

### Installation

1. Clone the repository
2. Create a `.env` file in the root directory with the following variables:
   ```
   DATABASE_URL=postgres://username:password@localhost/database_name
   JWT_SECRET=your_jwt_secret
   ```
3. Run the database migrations:
   ```
   sqlx migrate run
   ```
4. Build and run the application:
   ```
   cargo run
   ```

## Authentication

The API uses JWT (JSON Web Tokens) for authentication. To access protected endpoints, you need to include the JWT token in the Authorization header:

```
Authorization: Bearer <token>
```

You can obtain a token by logging in with a valid username and password using the `/api/auth/login` endpoint.

## Error Handling

The API returns consistent error responses with appropriate HTTP status codes:

- `400 Bad Request`: Invalid input data
- `401 Unauthorized`: Missing or invalid authentication
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server-side error

## License

This project is licensed under the MIT License.

