<h1>Attendly</h1>
This is a simple project to count my attendance in school. It uses a clever logic that automatically registers attendance on startup.

# How it works
The program checks if it is school time or not then if it is school time, it will add/increase the number in data.txt by one. This number indicates the number of days you DID NOT go to school.
This works in this way since if you went to school, you could not open up your pc (and this program could not be executed since it is meant to be executed on startup) and since you turned on your pc during school hours it registers a day off.

After that we can subtract the number of holidays and sundays from the number mentioned earlier to get the true number of days of. And once you know that, you can easily calculate you attendance% by calculating the number of working days divided by number of days off multiplied by 100.
