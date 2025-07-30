# Primo Poker WebAssembly Client

A modern poker client built with Rust and WebAssembly, using the Yew framework for reactive UI components.

## 🚀 Features

- **WebAssembly Performance**: Built with Rust for near-native performance in the browser
- **Reactive UI**: Powered by Yew framework with component-based architecture
- **Real-time Communication**: WebSocket integration for live poker gameplay
- **Authentication System**: Secure user authentication and session management
- **Professional Design**: Casino-quality visual theme with premium styling
- **Responsive Design**: Mobile-friendly layouts with modern CSS
- **Component Library**: Comprehensive UI components with accessibility features

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

## 🎨 Visual Design & Branding

The application features a professional casino-quality visual theme:

### Design System:
- **Color Palette**: Rich dark backgrounds with golden accents
- **Typography**: Premium fonts (Playfair Display, Inter, JetBrains Mono)
- **Logo System**: Professional text-based logo with gradient effects
- **Animations**: Smooth transitions with cubic-bezier easing

### Visual Features:
- **Glass Morphism**: Backdrop blur effects with elegant borders
- **Gradient Backgrounds**: Multi-layer gradients with texture overlays
- **Premium Shadows**: Layered shadows for depth and professionalism
- **Interactive Elements**: Hover effects with elevation and glow
- **Responsive Design**: Mobile-first approach with fluid layouts

### Brand Identity:
- **Primary Colors**: Dark slate (#0f172a) with poker green accents
- **Accent Gold**: Professional golden gradient (#fbbf24 → #d97706)
- **Casino Theme**: Poker-themed colors and premium aesthetics
- **Consistency**: Unified visual language across all components

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
- **Supporting Components (Phase 3) - COMPLETE**
  - Comprehensive UI component library with 8 button variants
  - Badge, Card, Modal, and Toast notification systems
  - Enhanced form components with validation
  - Loading states (spinner and skeleton)
  - Accessibility features (WCAG 2.1 compliance)
  - Dark mode and responsive design support
  - Component demo system at `/demo` route
- **Visual Enhancement & Branding (Phase 3.5) - COMPLETE ✅**
  - Professional logo suite with SVG assets and fallback text logo
  - Poker-themed color palette and premium design system
  - Enhanced CSS with glass morphism, gradients, and animations
  - Asset infrastructure for graphics, textures, and backgrounds
  - Google Fonts integration (Playfair Display, Inter, JetBrains Mono)
  - Professional header with golden gradient branding
  - Premium home page with hero section and feature cards
  - Responsive design with mobile-friendly layouts
  - Casino-quality visual theme throughout
- WebAssembly compilation and build system

**🚧 Next Phase:**
- **Phase 4: Game Interface Components**
  - Interactive poker table visualization with felt textures
  - Card display and animation system
  - Player action components and game controls
  - Advanced UI effects and transitions

**📋 Upcoming:**
- Phase 5: Backend integration and WebSocket implementation
- Phase 6: Advanced game features and tournament support
- Phase 7: Performance optimization and PWA features

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
