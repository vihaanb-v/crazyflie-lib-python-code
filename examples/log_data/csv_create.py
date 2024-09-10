import csv

filename = "static_drone_drift.csv"

fields = ['Initial_x_position', "Initial_y_position", "Initial_z_position", "Hover_x_position", "Hover_y_position", "Hover_z_position", 'Final_x_position', "Final_y_position", "Final_z_position", "Total_x_displacement", "Total_y_displacement", "Total_z_displacement", "Calculated_overall_drift"]
rows = []

with open(filename, 'r+w') as csvfile:
    csvreader = csv.reader(csvfile)