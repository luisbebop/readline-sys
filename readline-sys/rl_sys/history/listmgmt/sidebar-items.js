initSidebarItems({"fn":[["add","Place string at the end of the history list. The associated data field (if any) is set to NULL."],["add_time","Change the time stamp associated with the most recent history entry to the given time.  Note that if the `history_comment_char` variable has not been set this will have no effect.  This is stored as seconds since the epoch, so you may lose some precision."],["clear","Clear the history list by deleting all the entries."],["free_entry","Free the history entry and any history library private data associated with it. If there is application-specific data, an Err is returned with a pointer to the data so the caller can dispose of it"],["is_stifled","Returns true if the history is stifled, false if it is not."],["remove","Remove history entry at the given offset from the history. The removed element is returned so you can free the line, data, and containing structure."],["replace_entry","Replace the history entry at offset with the given line and data. This returns the old entry so the caller can dispose of any application-specific data. In the case of an invalid offset, an Err is returned."],["stifle","Stifle the history list, remembering only the last *max* entries."],["unstifle","Stop stifling the history. This returns the previously-set maximum number of history entries (as set by `stifle_history()`)."]]});