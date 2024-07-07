_uu_cut() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="uu_cut"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        uu_cut)
            opts="-A -b -e -E -n -s -t -T -v -u -h -V --show-all --number-nonblank --show-ends --number --squeeze-blank --show-tabs --show-nonprinting --help --version [file]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _uu_cut -o nosort -o bashdefault -o default uu_cut
