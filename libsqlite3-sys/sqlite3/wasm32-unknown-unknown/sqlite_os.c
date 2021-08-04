#include <time.h>

#define SQLITE_OK 0

int sqlite3_os_init(void) {return SQLITE_OK;}
// int sqlite3_os_end(void) {return SQLITE_OK;}

struct tm *localtime_r (const time_t *timep, struct tm *tmp)
{
    // TODO: fix this tz conversion
    /* return __tz_convert (t, 1, tp); */
    return tmp;
}
