set encoding=utf-8
set fileencodings=utf-8,cp932
set fileencoding=utf-8
set fileformats=unix
 
"dein Scripts-----------------------------
if &compatible
  set nocompatible               " Be iMproved
  endif
 
" Required:
set runtimepath+=~/.vim/dein/repos/github.com/Shougo/dein.vim
 
" Required:
if dein#load_state('~/.vim/dein')
  call dein#begin('~/.vim/dein')
 
  " Let dein manage dein
  " Required:
    call dein#add('~/.vim/dein/repos/github.com/Shougo/dein.vim')
 
  " Add or remove your plugins here:
  call dein#add('Shougo/neosnippet.vim')
  call dein#add('Shougo/neosnippet-snippets')

  " For rust
  call dein#add('rust-lang/rust.vim')
  call dein#add('racer-rust/vim-racer')
 
  " You can specify revision/branch/tag.
  "  call dein#add('Shougo/deol.nvim', { 'rev': 'a1b5108fd' })
 
  " Required:
  call dein#end()
  call dein#save_state()
endif
 
" Required:
filetype plugin indent on
syntax enable
 
" If you want to install not installed plugins on startup.
if dein#check_install()
  call dein#install()
endif
 
"End dein Scripts-------------------------
 
" racer
let g:racer_cmd = "/usr/local/cargo/bin/racer"
au FileType rust nmap gd (rust-def)
au FileType rust nmap gs (rust-def-split)
au FileType rust nmap gx (rust-def-vertical)
au FileType rust nmap gd (rust-doc)
