# Attendly
This is a simple project to count my attendance in school. It uses a clever logic that automatically registers attendance on startup.<br>
<br>

> **NOTE:**
> This project is still under development and thing are still missing. We will fix all of it eventually
> Please do not blame me in case the program does not work.

## How it works
The program checks if it is school time or not then if it is school time, it will add/increase the number in data.txt by one. This number indicates the number of days you DID NOT go to school.
This works in this way since if you went to school, you could not open up your pc (and this program could not be executed since it is meant to be executed on startup) and since you turned on your pc during school hours it registers a day off.

After that we can subtract the number of holidays and sundays from the number mentioned earlier to get the true number of days of. And once you know that, you can easily calculate you attendance% by calculating the number of working days divided by number of days off multiplied by 100.

## Building from source
To build it from source, you first need to make sure if rust is installed. You can do that buy running this command:
```bash
rustc --version
```
If it shows the version, you're good to go! If not, first install rust.
After that, you can clone the git repository with the following command:
```bash
git clone https://github.com/RayAntariksha/attendly.git
```
Then to compile it:
```bash
cd attendly
cargo build
```
You can run the program with
```bash
cargo run    # while inside the project directory
```
## Contributions
Any kind of contribution is appreciated. One can contribute by raising an issue, giving a pull request, etc. And if you are planning to contribute to this project, THANK YOU in advance!
