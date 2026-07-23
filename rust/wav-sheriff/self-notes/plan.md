## Steps
x Args parsing
x text files output
x Pass files as arguments 
x Make sure audio files can be passed

### Audio files:
x Understand samples and sample rate, if they can be used in some way
x Able to plot audio files
  x research on representing it in experimentable format
x takes any value (int / float) using generic types

---
Conceptually:
```bash
$ wav-sheriff recording.wav

recording.wav
Format: PCM, 24-bit, 48,000 Hz, stereo
Duration: 02:14.380

00:18.420–00:18.487  channel 1  clipping
  37 samples exceeded 99.5% of full scale

01:03.100–01:03.640  channel 2  dropout
  signal remained below -72 dBFS for 540 ms
```

### formatting (phase 1)
- file name
- Format: `{PCM}{bits}{sample rate}{stereo or not}`
- Duration: time
