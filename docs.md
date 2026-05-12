## building the extension

its actully freaking crazy developing due to the lack full proper documentation on how to build a pgrx extension. I have to go through the source code of figure out how to build the extension and how to define GUCs and all that.

And the thing is docs only exists for pgx, a precessor of pgrx, and not for pgrx. 
pgx docs:  https://docs.rs/pgx/latest/pgx/

And there is a this maintainer of pgx/pgrx streaming a few vids about it on twitch and those are 6 year old vids at https://www.twitch.tv/zombodb/videos?filter=all&sort=time

https://docs.rs/pgx/latest/pgx/guc/enum.GucContext.html

Rest I am relying on the source code of pgrx to figure out what and how to do things.

another ref: https://archive.fosdem.org/2025/events/attachments/fosdem-2025-4317-writing-safe-postgresql-extensions-in-rust-a-practical-guide/slides/238202/writing_p_EGMYZay.pdf

