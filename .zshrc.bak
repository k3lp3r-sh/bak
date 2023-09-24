# Lines configured by zsh-newuser-install
HISTFILE=~/.histfile
HISTSIZE=8000
SAVEHIST=1000
setopt autocd extendedglob
bindkey -e
# End of lines configured by zsh-newuser-install
# The following lines were added by compinstall
zstyle :compinstall filename '/home/kelper/.zshrc'

alias ls='ls --color=auto'
alias grep='grep --color=auto'
alias r='rofi -show drun'
alias oby='cd ~/Downloads && ./Obsidian-1.3.7.AppImage && cd'
alias mkjv='cp -r ~/.default/defaultjava'
alias mkc='cp -r ~/.default/defaultc'
alias mkcp='cp -r ~/.default/defaultcpp'

# Designing the prompt
# Loading version control information
autoload -Uz vcs_info
precmd() { vcs_info }
zstyle ':vcs_info:git:*' formats 'on %F{#ff007f}(%b)%f'
setopt PROMPT_SUBST # Enable prompt substitution
PROMPT='%B%F{cyan}%~/%f%b %B${vcs_info_msg_0_}%b%B%F{cyan}%f%b'

# Define a function to delete a whole word with Ctrl+Backspace
backward-delete-word() {
    local WORDCHARS="${WORDCHARS:s/\\/}"
    zle backward-kill-word
}

# Bind Ctrl+Backspace to the defined function
zle -N backward-delete-word
bindkey '^H' backward-delete-word

autoload -Uz compinit
compinit
# End of lines added by compinstall
