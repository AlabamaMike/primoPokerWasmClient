use yew::prelude::*;
use crate::components::common::*;

#[function_component(ComponentDemo)]
pub fn component_demo() -> Html {
    let show_modal = use_state(|| false);
    let toast_messages = use_state(|| Vec::<String>::new());

    let toggle_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_| {
            show_modal.set(!*show_modal);
        })
    };

    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |()| {
            show_modal.set(false);
        })
    };

    let add_toast = {
        let toast_messages = toast_messages.clone();
        Callback::from(move |_| {
            let mut messages = (*toast_messages).clone();
            messages.push(format!("Toast message #{}", messages.len() + 1));
            toast_messages.set(messages);
        })
    };

    let remove_toast = {
        let toast_messages = toast_messages.clone();
        Callback::from(move |index: usize| {
            let mut messages = (*toast_messages).clone();
            if index < messages.len() {
                messages.remove(index);
                toast_messages.set(messages);
            }
        })
    };

    html! {
        <div class="component-demo">
            <div class="demo-section">
                <h2>{"Enhanced UI Components Demo"}</h2>
                
                // Button Variants Demo
                <div class="demo-group">
                    <h3>{"Button Variants"}</h3>
                    <div class="button-grid">
                        <Button variant={ButtonType::Primary}>{"Primary"}</Button>
                        <Button variant={ButtonType::Secondary}>{"Secondary"}</Button>
                        <Button variant={ButtonType::Success}>{"Success"}</Button>
                        <Button variant={ButtonType::Danger}>{"Danger"}</Button>
                        <Button variant={ButtonType::Warning}>{"Warning"}</Button>
                        <Button variant={ButtonType::Info}>{"Info"}</Button>
                        <Button variant={ButtonType::Light}>{"Light"}</Button>
                        <Button variant={ButtonType::Dark}>{"Dark"}</Button>
                    </div>
                </div>

                // Button Sizes Demo
                <div class="demo-group">
                    <h3>{"Button Sizes"}</h3>
                    <div class="button-grid">
                        <Button size={ButtonSize::Small}>{"Small"}</Button>
                        <Button size={ButtonSize::Medium}>{"Medium"}</Button>
                        <Button size={ButtonSize::Large}>{"Large"}</Button>
                        <Button size={ButtonSize::ExtraLarge}>{"Extra Large"}</Button>
                    </div>
                </div>

                // Button States Demo
                <div class="demo-group">
                    <h3>{"Button States"}</h3>
                    <div class="button-grid">
                        <Button loading={true}>{"Loading"}</Button>
                        <Button disabled={true}>{"Disabled"}</Button>
                        <Button full_width={true}>{"Full Width"}</Button>
                    </div>
                </div>

                // Badge Demo
                <div class="demo-group">
                    <h3>{"Badges"}</h3>
                    <div class="badge-grid">
                        <Badge variant={BadgeType::Primary}>{"Primary"}</Badge>
                        <Badge variant={BadgeType::Success}>{"Success"}</Badge>
                        <Badge variant={BadgeType::Danger}>{"Danger"}</Badge>
                        <Badge variant={BadgeType::Warning}>{"Warning"}</Badge>
                        <Badge variant={BadgeType::Info}>{"Info"}</Badge>
                        <Badge variant={BadgeType::Primary} outline={true}>{"Outline"}</Badge>
                    </div>
                </div>

                // Card Demo
                <div class="demo-group">
                    <h3>{"Cards"}</h3>
                    <div class="card-grid">
                        <Card title={Some("Basic Card".to_string())}>
                            <p>{"This is a basic card component with a title."}</p>
                        </Card>
                        
                        <Card 
                            title={Some("Clickable Card".to_string())} 
                            subtitle={Some("Click me!".to_string())}
                            clickable={true}
                            onclick={Callback::from(|_| web_sys::console::log_1(&"Card clicked!".into()))}
                        >
                            <p>{"This card is clickable and has a subtitle."}</p>
                        </Card>
                    </div>
                </div>

                // Form Components Demo
                <div class="demo-group">
                    <h3>{"Form Components"}</h3>
                    <div class="form-demo">
                        <div class="form-field">
                            <label class="form-label">{"Username"}<span class="required">{"*"}</span></label>
                            <EnhancedInput 
                                placeholder={"Enter your username".to_string()}
                                value={""}
                                oninput={Callback::noop()}
                            />
                        </div>
                        
                        <div class="form-field">
                            <label class="form-label">{"Email"}<span class="required">{"*"}</span></label>
                            <EnhancedInput 
                                input_type={"email".to_string()}
                                placeholder={"Enter your email".to_string()}
                                value={"invalid-email"}
                                error={true}
                                oninput={Callback::noop()}
                            />
                            <div class="form-error">{"Please enter a valid email address"}</div>
                        </div>
                    </div>
                </div>

                // Loading Components Demo
                <div class="demo-group">
                    <h3>{"Loading Components"}</h3>
                    <div class="loading-demo">
                        <LoadingSpinner message={Some("Loading data...".to_string())} />
                        
                        <div class="skeleton-demo">
                            <h4>{"Skeleton Loading"}</h4>
                            <Skeleton width={200} height={20} count={3} />
                            <Skeleton width={50} height={50} circle={true} />
                        </div>
                    </div>
                </div>

                // Modal Demo
                <div class="demo-group">
                    <h3>{"Modal"}</h3>
                    <Button onclick={toggle_modal.clone()}>{"Open Modal"}</Button>
                    
                    <Modal 
                        is_open={*show_modal} 
                        title={Some("Demo Modal".to_string())}
                        on_close={close_modal}
                    >
                        <p>{"This is a modal dialog component."}</p>
                        <p>{"It supports titles, custom content, and accessibility features."}</p>
                        <div style="margin-top: 1rem;">
                            <Button variant={ButtonType::Primary}>{"Primary Action"}</Button>
                            <span style="margin-left: 1rem;">
                                <Button variant={ButtonType::Secondary}>{"Secondary Action"}</Button>
                            </span>
                        </div>
                    </Modal>
                </div>

                // Toast Demo
                <div class="demo-group">
                    <h3>{"Toast Notifications"}</h3>
                    <Button onclick={add_toast} variant={ButtonType::Info}>{"Add Toast"}</Button>
                </div>
            </div>

            // Simple Toast Display (custom implementation for demo)
            <div class="toast-container">
                {for toast_messages.iter().enumerate().map(|(index, message)| {
                    let remove_callback = {
                        let remove_toast = remove_toast.clone();
                        Callback::from(move |_| remove_toast.emit(index))
                    };
                    
                    html! {
                        <div key={index} class="toast toast-info">
                            <span class="toast-icon">{"ℹ"}</span>
                            <div class="toast-content">
                                <div class="toast-message">{message.clone()}</div>
                            </div>
                            <button class="toast-close" onclick={remove_callback}>{"×"}</button>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}