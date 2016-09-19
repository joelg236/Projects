import ntplib,datetime
print datetime.datetime.utcfromtimestamp(ntplib.NTPClient().request('north-america.pool.ntp.org').tx_time)
