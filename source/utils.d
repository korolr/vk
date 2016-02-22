module utils;

import std.stdio, std.array, std.file, core.thread;

const bool debugMessagesEnabled = false;
const bool dbmfe = true;

__gshared string dbmlog = "";

private void appendDbg(string app) {
    thread_enterCriticalRegion();
    dbmlog ~= app;
    thread_exitCriticalRegion();
}

void dbmclose() {
    if(!dbmfe) return;
    auto ff = File("dbg", "w");
    ff.write(dbmlog);
    ff.close();
}

void dbm(string msg) {
    if(debugMessagesEnabled) writeln("[debug]" ~ msg);
    if(dbmfe) appendDbg(msg ~ "\n");
}

string longpollReplaces(string inp) {
    return inp
        .replace("<br>", "\n")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&");
}