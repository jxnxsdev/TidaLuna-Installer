export type Popup = {
    title: string;                                      // Title of the popup
    message: string;                                    // Message/body of the popup     
    type: 'info' | 'warning' | 'error' | 'success';     // Type of the popup, affects the icon shown
    buttons?: PopupButton[];                            // Optional array of buttons to show in the popup
}

export type PopupButton = {
    label: string;                                                           // Label of the button 
    action: 'open_url' | 'send_websocket_message' | 'send_api_request';      // Action to perform when the button is clicked
    value: string;                                                           // Value associated with the action (URL or message)    
    color: 'default' | 'primary' | 'danger';                                 // Color of the button
} 