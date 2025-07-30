# Primo Poker Client Implementation Plan

## User Journey Flow

Following the natural user experience from initial visit to playing poker:

1. **Landing/Home** → **Login** → **Lobby** → **Game Room** → **Profile**

## Phase 1: Authentication Components

### 1.1 Login Component (`src/components/auth.rs`)

#### User Experience Flow:
- User visits `/login` or clicks "Login" from home page
- Sees login form with username/email and password fields
- Can toggle to "Remember Me" option
- Can navigate to registration page
- On successful login, redirects to lobby
- On error, shows validation messages

#### Implementation Requirements:

**State Management:**
```rust
pub struct LoginPageState {
    credentials: LoginCredentials,
    loading: bool,
    error_message: Option<String>,
    remember_me: bool,
    show_password: bool,
}
```

**Key Features:**
- Input validation (client-side)
- Loading states during authentication
- Error handling with user-friendly messages
- "Forgot Password" link preparation
- Responsive design for mobile/desktop
- Password visibility toggle
- Form submission on Enter key

**Integration Points:**
- Uses `AuthService::login()` 
- Sends `AppMsg::UserLoggedIn` on success
- Redirects to `/lobby` after successful authentication
- Integrates with `yew-router` for navigation

#### Implementation Priority: **HIGH** ⭐⭐⭐
*Essential for user onboarding*

### 1.2 Registration Component (`src/components/auth.rs`)

#### User Experience Flow:
- User clicks "Sign Up" from login page or home
- Fills out registration form (username, email, password, confirm password, display name)
- Sees real-time validation feedback
- Agrees to terms of service
- On successful registration, automatically logs in and redirects to lobby

#### Implementation Requirements:

**State Management:**
```rust
pub struct RegisterPageState {
    form_data: RegisterData,
    confirm_password: String,
    loading: bool,
    errors: HashMap<String, String>,
    terms_accepted: bool,
}
```

**Key Features:**
- Real-time validation (username availability, email format, password strength)
- Password confirmation matching
- Terms of service checkbox
- Username availability checking
- Strong password requirements indicator
- Success confirmation with auto-login

#### Implementation Priority: **HIGH** ⭐⭐⭐
*Essential for user acquisition*

## Phase 2: Lobby Component

### 2.1 Main Lobby Interface (`src/components/lobby.rs`)

#### User Experience Flow:
- User arrives after successful login
- Sees welcome message with username and chip count
- Views list of available game rooms
- Can filter/sort rooms by stakes, player count, game type
- Can create new room or join existing room
- Sees real-time updates of room availability
- Can access profile and settings

#### Implementation Requirements:

**State Management:**
```rust
pub struct LobbyPageState {
    user: User,
    available_rooms: Vec<GameRoom>,
    filtered_rooms: Vec<GameRoom>,
    loading: bool,
    websocket_connected: bool,
    filter_criteria: RoomFilter,
    show_create_room_modal: bool,
}
```

**Key Components:**
1. **Header Section**: User info, chips, logout button
2. **Room List**: Sortable/filterable table of game rooms
3. **Room Filters**: Stakes, player count, game type filters  
4. **Create Room**: Modal for creating new game room
5. **Quick Join**: Button for fastest available game

**Real-time Features:**
- WebSocket connection for live room updates
- Player count changes in real-time
- New rooms appear automatically
- Notifications for room availability

**Integration Points:**
- Uses `WebSocketService` for real-time updates
- Connects to `/lobby` WebSocket endpoint
- Handles room join/create via WebSocket messages
- Navigates to `/game/:room_id` on room join

#### Implementation Priority: **HIGH** ⭐⭐⭐
*Core platform functionality*

## Phase 3: Supporting Components

### 3.1 Common Components (`src/components/common.rs`)

#### Required Components:

**LoadingSpinner**: 
- Reusable across login, lobby, game transitions
- Shows during authentication, room joining, etc.

**Header Component**:
- Navigation bar with user info when authenticated
- Login/Register buttons when unauthenticated
- Responsive mobile menu

**Footer Component**:
- Links to terms, privacy, support
- Copyright information

