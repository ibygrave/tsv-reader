#!/usr/bin/env python3
import csv
import sys

def main(tsvfilename):
    with open(tsvfilename, 'w', newline='') as tsvfile:
        writer = csv.writer(tsvfile, delimiter='\t', lineterminator='\n')
        # header
        writer.writerow(["1", "Example Title", "FFFFFF"])
        # shapes
        writer.writerow(["000000", "false", "Line", "0", "0", "500", "500"])
        writer.writerow(["550055", "true", "Circle", "200", "300", "20"])
        writer.writerow(["FF0055", "false", "Rectangle", "100", "100", "200", "200"])


if __name__ == '__main__':
    main(sys.argv[1])
