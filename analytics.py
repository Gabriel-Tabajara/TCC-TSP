import os
import numpy as np

def parse_log_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
        distance = float(lines[3].strip())
        time = float(lines[4].strip())
    return distance, time

def analyze_logs(base_path):
    results = {}

    for state in os.listdir(base_path):
        state_path = os.path.join(base_path, state)
        if not os.path.isdir(state_path):
            continue

        results[state] = {}

        for log_type in ['generic', 'greedy']:
            distances = []
            times = []

            for i in range(1, 11):
                file_name = f"{i}_{log_type}.txt"
                file_path = os.path.join(state_path, file_name)

                if os.path.exists(file_path):
                    distance, time = parse_log_file(file_path)
                    distances.append(distance)
                    times.append(time)

            if distances and times:
                mean_time = np.mean(times)
                mean_distance = np.mean(distances)
                best_result = min(distances)
                worst_result = max(distances)
                time_of_best_distance = times[distances.index(best_result)]

                results[state][log_type] = {
                    'mean_time': mean_time,
                    'mean_distance': mean_distance,
                    'best_result': best_result,
                    'worst_result': worst_result,
                    'time_of_best_distance': time_of_best_distance
                }

    return results

def save_analytics(base_path, analytics):
    for state, log_types in analytics.items():
        state_path = os.path.join(base_path, state)
        output_file = os.path.join(state_path, 'analytics.txt')

        with open(output_file, 'w') as file:
            for log_type, data in log_types.items():
                file.write(f"Log Type: {log_type}\n")
                file.write(f"Mean Time: {data['mean_time']:.2f}\n")
                file.write(f"Mean Distance: {data['mean_distance']:.2f}\n")
                file.write(f"Best Result: {data['best_result']:.2f}\n")
                file.write(f"Worst Result: {data['worst_result']:.2f}\n")
                file.write(f"Time of Best Distance: {data['time_of_best_distance']:.2f}\n\n")
                print(f"State: {state}")
                print(f"  Log Type: {log_type}")
                print(f"    Mean Time: {data['mean_time']:.2f}")
                print(f"    Mean Distance: {data['mean_distance']:.2f}")
                print(f"    Best Result: {data['best_result']:.2f}")
                print(f"    Worst Result: {data['worst_result']:.2f}")
                print(f"    Time of Best Distance: {data['time_of_best_distance']:.2f}")

def main():
    base_path = 'src/assets/outputs/SA'
    analytics = analyze_logs(base_path)
    save_analytics(base_path, analytics)

if __name__ == "__main__":
    main()