**Button Components**:
- Primary, secondary, danger button variants
- Loading states with spinners
- Disabled states

**Form Components**:
- Input fields with validation styling
- Error message display
- Label components

#### Implementation Priority: **MEDIUM** ⭐⭐
*Supports main components*

## Phase 4: Enhanced Lobby Features

### 4.1 Room Management

**Create Room Modal**:
```rust
pub struct CreateRoomModalState {
    room_config: RoomConfiguration,
    loading: bool,
    errors: HashMap<String, String>,
}
```

**Features:**
- Game type selection (Texas Hold'em, Omaha, etc.)
- Blinds/stakes configuration
- Player limit settings
- Private room options
- Buy-in amounts

**Room Filtering System**:
- Stakes range slider
- Player count filter
- Game type dropdown
- "Empty rooms" toggle
- "Full rooms" toggle

### 4.2 Social Features

**Player List Sidebar**:
- Online friends
- Recent players
- Player stats preview

**Chat System** (Future):
- Lobby chat
- Private messaging preparation

#### Implementation Priority: **MEDIUM** ⭐⭐
*Enhances user experience*

## Technical Implementation Details

### State Management Strategy

**Global App State** (in `app.rs`):
- Authentication state
- User information
- WebSocket connection status
- Global error handling

**Component-Level State**:
- Form data and validation
- Loading states
- Local UI state

### WebSocket Integration

**Connection Management**:
- Auto-reconnection logic
- Connection status indicators
- Graceful degradation for connection issues

**Message Types**:
```rust
pub enum LobbyMessage {
    RoomListUpdate(Vec<GameRoom>),
    RoomCreated(GameRoom),
    RoomDestroyed(String), // room_id
    PlayerJoined { room_id: String, player: User },
    PlayerLeft { room_id: String, player_id: Uuid },
}
```

### CSS/Styling Structure

**Component-Based Styles**:
- `styles/components/auth.css`
- `styles/components/lobby.css`
- `styles/components/common.css`

**Design System**:
- Color palette for poker theme
- Typography scale
- Spacing system
- Responsive breakpoints

### Error Handling Strategy

**User-Friendly Messages**:
- Network errors: "Connection issue, please try again"
- Validation errors: Field-specific messages
- Server errors: "Something went wrong, please contact support"

**Error Recovery**:
- Retry mechanisms for network calls
- Fallback UI states
- Clear error dismissal actions

## Implementation Timeline

### Week 1: Authentication Foundation
- [ ] Login component with basic form
- [ ] Registration component with validation
- [ ] Integration with AuthService
- [ ] Basic routing between auth pages

### Week 2: Lobby Core Functionality  
- [ ] Basic lobby layout with room list
- [ ] WebSocket connection setup
- [ ] Room join functionality
- [ ] User info display

### Week 3: Enhanced Features
- [ ] Room filtering and sorting
- [ ] Create room modal
- [ ] Real-time updates
- [ ] Error handling and loading states

### Week 4: Polish and Testing
- [ ] Responsive design implementation
- [ ] Accessibility improvements
- [ ] End-to-end testing
- [ ] Performance optimization

## Success Metrics

**Authentication Success**:
- ✅ Users can register new accounts
- ✅ Users can log in with existing credentials
- ✅ Form validation works correctly
- ✅ Error messages are user-friendly

**Lobby Functionality**:
- ✅ Users see available game rooms
- ✅ Real-time updates work correctly
- ✅ Room filtering functions properly
- ✅ Users can join games successfully

**Technical Quality**:
- ✅ Components are reusable and maintainable
- ✅ State management is predictable
- ✅ WebSocket connection is stable
- ✅ Error handling is comprehensive

## Next Steps

1. **Start with Login Component**: Implement the login form with basic functionality
2. **Add Form Validation**: Client-side validation with error display
3. **Integrate AuthService**: Connect form submission to authentication service
4. **Implement Lobby Basic Layout**: Create the room list and user info display
5. **Add WebSocket Connection**: Enable real-time lobby updates

This plan prioritizes the core user journey while building a solid foundation for future features. Each component is designed to be modular and reusable across the application.
