stream = open("../inputs/06.txt").read()
packet_start = 0
message_start = 0
for i in range(len(stream) - 14):
    if not packet_start and len(set(stream[i:i+4])) == 4:
        packet_start = i + 4
    if not message_start and len(set(stream[i:i+14])) == 14:
        message_start = i + 14

print(packet_start)
print(message_start)
