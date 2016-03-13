initSidebarItems({"fn":[["add_no_expand_char","Add a character to the list of no expand characters."],["add_search_delimiter_char","Add a character to the list of search delimiter characters."],["add_word_delimiter","Add a character from the list of word delimiter characters."],["get_comment_char","Get the current value of the `history_comment_char` variable."],["get_expansion_char","Get the current value of the `history_expansion_char` variable."],["get_inhibit_expansion_function","Get the value of the inhibit expansion function.  This will be None if it is not set."],["get_no_expand_chars","The list of characters which inhibit history expansion if found immediately following `history_expansion_char`. The default is space, tab, newline, carriage return, and `='."],["get_quotes_inhibit_expansion","Get the current value of the `history_quotes_inhibit_expansion` variable."],["get_search_delimiter_chars","Get the list of search delimiter characters. The list of additional characters which can delimit a history search string, in addition to space, TAB, ':' and '?' in the case of a substring search. The default is empty."],["get_subst_char","Get the current value of the `history_subst_char` variable."],["get_word_delimiters","Get the list of word delimiter characters. The characters that separate tokens for `history_tokenize()`. The default value is `\" \\t\\n;&()<>\"`."],["get_write_timestamps","Get the current value of the `history_write_timestamps` variable."],["remove_no_expand_char","Remove a character from the list of no expand characters."],["remove_search_delimiter_char","Remove a character from the list of search delimiter characters."],["remove_word_delimiter","Remove a character from the list of word delimiter characters."],["set_comment_char","During tokenization, if this character is seen as the first character of a word, then it and all subsequent characters up to a newline are ignored, suppressing history expansion for the remainder of the line. This is disabled by default."],["set_expansion_char","The character that introduces a history event. The default is `!'. Setting this to 0 inhibits history expansion."],["set_inhibit_expansion_function","This should be set to a function of the following type: `extern fn(*mut c_char, c_uint) -> c_int`.  The first argument is a string pointer and the second is an int index into that string. It should return a non-zero value if the history expansion starting at string[i] should not be performed; zero if the expansion should be done. It is intended for use by applications like Bash that use the history expansion character for additional purposes. By default, this variable is set to NULL."],["set_no_expand_chars","Set the list of no expand characters.  Note, this will replace any existing list."],["set_quotes_inhibit_expansion","If non-zero, single-quoted words are not scanned for the history expansion character. The default value is 0."],["set_search_delimiter_chars","Set the list of search delimiter characters.  Note, this will replace any existing list."],["set_subst_char","The character that invokes word substitution if found at the start of a line. The default is `^'."],["set_word_delimiters","Set the list of word delimiter characters.  Note, this will replace any existing list."],["set_write_timestamps","If non-zero, timestamps are written to the history file, so they can be preserved between sessions. The default value is 0, meaning that timestamps are not saved."]],"static":[["history_base","The logical offset of the first entry in the history list."],["history_length","The number of entries currently stored in the history list."],["history_max_entries","The maximum number of history entries. This must be changed using `stifle_history()`."]]});