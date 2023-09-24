#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return
LANG=en_IN.UTF-8

alias ls='ls --color=auto'
alias grep='grep --color=auto'
alias a='sh ~/.auto.sh > /dev/null 2>&1'
alias r='rofi -show drun'
alias oby='cd ~/Downloads && ./Obsidian-1.3.7.AppImage && cd'
alias mkjv='cp -r ~/.default/defaultjava'
alias mkc='cp -r ~/.default/defaultc'
alias mkcp='cp -r ~/.default/defaultcpp'
alias x='exit'

PS1='[\u@\h \W]\$ '
source ~/.bash_completion/alacritty
sh ~/.auto.sh > /dev/null 2>&1

#eval "$(starship init bash)"
#xrandr --output Virtual-1 --mode 1920x1080 > /dev/null 2>&1 & 
#feh --bg-fill ~/Downloads/Wallpaper/wallpaperflare.com_wallpaper.jpg &

#picom -b > /dev/null 2>&1
