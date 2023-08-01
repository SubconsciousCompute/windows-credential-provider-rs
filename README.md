# Windows Credential Provider in Rust
This project is a sample implementation of windows credential provider in the Rust programming language. It performs local user login using the MSV1_0 authentication package. It is meant to be used as a starting point for developing custom credential providers.

Note: This project is a very basic implementation created for learning purposes, and it may contain bugs. Feel free to provide feedback and report any issues you encounter. 

## Installation
Follow these steps to set up and install the credential provider:

1. **Generate a Unique GUID**: In order to avoid conflicts with other implementations,  you need to generate a unique GUID. You can use tools like GUID Generator to generate a new GUID. Update the GUID in the following files:
    - `register.rg`
    - `unregister.rg`
    - `lib.rs`


2. **Register the Credential Provider**: Run the register.rg script on the target machine to register the credential provider.

3. **Build and Copy the DLL**: Build your project to generate the DLL file. After successful compilation, copy the DLL to the System32 folder of the target machine.


4. **Verify the Installation**: Once the machine restarts, your custom credential provider should be available in the login screen options.

![Screenshot of WCP](/assets/wcp_image.png)

## Debugging
Refer to this [blog](https://blog.subcom.tech/setting-up-a-windows-machine-for-drivers-and-minifilters-testing-and-debugging-using-virtualkd-redux/) for setting up debugging. 


## Contributing
If you would like to contribute to this project, we appreciate your interest! Make sure to raise an issue before starting.

## Acknowledgements
 This project was inspired by an [answer](https://stackoverflow.com/a/75290255/22244436) on Stack Overflow provided by [IInspectable](https://stackoverflow.com/users/1889329/iinspectable). We are grateful for the valuable insights that served as a starting point for our work.