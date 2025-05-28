# SecureShareSystem

**SecureShareSystem** is a web-based application designed for securely sharing sensitive images. It prioritizes confidentiality and integrity through encryption and hashing, making it an ideal solution for photographers, journalists, medical professionals, or anyone needing to share private content securely.

## Table of Contents

- [Introduction](#introduction)
- [Key Features](#key-features)
- [Tech Stack](#tech-stack)
- [Setup and Installation](#setup-and-installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

SecureShareSystem enables secure image sharing with the following goals:
- **Security**: End-to-end encryption protects images during transmission and storage.
- **Usability**: An intuitive interface ensures seamless use on desktop and mobile devices.
- **Scalability**: The system supports multiple users and efficient image uploads.

## Key Features

### User Authentication
- **Secure Login/Registration**: Passwords are hashed with Argon2 for robust security.
- **Token Management**: JSON Web Tokens (JWTs) with expiration times authenticate sessions.

### Image Upload and Encryption
- **Upload**: Supports JPEG/PNG files with a 10MB size limit.
- **Encryption**: Client-side AES-256 encryption with RSA key management ensures privacy.

### Image Sharing
- **Unique Links/IDs**: Shareable links with optional expiration enhance security.
- **Access Control**: Only recipients with the private key can decrypt and view images.

### Image Viewing
- **Decryption**: Client-side decryption keeps unencrypted data off the server.
- **Viewer**: A secure, browser-based interface displays images without local storage.

### Integrity Check
- **Hashing**: SHA-256 hashes verify image integrity after decryption.

## Tech Stack

### Backend
- **Framework**: `actix-web` (Rust) for performance and security.
- **Database**: `sqlx` (async SQL toolkit) for reliable data management.
- **File Storage**: Local filesystem, with potential AWS S3 integration.

### Frontend
- **Framework**: ReactJS for a dynamic, component-based UI.
- **UI Components**: Custom or React Bootstrap for a polished design.
- **State Management**: React hooks (`useState`, `useContext`).
- **Encryption**: Web Crypto API for secure operations.
- **Communication**: HTTPS with TLS for secure backend interaction.

### Development Tools
- **Build Tools**: `cargo` (Rust), `npm` (JavaScript).
- **Version Control**: Git.
- **Deployment**: Docker for containerization.

## Setup and Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/yourusername/SecureShareSystem.git
   cd SecureShareSystem
   ```

2. **Backend Setup**:
   - Install Rust and Cargo.
   - In the backend directory, run:
     ```bash
     cargo build
     cargo run
     ```

3. **Frontend Setup**:
   - Install Node.js and npm.
   - In the frontend directory, run:
     ```bash
     npm install
     npm start
     ```

4. **Database**:
   - Configure a local SQL database and update backend connection settings.

## Usage

1. **Register** an account.
2. **Upload** an image (encrypted client-side).
3. **Share** the generated unique link with the recipient.
4. **View** the image by decrypting it with the recipient’s private key.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License.