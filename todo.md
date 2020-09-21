This file describes ideas that can be implemented.

* Manage the time sleep better, maybe just sleep the amount till the next scheduled event to decrease cpu usage even more.
* Add better error handling.
* Maybe add the posibility to also pass integers into the date/time functions, like being able to do `cron.seconds(0)` and `cron.seconds("0,1,2")`
