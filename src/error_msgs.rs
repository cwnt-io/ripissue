pub const ERROR_MSG_NOT_PROJECT_ROOT: &str = "
Ripissue Error: Not a Git repository.\n
\n
The 'ripi' command must be run within a Git repository.\n
A ripissue project needs:\n
\n
(1) the presence of a '.git' directory;\n
(2) to be executed at the project root;\n
\n
Please navigate to a ripissue project root directory and try again.\n
\n
If you believe this is a valid project directory, you can create an empty
\x20'.git' directory or initialize a new Git repository with 'git init'.\n
\n
For more information, visit https://github.com/cwnt-io/ripissue\n
";

pub const ERROR_MSG_CONFIG_FILE_ALREADY_EXISTS: &str = "
Error: The 'ripissue.toml' file already exists in the current directory.\n
\n
The 'ripi init' command is used to initialize a new ripissue project by
\x20creating a 'ripissue.toml' configuration file. However, it looks like this
\x20directory already contains a 'ripissue.toml' file.\n
\n
If you want to reinitialize the project with a new configuration, please delete
\x20the existing 'ripissue.toml' file first and then run 'ripi init' again.\n
\n
Alternatively, if you want to update the existing configuration, you can
\x20manually edit the 'ripissue.toml' file using a text editor.\n
\n
For more information and examples, please visit the ripissue documentation at
\x20https://github.com/cwnt-io/ripissue.\n
";
