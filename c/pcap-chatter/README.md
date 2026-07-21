# PCAP Chatter

PCAP Chatter is an offline conversation summarizer for classic PCAP network
captures. It turns individual Ethernet, IPv4, TCP, and UDP packets into a
compact view of which endpoints communicated and how much traffic passed
between them.

Each conversation summary includes endpoints, protocol, duration, packet and
byte counts, and relevant TCP flag activity. Truncated packets and fragmented
traffic are surfaced rather than silently treated as complete data.

The project is not a live packet sniffer, protocol reassembler, intrusion
detection system, or replacement for Wireshark.
