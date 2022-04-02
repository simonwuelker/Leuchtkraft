" Vim syntax file
" Language: Leuchtkraft
" Maintainer: Alaska
" Latest Revision: 19 March 2022

if exists("b:current_syntax")
  finish
endif

syn keyword leuchtkraftKeyword and forall
syn keyword leuchtkraftKeyword "=>"
syn keyword leuchtkraftBoolean true false
syn keyword leuchtkraftQuestion "?"
syn region  leuchtkraftCommentLine start="//" end="\n"
syn match   leuchtkraftFunction "\(\(\a\|[.][._\a]\)[._\w]*\)\+\ *[(]\@="

let b:current_syntax = "leuchtkraft"

hi def link leuchtkraftKeyword       Keyword 
hi def link leuchtkraftBoolean       Boolean
hi def link leuchtkraftCommentLine   Comment
hi def link leuchtkraftQuestion      Comment
hi def link leuchtkraftFunction      Function
