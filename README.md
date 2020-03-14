# rivm-covid-19
Small program to get targeted updates from the RIVM covid 19 csv reports.

# Compatability
This program was written in rust. And uses the Linux environment variables.
It was written in Manjaro linux, but should work on any Linux distro with rust
installed.

# Installation
First, [install rust](https://www.rust-lang.org/tools/install).

The program will assume that there is a folder called /home/{{user}}/Data.
We need to create this folder. The code below will do that, and download the
code to /home/{{user}}/git/rivm-covid-19. This latter location can be changed.

```bash
cd ~
mkdir Data
mkdir git
cd git
git clone https://github.com/dwrolvink/rivm-covid-19.git
cd rivm-covid-19
cargo build --release
cp target/release/rivm-updates ~/.local/bin 
```

# Deinstallation
```bash
rm ~/.local/bin/rivm-updates
rm -rf ~/git/rivm-covid-19
```

# Usage
The program will look for three files in the Data directory. The name is dependent on the current date. If the current date is March 14th, it will look for these files:
- The new data export: Mar-14-total.csv
- The previous data export: Mar-13-total.csv
- The tracking file: Mar-13-tracked.csv

So the date format is "Mmm-DD".

The last two files need to be created in the Data folder. (`with the date of yesterday`).   
These should be empty. You'll only have to do this step once.

Then, download the latest csv from the RIVM at [https://www.rivm.nl/coronavirus-kaart-van-nederland](https://www.rivm.nl/coronavirus-kaart-van-nederland). Click on the little down-arrow in the bottom right corner, and select .csv. Save it to the Data directory with the name listed above (`with your current date`).

Finally, run the following to execute:
```bash
rivm-updates
```

This will give you a summary of all changes for the tracked cities.  For every city that is new 
on the most recent csv (that isn't on the csv from a day before), you'll get the question if you want
to add it to your tracking list.

After completion, a new file will have been created, called "Mmm-DD-tracked.csv".

If you want to stop tracking a city, open the most recent tracking file, and manually remove the city
from that list.

