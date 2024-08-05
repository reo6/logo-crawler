"""
Simple script to select random URLs from file.
"""

import csv
import random
import sys

def pick_random_urls(input_file, output_file, num_urls):
    with open(input_file, 'r') as file:
        urls = [line.strip() for line in file if line.strip()]

    if num_urls > len(urls):
        print("Requested amount of samples is more than what is in the file!!")
        return

    selected_urls = random.sample(urls, num_urls)

    with open(output_file, 'w', newline='') as file:
        writer = csv.writer(file)
        for url in selected_urls:
            writer.writerow([url])

if len(sys.argv) != 2:
    print("Usage: python csv.py <number_of_urls>")
    sys.exit(1)

try:
    num_urls = int(sys.argv[1])
except ValueError:
    print("Error: Please provide a valid number.")
    sys.exit(1)

pick_random_urls('websites.csv', f'sample-csv/{num_urls}.csv', num_urls)
