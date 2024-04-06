#!/usr/bin/env python3
import csv
import sys

def main(tsvfilename):
    with open(tsvfilename, 'w', newline='') as tsvfile:
        writer = csv.writer(tsvfile, delimiter='\t', lineterminator='\n')
        # alice
        writer.writerow(["13", "Alice In Windowland"])
        # bob
        writer.writerow(["Cat"])
        writer.writerow(["Dog"])
        writer.writerow(["Fox", "-9"])
        writer.writerow(["Mouse", "10000", "00000000"])
        writer.writerow(["Mouse", "20000", "05050505"])
        writer.writerow(["Mouse", "30000", "30303030"])


if __name__ == '__main__':
    main(sys.argv[1])
