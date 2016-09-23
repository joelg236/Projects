import GeoIP

gi = GeoIP.new(GeoIP.GEOIP_MEMORY_CACHE)

print(gi.country_name_by_addr(raw_input("Enter IP address:\n")))
