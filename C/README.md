Martyr2’s Mega Project List
========

Trying to complete all projects from [Martyr2’s Mega Project List](http://www.dreamincode.net/forums/topic/78802-martyr2s-mega-project-ideas-list/).

**Note**: If you fork this repo to solve these projects in any language of your choice, please remove all my code, and start from scratch; you'll benefit a lot. Do ***not*** send pull requests.

Some details:

* I will use C to solve these.
* I have no interest in making games, so I'm excluding those from the list below.
* I'm not interested in networking, so I *might* skip some of them.
* The projects will not be made in the order posted.
* I may not be able to complete all of them. 
* My method of solving them may not be the best.
* I will be treating each project as a script and they will not be designd to becallable from other scripts.
* I will not account for mallicious use of programs. Only qualification is to work. I'm learning, not perfecting.

I will link to each project that I complete. Some will be in this same repo, some bigger ones will have dedicated repos.

==============================

Numbers
---------

[**Find PI to the Nth Digit**](https://github.com/joelg236/Projects/blob/master/C/Numbers/pi.c) – Enter a number and have the program generate PI up to that many decimal places. Keep a limit to how far the program will go.

[**Fibonacci Sequence**](https://github.com/joelg236/Projects/blob/master/C/Numbers/fibonacci.c) – Enter a number and have the program generate the Fibonacci sequence to that number or to the Nth number.

[**Prime Factorization**](https://github.com/joelg236/Projects/blob/master/C/Numbers/primes.c) – Have the user enter a number and find all Prime Factors (if there are any) and display them.

[**Next Prime Number**](https://github.com/joelg236/Projects/blob/master/C/Numbers/nextPrime.c) – Have the program find prime numbers until the user chooses to stop asking for the next one.

[**Find Cost of Tile to Cover W x H Floor**](https://github.com/joelg236/Projects/blob/master/C/Numbers/tile.c) – Calculate the total cost of tile it would take to cover a floor plan of width and height, using a cost entered by the user.

[**Change Return Program**](https://github.com/joelg236/Projects/blob/master/C/Numbers/changeReturn.c) – The user enters a cost and then the amount of money given. The program will figure out the change and the number of quarters, dimes, nickels, pennies needed for the change.

**Binary to Decimal and Back Converter** – Develop a converter to convert a decimal number to binary or a binary number to its decimal equivalent.

**Calculator** – A simple calculator to do basic operators. Make it a scientific calculator for added complexity.

**Alarm Clock** – A simple clock where it plays a sound after X number of minutes/seconds or at a particular time.

**Credit Card Validator** – Takes in a credit card number from a common credit card vendor (Visa, MasterCard, American Express, Discoverer) and validates it to make sure that it is a valid number (look into how credit cards use a checksum).

**Dijkstra’s Algorithm** – Create a program that finds the shortest path through a graph using its edges.

Text
---------

[**Reverse a String**](https://github.com/joelg236/Projects/blob/master/C/Text/reverseString.c) – Enter a string and the program will reverse it and print it out.

[**Pig Latin**](https://github.com/joelg236/Projects/blob/master/C/Text/piglatin.c) – Pig Latin is a game of alterations played on the English language game. To create the Pig Latin form of an English word the initial consonant sound is transposed to the end of the word and an ay is affixed (Ex.: "banana" would yield anana-bay). Read Wikipedia for more information on rules.

[**Count Vowels**](https://github.com/joelg236/Projects/blob/master/C/Text/countVowels.c) – Enter a string and the program counts the number of vowels in the text. For added complexity have it report a sum of each vowel found.

**Check if Palindrome** – Checks if the string entered by the user is a palindrome. That is that it reads the same forwards as backwards like “racecar”

**Count Words in a String** – Counts the number of individual words in a string. For added complexity read these strings in from a text file and generate a summary.

**Text Editor** – Notepad style application that can open, edit, and save text documents. Add syntax highlighting and other features.

**Vigenere / Vernam / Ceasar Ciphers** – Functions for encrypting and decrypting data messages. Then send them to a friend.

**Text to HTML Generator** – Converts text files into web HTML files and stylizes them. Great for making online documentation of standard text documentation.

**CD Key Generator** – Generates a unique key for your applications to use based on some arbitrary algorithm that you can specify. Great for software developers looking to make shareware that can be activated.

**Regex Query Tool** – A tool that allows the user to enter a text string and then in a separate control enter a regex pattern. It will run the regular expression against the source text and return any matches or flag errors in the regular expression.

Networking
---------

**FTP Program** – A file transfer program which can transfer files back and forth from a remote web sever.

**Get Atomic Time from Internet Clock** – This program will get the true atomic time from an atomic time clock on the Internet. There are various clocks across the world. Do a search for a list of them.

**Chat Application (IRC or MSN Style)** – Create a chat application that can create simple chat rooms like on Internet Relay Chat (IRC) or a more direct chatting style like MSN. For added complexity, create your own protocol to facilitate this chatting.

**Fetch Current Weather** – Get the current weather for a given zip/postal code.

**Port Scanner** – Enter an IP address and a port range where the program will then attempt to find open ports on the given computer by connecting to each of them. On any successful connections mark the port as open.

**Mail Checker (POP3 / IMAP)** – The user enters various account information include web server and IP, protocol type (POP3 or IMAP) and the application will check for email on several accounts at a given interval.

**Packet Sniffer** – A utility program that will read packets coming in and out of the machine along with related information like destination and payload size.

**Country from IP Lookup** – Enter an IP address and find the country that IP is registered in.

**Whois Search Tool** – Enter an IP or host address and have it look it up through whois and return the results to you.

**Site Checker with Time Scheduling** – An application that attempts to connect to a website or server every so many minutes or a given time and check if it is up. If it is down, it will notify you by email or by posting a notice on screen.

**Small Web Server** – A simple web server that can serve HTML files that contain Javascript and other forms of non-code executing code. Added complexity would be to try and implement streaming video, create a server-side language, or serve up other stream types.

Classes
---------

**Product Inventory Project** – Create an application which manages an inventory of products. Create a product class which has a price, id, and quantity on hand. Then create an inventory class which keeps track of various products and can sum up the inventory value.

**Movie Store** – Manage video rentals and controls when videos are checked out, due to return, overdue fees and for added complexity create a summary of those accounts which are overdue for contact.

**Airline / Hotel Reservation System** – Create a reservation system which books airline seats or hotel rooms. It charges various rates for particular sections of the plane or hotel. Example, first class is going to cost more than coach. Hotel rooms have penthouse suites which cost more. Keep track of when rooms will be available and can be scheduled.

**Student Grade Book Application** – Keep track of students (with a student class that has their name, average, and scores) in a class and their grades. Assign their scores on tests and assignments to the students and figure out their average and grade for the class. For added complexity put the students on a bell curve.

**Bank Account Manager** – Create a class called “Account” which will be an abstract class for three other classes called “CheckingAccount”, “SavingsAccount” and “BusinessAccount”. Manage credits and debits from these accounts through an ATM style program.

**Library Catalog** – Create a book class with a title, page count, ISBN and whether or not it is checked out or not. Manage a collection of various books and allow the user to check out books or return books. For added complexity generate a report of those books overdue and any fees. Also allow users to put books on reserve.

**Patient / Doctor Scheduler** – Create a patient class and a doctor class. Have a doctor that can handle multiple patients and setup a scheduling program where a doctor can only handle 16 patients during an 8 hr work day.

**Recipe Creator and Manager** – Create a recipe class with ingredients and a put them in a recipe manager program that organizes them into categories like deserts, main courses or by ingredients like chicken, beef, soups, pies etc.

**Image Gallery** – Create an image abstract class and then a class that inherits from it for each image type. Put them in a program which displays them in a gallery style format for viewing.

**Class to Handle Large Numbers** – We know that the basic data types like integer, long, double, and floats only go so far. Create a class that can manage extremely large numbers like those used in space exploration.

**Shape Area and Perimeter Classes** – Create an abstract class called “Shape” and then inherit from it other shapes like diamond, rectangle, circle, triangle etc. Then have each class override the area and perimeter functionality to handle each shape type.

**Matrix Class** – A class to manage matrices. Add, subtract and multiple matrices.

**Flower Shop Ordering To Go** – Create a flower shop application which deals in flower objects and use those flower objects in a bouquet object which can then be sold. Keep track of the number of objects and when you may need to order more.

**Vending Machine** – Create an application which takes money and dispenses various types of candy or other item. The user enters a number and letter sequence, like D9, and have it return an instance of “Item” which of the proper type. Example when they press D9 it will return a type of candy bar which is an instance of Mr. GoodBar.

**Josephus Problem** – Create a program which links together various node objects and then every Nth object is removed until you have one object left. This last object is the sole survivor. Look it up on Google under “Josephus Algorithm”

**Family Tree Creator** – Create a class called “Person” which will have a name, when they were born and when (and if) they died. Allow the user to create these Person classes and put them into a family tree structure. Print out the tree to the screen.

Threading
---------

**Create A Progress Bar for Downloads** – Create a progress bar for applications that can keep track of a download in progress. The progress bar will be on a separate thread and will communicate with the main thread using delegates.

**Download Manager** – Allow your program to download various files and each one is downloading in the background on a separate thread. The main thread will keep track of the other thread’s progress and notify the user when downloads are completed.

**Chat Application (remoting style)** – Create a chat application which allows you to connect directly to another computer by their IP through the use of remoting and allow your “server” application handle multiple incoming connections.

Web
---------

**Web Browser with Tabs** – Create a small web browser that allows you to navigate the web and contains tabs which can be used to navigate to multiple web pages at once. For simplicity don’t worry about executing Javascript or other client side code.

**Page Scraper** – Create an application which connects to a site and pulls out all links, or images, and saves them to a list. For added complexity, organize the indexed content and don’t allow duplicates. Have it put the results into an easily searchable index file.

**File Downloader** – An application which can download various objects on a page including video streams or all files on a page. Great for pages with a lot of download links.

**Telnet Application** – Create an application which can telnet into servers across the internet and run basic commands.

**Bandwidth Monitor** – A small utility program that tracks how much data you have uploaded and downloaded from the net during the course of your current online session. See if you can find out what periods of the day you use more and less and generate a report or graph that shows it.

**Bookmark Collector and Sorter** – An application that you can put online for people to upload bookmarks to, have it sort them, remove duplicates and export the entire list as a Firefox/IE/Safari bookmark file. For added complexity see if you can group the bookmark items into various folders.

**Content Management System** – Create a content management system (CMS) like Joomla, Drupal, PHP Nuke etc. Start small and allow for the addition of modules/addons later.

**CAPTCHA Maker** – Ever see those images with letters a numbers when you signup for a service and then asks you to enter what you see? It keeps web bots from automatically signing up and spamming. Try creating one yourself for online forms. If you use PHP, take a look at the image functions of GD.

Files
---------

**Quiz Maker** – Make an application which takes various questions form a file, picked randomly, and puts together a quiz for students. Each quiz can be different and then reads a key to grade the quizzes.

**File Explorer** – Create your own windows explorer program but with added features, better searching, new icons and other views.

**Sort File Records Utility** – Reads a file of records, sorts them, and then writes them back to the file. Allow the user to choose various sort style and sorting based on a particular field.

**Create Zip File Maker** – The user enters various files from different directories and maybe even another computer on the network and the program transfers them and zips them up into a zip file. For added complexity, apply actual compression to the files.

**PDF Generator** – An application which can read in a text file, html file or some other file and generates a PDF file out of it. Great for a web based service where the user uploads the file and the program returns a PDF of the file.

**Bulk Renamer and Organizer** – This program will take a series of files and renames them with a specific filename filter entered by the user. For instance if the user enters myimage###.jpg it will rename all files with a “minimum” of three numbers like “myimage001.jpg”, “myimage145.jpg” or even “myimage1987.jpg” since 1987 has at least three numbers.

**Mp3 Tagger** – Modify and add ID3v1 tags to MP3 files. See if you can also add in the album art into the MP3 file’s header as well as other ID3v2 tags.

**Excel Spreadsheet Exporter** – Create an online application which can read in a file and create an Excel Spreadsheet to export back. This can be through CVS or other file formats. For added complexity, see if you can create formula fields as well.

**File Copy Utility** – Create a utility that can do bulk file copying and backups of other files.

**Code Snippet Manager** – Another utility program that allows coders to put in functions, classes or other tidbits to save for use later. Organized by the type of snippet or language the coder can quickly look up code. For extra practice try adding syntax highlighting based on the language.

**Versioning Manager** – Create your own versioning system for code files. Users are forced to check out items and lock items during reading and writing so that a group of programmers are not accidentally overwriting code files on one another.

Databases
---------

**Remote SQL Tool** – A utility that can execute queries on remote servers from your local computer across the Internet. It should take in a remote host, user name and password, run the query and return the results.

**Database Backup Script Maker** – A program which reads a database’s objects, relationships, records and stored procedures and creates a .sql file which can then be imported into another database or kept as a backup file to rebuild the database with.

**Event Scheduler and Calendar** – Make an application which allows the user to enter a date and time of an event, event notes and then schedule those events on a calendar. The user can then browse the calendar or search the calendar for specific events. For added complexity, allow the application to create reoccurrence events that reoccur every day, week, month, year etc.

**Budget Tracker** – Write an application that keeps track of a household’s budget. The user can add expenses, income, and recurring costs to find out how much they are saving or losing over a period of time. For added complexity allow the user to specify a date range and see the net flow of money in and out of the house budget for that time period.

**Address Book** – Keep track of various contacts, their numbers, emails and little notes about them like a Rolodex in the database. For extra complexity, allow the user to connect to a website publish their address book based on specific options the user has set.

**TV Show Tracker** – Got a favorite show you don’t want to miss? Don’t have a PVR or want to be able to find the show to then PVR it later? Make an application which can search various online TV Guide sites, locate the shows/times/channels and add them to a database application. The database/website then can send you email reminders that a show is about to start and which channel it will be on.

**Database Translation (MySQL  SQL Server)** – A simple utility that reads in from one database and constructs SQL compliant with another database. Then saves that to another database. One popular transition would be to and from MySQL server for databases like SQL Server and Oracle.

Graphics and Multimedia
---------

**Slide Show** – Make an application that shows various pictures in a slide show format. For extra complexity try adding various effects like fade in/out, star wipe and window blinds transitions.

**Mind Mapper** – Allow the user to put down ideas and quickly brainstorm how they are related into a mind map. The goal here is speed so let the user quickly write in an idea and drag it around in a visual map to show relationships.

**Import Picture and Save as Grayscale** – A utility that sucks the color right out of an image and saves it. You could add more including adjusting contrast, colorizing and more for added complexity.

**Stream Video from Online** – Try to create your own online streaming video player.

**Mp3 Player (and Other Formats)** – A simple program for playing your favorite music files. For extra complexity see if you can add in playlists and an equalizer.

**Image Browser** – This application is used to view various image files on your computer from PNG, GIF, JPG to BMP, TIFF etc.

**Traffic Light Application** – See if you can make your own street light application and then put it into an intersection scenario. Don’t let any cars run the lights and crash into one another!

**MP3 to Wav Converter** – MP3 is essentially compressed wav format. See if you can translate it back into wav so that some other sound editing programs can work with the wav file itself. Keep in mind that 1 MB of MP3 is relative 10MB wav.

**Watermarking Application** – Have some pictures you want copyright protected? Add your own logo or text lightly across the background so that no one can simply steal your graphics off your site. Make a program that will add this watermark to the picture.

**Turtle Graphics** – This is a common project where you create a floor of 20 x 20 squares. Using various commands you tell a turtle to draw a line on the floor. You have move forward, left or right, lift or drop pen etc. For added complexity, allow the program to read in the list of commands from a file. Do a search online for “Turtle Graphics” for more information.
