set windows-powershell := true

alias n := new

new DAY:
    cargo generate --name day-{{DAY}} day-00
    git add day-{{DAY}}/*