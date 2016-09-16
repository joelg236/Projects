Learning languages and their tools using some preset "easy" projects

* The projects will not be done in the order posted
* I may not complete them all
* My method of solving them will not be the best
* I will not account for heavy error handling or invalid input

There is no code in the master branch - I'm separating different languages in
branches of their own name.

Numbers
-------

**Find PI to the Nth Digit** – Enter a number and have the program generate PI up to that many decimal places. Keep a limit to how far the program will go.

**Fibonacci Sequence** – Enter a number and have the program generate the Fibonacci sequence to that number or to the Nth number.

**Prime Factorization** – Have the user enter a number and find all Prime Factors (if there are any) and display them.

**Next Prime Number** – Have the program find prime numbers until the user chooses to stop asking for the next one.

**Find Cost of Tile to Cover W x H Floor** – Calculate the total cost of tile it would take to cover a floor plan of width and height, using a cost entered by the user.

**Change Return Program** – The user enters a cost and then the amount of money given. The program will figure out the change and the number of quarters, dimes, nickels, pennies needed for the change.

**Binary to Decimal and Back Converter** – Develop a converter to convert a decimal number to binary or a binary number to its decimal equivalent.

**Calculator** – A simple calculator to do basic operators. Make it a scientific calculator for added complexity.

**Alarm Clock** – A simple clock where it plays a sound after X number of minutes/seconds or at a particular time.

**Credit Card Validator** – Takes in a credit card number from a common credit card vendor (Visa, MasterCard, American Express, Discoverer) and validates it to make sure that it is a valid number (look into how credit cards use a checksum).


Text
----

**Reverse a String** – Enter a string and the program will reverse it and print it out.

**Pig Latin** – Pig Latin is a game of alterations played on the English language game. To create the Pig Latin form of an English word the initial consonant sound is transposed to the end of the word and an ay is affixed (Ex.: "banana" would yield anana-bay). Read Wikipedia for more information on rules.

**Count Vowels** – Enter a string and the program counts the number of vowels in the text. For added complexity have it report a sum of each vowel found.

**Check if Palindrome** – Checks if the string entered by the user is a palindrome. That is that it reads the same forwards as backwards like “racecar”

**Count Words in a String** – Counts the number of individual words in a string. For added complexity read these strings in from a text file and generate a summary.

**Vigenere / Vernam / Ceasar Ciphers** – Functions for encrypting and decrypting data messages. Then send them to a friend.

**Text to HTML Generator** – Converts text files into web HTML files and stylizes them. Great for making online documentation of standard text documentation.

**Regex Query Tool** – A tool that allows the user to enter a text string and then in a separate control enter a regex pattern. It will run the regular expression against the source text and return any matches or flag errors in the regular expression.


Networking
----------

**FTP Program** – A file transfer program which can transfer files back and forth from a remote web sever.

**Get Atomic Time from Internet Clock** – This program will get the true atomic time from an atomic time clock on the Internet. There are various clocks across the world. Do a search for a list of them.

**Fetch Current Weather** – Get the current weather for a given zip/postal code.

**Country from IP Lookup** – Enter an IP address and find the country that IP is registered in.

**Whois Search Tool** – Enter an IP or host address and have it look it up through whois and return the results to you.

**Small Web Server** – A simple web server that can serve HTML files that contain Javascript and other forms of non-code executing code. Added complexity would be to try and implement streaming video, create a server-side language, or serve up other stream types.


Data
----

**Product Inventory Project** – Create an application which manages an inventory of products. Create a product class which has a price, id, and quantity on hand. Then create an inventory class which keeps track of various products and can sum up the inventory value.

**Recipe Creator and Manager** – Create a recipe class with ingredients and a put them in a recipe manager program that organizes them into categories like deserts, main courses or by ingredients like chicken, beef, soups, pies etc.

**Class to Handle Large Numbers** – We know that the basic data types like integer, long, double, and floats only go so far. Create a class that can manage extremely large numbers like those used in space exploration.

**Matrix Class** – A class to manage matrices. Add, subtract and multiple matrices.


Threading
---------

**Create A Progress Bar for Downloads** – Create a progress bar for applications that can keep track of a download in progress. The progress bar will be on a separate thread and will communicate with the main thread using delegates.

**Download Manager** – Allow your program to download various files and each one is downloading in the background on a separate thread. The main thread will keep track of the other thread’s progress and notify the user when downloads are completed.


Web
---

**Telnet Application** – Create an application which can telnet into servers across the internet and run basic commands.

**Bandwidth Monitor** – A small utility program that tracks how much data you have uploaded and downloaded from the net during the course of your current online session. See if you can find out what periods of the day you use more and less and generate a report or graph that shows it.

**CAPTCHA Maker** – Ever see those images with letters a numbers when you signup for a service and then asks you to enter what you see? It keeps web bots from automatically signing up and spamming. Try creating one yourself for online forms. If you use PHP, take a look at the image functions of GD.


Databases
---------

**Remote SQL Tool** – A utility that can execute queries on remote servers from your local computer across the Internet. It should take in a remote host, user name and password, run the query and return the results.

**TV Show Tracker** – Got a favorite show you don’t want to miss? Don’t have a PVR or want to be able to find the show to then PVR it later? Make an application which can search various online TV Guide sites, locate the shows/times/channels and add them to a database application. The database/website then can send you email reminders that a show is about to start and which channel it will be on.


Graphics and Multimedia
---------

**Slide Show** – Make an application that shows various pictures in a slide show format. For extra complexity try adding various effects like fade in/out, star wipe and window blinds transitions.

**Stream Video from Online** – Try to create your own online streaming video player.

**Traffic Light Application** – See if you can make your own street light application and then put it into an intersection scenario. Don’t let any cars run the lights and crash into one another!

**Turtle Graphics** – This is a common project where you create a floor of 20 x 20 squares. Using various commands you tell a turtle to draw a line on the floor. You have move forward, left or right, lift or drop pen etc. For added complexity, allow the program to read in the list of commands from a file. Do a search online for “Turtle Graphics” for more information.
