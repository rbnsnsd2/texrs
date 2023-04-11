# textrs
*textrs* is a simple little command line Rust application to do one job, turn a *.csv* file into a *LaTeX* table.

## Example use
```shell
texrs --help
```
This will return the result:
```shell
.csv to LaTeX table converter 0.1.0
Fraign Analytics LLC
A command line tool for the conversion of a .csv file to a LaTeX table with some simple defaults.

USAGE:
    texrs [OPTIONS] <input>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delimiter <delimiter>       [default: ,]
    -h, --header-row <header-row>     [default: 0]
    -r, --max-rows <max-rows>         [default: 20]

ARGS:
    <input>    Input file
```

```shell
texrs /path/to/csv -r 2 # read only the first ten rows
```
This will return a *LaTeX table:
```shell
\begin{table}[ht]
\centering
\caption{CAPTION}
\begin{tabular}{p{0.33333334\textwidth}p{0.33333334\textwidth}}
\toprule
label & narrative \\
\midrule
Procedure & The 757 fleet (and possibly others) has a deferral that allows the aircraft to operate for a specific number of cycles if a passenger cell phone or other electronic device falls down a gap in the cabin sidewall into the cargo pit and cannot be located. Maintenance usually tries to look for the phone; to include disassembling some of the cargo pit panels - but if they can't find it in a 'reasonable' amount of time then a deferral is created and additional flights are authorized.The FAA website says this about devices with lithium batteries: 'Devices containing lithium metal batteries or lithium-ion batteries; including - but not limited to - smartphones; tablets; cameras and laptops; should be kept in carry-on baggage. If these devices are packed in checked baggage; they should be turned completely off; protected from accidental activation and packed so they are protected from damage.'We have provisions for Dangerous Goods being shipped as freight; being used by crew on the airplane; as part of installed equipment; and being carried by passengers or crew on their person; in carry-on bags; and in checked bags. There are no Dangerous Goods provisions providing for lost dangerous goods floating around the airplane somewhere.The concern here is the potential for damage and subsequent fire. We won't even let a passenger move a seat if they drop their phone in the seat - yet somehow; it's okay to drop a phone into the cargo area where it could easily be damaged by the cargo loading system or something else? How is this safe? Or even legal? If a lost device with lithium battery gets crushed and catches on fire; we are not going to be able to fight that fire.I'd like to know specifically what kind of risk analysis (if any) was done for this deferral. \\
Human Factors & Detected an odor onboard the aircraft; similar to burning Lithium Ion battery. After takeoff I noticed there's [smoke] or a cloud around the exit row. After confirming it with a aft FA (Flight Attendant); called the Captain and I told him there's a smoke [area] and I'm going to check it [out]. Put the lights on and I started opening the overhead [bins]; to check if there's anything on fire and also under the seats as well as [asking] the passengers to remove their mask and so they can confirm [if] there's a smell in [area]. One of the passenger sitting in 14A told me he had Lithium Ion batteries in his bag. I grabbed the bag and opened it and I checked the battery however there's smoke coming out of his bag and neither the batteries are hot either. I tried to keep the Captain updated with the same information above. The smell lasted about 3 to 5 minutes and disappeared after that. Then we prepared the cabin for [a precautionary air return]. \\
\bottomrule
\end{tabular}
\label{tab:ref}
\end{table}
```


## Install
After compiling, you can make the app available from the terminal in Linux:
Get your all the directories that will be checked the terminal checks for an application.
```shell
echo $PATH
```
Make sure that you have */usr/local/bin*, if not you may add it with:
```shell
sudo mkdir -p /usr/local/bin
```
Then you have the option of creating a symbolic link of the path to *texrs* or move it into your *bin* directory.
```shell
sudo ln -s /location/of/texrs /usr/local/bin
```
or
```shell
sudo ln -s /location/of/texrs /usr/local/bin
```
