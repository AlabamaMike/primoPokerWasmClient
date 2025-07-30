# Primo Poker WebAssembly Client

A modern poker client built with Rust and WebAssembly, using the Yew framework for reactive UI components.

## 🚀 Features

- **WebAssembly Performance**: Built with Rust for near-native performance in the browser
- **Reactive UI**: Powered by Yew framework with component-based architecture
- **Real-time Communication**: WebSocket integration for live poker gameplay
- **Authentication System**: Secure user authentication and session management
- **Responsive Design**: Modern CSS with mobile-friendly layouts

## 🛠️ Technology Stack

- **Language**: Rust
- **Frontend Framework**: Yew v0.21
- **Build Tool**: Cargo (WebAssembly target)
- **Target**: `wasm32-unknown-unknown`
- **Styling**: CSS3 with modern layouts
- **Communication**: WebSockets via gloo-net

## 📋 Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- Modern web browser with WebAssembly support

## 🔧 Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd primoPokerWasmClient
```

2. Install the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Build the project:
```bash
cargo build --target wasm32-unknown-unknown
```

## 🏗️ Project Structure

```
primoPokerWasmClient/
├── src/
│   ├── app.rs              # Main application component
│   ├── auth.rs             # Authentication logic
│   ├── game.rs             # Game state management
│   ├── lib.rs              # Library root
│   ├── types.rs            # Type definitions
│   ├── components/         # UI components
│   │   ├── mod.rs
│   │   ├── common.rs       # Shared components
│   │   ├── auth.rs         # Authentication components
│   │   ├── lobby.rs        # Lobby interface
│   │   ├── game.rs         # Game interface
│   │   ├── pages.rs        # Page components
│   │   └── profile.rs      # Profile components
│   ├── services/           # Business logic services
│   │   ├── mod.rs
│   │   ├── auth_service.rs # Authentication service
│   │   └── websocket_service.rs # WebSocket handling
│   ├── networking/         # Network communication
│   │   └── mod.rs
│   ├── graphics/           # Graphics and rendering
│   │   └── mod.rs
│   └── utils/              # Utility functions
│       └── mod.rs
├── styles/
│   └── main.css            # Main stylesheet
├── Cargo.toml              # Project configuration
├── Trunk.toml              # Trunk build configuration
└── index.html              # HTML entry point
```

## 🔑 Key Dependencies

- **yew** v0.21 - React-like framework for Rust/WebAssembly
- **gloo-net** - HTTP requests and WebSocket communication
- **serde** - Serialization framework
- **uuid** - UUID generation with serde support
- **chrono** - Date and time handling
- **yew-router** - Client-side routing

## 🎮 Development

### Building for Development
```bash
cargo build --target wasm32-unknown-unknown
```

### Building for Release
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Running Tests
```bash
cargo test
```

## 🌐 Architecture

### Component Architecture
The application follows a component-based architecture with:

- **App Component**: Root component handling routing and global state
- **Page Components**: Auth, Lobby, Game, Profile pages
- **Service Layer**: Authentication and WebSocket services
- **State Management**: Centralized game state and user session

### Communication Flow
1. **Authentication**: HTTP requests for login/register
2. **Real-time Updates**: WebSocket connection for game events
3. **State Updates**: Reactive UI updates via Yew's virtual DOM

## 🔐 Authentication

The authentication system includes:
- ✅ User registration with validation
- ✅ User login with "remember me" functionality  
- ✅ JWT token management (mock implementation)
- ✅ Session persistence via browser storage
- ✅ Authentication-aware routing and navigation
- ✅ Professional UI with loading states and error handling
- 🔄 Token refresh mechanism (ready for backend)

### Authentication Features:
- **LoginPage**: Username/email + password with validation
- **RegisterPage**: Full registration form with real-time validation
- **Session Management**: Persistent login across browser sessions
- **Security**: Form validation, error handling, logout functionality

## 🎯 Current Status

**✅ Completed:**
- Project setup and configuration
- Dependency management
- Component structure framework
- **Authentication system (Phase 1) - COMPLETE**
  - Full LoginPage and RegisterPage components
  - Form validation and error handling
  - Session management and persistence
  - Authentication-aware navigation
- **Lobby interface (Phase 2) - COMPLETE**
  - Room list display and filtering
  - Create room functionality
  - Mock real-time updates
  - Authentication protection
- WebAssembly compilation and build system

**🚧 In Progress:**
- **Phase 3: Supporting Components (UI/UX polish)**
- Component library development
- Enhanced user experience features

**📋 Upcoming:**
- Phase 4: Enhanced lobby features
- Phase 5: Backend integration
- Phase 6: Game room functionality
- Phase 7: Real WebSocket implementation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🐛 Known Issues

- ~~Trunk build tool integration pending~~ ✅ **RESOLVED**
- ~~Some placeholder components need implementation~~ ✅ **RESOLVED**
- WebSocket service requires backend integration (mock implementation working)
- Component library needs expansion for Phase 3

## 📞 Support

For support, please open an issue on GitHub or contact the development team.

---

Built with ❤️ using Rust and WebAssembly
