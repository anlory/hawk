package com.anlory.hawk;

interface IHawkService {
    void trigger(String tag, String processName, int pid, in String[] args);
    oneway void schedule(String tag, String processName, int pid, in String[] args);
}