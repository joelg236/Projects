import pywhois, os

domain = raw_input('Enter domain name: ')
os.system('whois ' + domain)
