# Payment Data Service

A RESTful microservice for storing and managing payment events and user payment state from payment processors (e.g., Stripe) via webhooks. This service provides a centralized data store for payment events and tracks the current payment state of users for specific SKUs.

## Features

- **Payment Event Storage**: Store complete payment processor webhook events (e.g., Stripe events) with full event data
- **User Payment State Tracking**: Track current payment state per user and SKU, including:
  - Payment timestamps
  - Expiration dates
  - Refund information
  - Customer and product identifiers
- **RESTful API**: Clean REST endpoints for querying and managing payment data
- **PostgreSQL Backend**: Uses PostgreSQL with Knex.js for database migrations and queries
- **Docker Support**: Includes Docker and Docker Compose configuration for easy deployment
- **TypeScript**: Fully typed with TypeScript for better developer experience

## Prerequisites

- Node.js 22 or higher
- PostgreSQL database
- npm or yarn package manager

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd payment-data-service
```

2. Install dependencies:
```bash
npm install
```

3. Configure the database connection in `config.json`:
```json
{
  "server": {
    "port": 3000
  },
  "log": {
    "level": "debug"
  },
  "db": {
    "database": "payment-data-service",
    "host": "localhost",
    "port": "5432",
    "username": "your-username",
    "password": "your-password"
  }
}
```

4. Run database migrations:
```bash
npm run build
npm start
```

The service will automatically run migrations on startup.

## Usage

### Starting the Service

#### Development Mode
```bash
npm run watch  # Watch mode for TypeScript compilation
# In another terminal:
npm start
```

#### Production Mode
```bash
npm run build
npm start
```

#### Using Docker Compose
```bash
docker-compose up
```

The service will be available at `http://localhost:3000` (or port 3001 when using Docker Compose).

### API Endpoints

#### Health Check
```
GET /healthcheck
```
Returns `200 OK` if the service is running.

#### Payment Events

**Get a payment event by ID:**
```
GET /api/payment-event/:id
```

**Create a payment event:**
```
POST /api/payment-event
Content-Type: application/json

{
  "providerEventId": "evt_1234567890",
  "eventData": "{\"type\":\"payment_intent.succeeded\",\"data\":{...}}"
}
```

**Update a payment event:**
```
PUT /api/payment-event/:id
Content-Type: application/json

{
  "providerEventId": "evt_1234567890",
  "eventData": "{\"type\":\"payment_intent.succeeded\",\"data\":{...}}"
}
```

#### User Payment State

**Get user payment state by ID:**
```
GET /api/user-payment-state/:id
```

**Get all payment states for a user:**
```
GET /api/user-payment-state/byUserId/:userId
```

**Get payment state for a user and SKU:**
```
GET /api/user-payment-state/byUserIdAndSku/:userId/:sku
```

**Create a user payment state:**
```
POST /api/user-payment-state
Content-Type: application/json

{
  "userId": "user_123",
  "sku": "premium_monthly",
  "providerProductId": "prod_123",
  "providerPriceId": "price_123",
  "customerId": "cus_123",
  "plan": "premium",
  "paidAt": "2024-01-01T00:00:00Z",
  "expiresAt": "2024-02-01T00:00:00Z",
  "refundedAt": null
}
```

**Update a user payment state:**
```
PUT /api/user-payment-state/:id
Content-Type: application/json

{
  "userId": "user_123",
  "sku": "premium_monthly",
  "providerProductId": "prod_123",
  "providerPriceId": "price_123",
  "customerId": "cus_123",
  "plan": "premium",
  "paidAt": "2024-01-01T00:00:00Z",
  "expiresAt": "2024-02-01T00:00:00Z",
  "refundedAt": "2024-01-15T00:00:00Z"
}
```

## Database Schema

### paymentEvent Table
- `id` (UUID, Primary Key)
- `providerEventId` (String) - Event ID from the payment provider
- `eventData` (JSONB) - Full event data from the payment provider
- `createdAt` (Timestamp)
- `updatedAt` (Timestamp)

### userPaymentState Table
- `id` (UUID, Primary Key)
- `userId` (String) - User identifier
- `sku` (String) - Stock Keeping Unit identifier
- `providerProductId` (String) - Product ID from payment provider
- `providerPriceId` (String) - Price ID from payment provider
- `customerId` (String) - Customer ID from payment provider
- `plan` (String) - Plan name
- `paidAt` (Timestamp) - When payment was made
- `expiresAt` (Timestamp) - When payment expires
- `refundedAt` (Timestamp, Nullable) - When refund was issued
- `createdAt` (Timestamp)
- `updatedAt` (Timestamp)

## Development

### Scripts

- `npm test` - Run tests
- `npm testci` - Run tests in CI mode
- `npm lint` - Run ESLint
- `npm run build` - Build TypeScript to JavaScript
- `npm run build:esbuild` - Build optimized bundle with esbuild
- `npm run watch` - Watch mode for TypeScript compilation
- `npm start` - Start the server

### Project Structure

```
payment-data-service/
├── lib/
│   ├── model/          # Data models and database operations
│   ├── route/          # Express route handlers
│   ├── index.ts        # Route aggregator
│   ├── server.ts       # Server entry point
│   └── knexfile.ts     # Knex configuration
├── migrations/         # Database migrations
├── static/             # Static files
├── __tests__/          # Test files
├── config.json         # Configuration file
├── docker-compose.yml  # Docker Compose configuration
└── Dockerfile          # Docker build configuration
```

## Configuration

The service uses a `config.json` file for configuration. You can also override values using environment variables with the format `SECTION__KEY` (double underscore), following the structure of the JSON file.

Example environment variables:
- `SERVER__PORT=3000`
- `DB__HOST=localhost`
- `DB__DATABASE=payment-data-service`

For SSL database connections, additional configuration options are available:
- `DB__SSL__ENABLED=true`
- `DB__SSL__VERIFY=true`
- `DB__SSL__CA_CERT_FILE=/path/to/ca-cert.pem`

## License

Copyright 2024 Rick Knowles <rick.knowles@hikari-systems.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

