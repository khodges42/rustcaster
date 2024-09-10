import os
import re
import random

def generate_random_map(width, height):
    """Generate a random map with solid borders and fewer walls."""
    # Initialize the map with solid borders
    map_data = [['#' for _ in range(width)] for _ in range(height)]
    
    # Fill the interior with fewer walls and empty spaces
    for row in range(1, height - 1):
        for col in range(1, width - 1):
            # Randomly decide to place a wall or leave empty space with fewer walls
            map_data[row][col] = random.choices([' ', '#'], weights=[75, 25])[0]
    
    # Convert each row list to a string
    map_data = [''.join(row) for row in map_data]
    return map_data

def save_map_to_file(map_data, file_path):
    """Save the map data to a text file."""
    with open(file_path, 'w') as file:
        for line in map_data:
            file.write(line + '\n')

def get_latest_file_number(directory, prefix='map', extension='.txt'):
    """Find the latest file number in the directory."""
    latest_number = 0
    pattern = re.compile(rf"{prefix}(\d+){extension}")
    for filename in os.listdir(directory):
        match = pattern.match(filename)
        if match:
            number = int(match.group(1))
            if number > latest_number:
                latest_number = number
    return latest_number + 1

def main():
    # Random dimensions between 5 and 50
    width = random.randint(5, 50)
    height = random.randint(5, 50)
    
    directory = '.'  # Change this to the directory where you want to save the file
    
    map_data = generate_random_map(width, height)
    next_number = get_latest_file_number(directory)
    file_name = f"map{next_number}.txt"
    file_path = os.path.join(directory, file_name)
    save_map_to_file(map_data, file_path)
    print(f"Map saved to {file_path}")

if __name__ == "__main__":
    main()
