#include <time.h>

struct tm *localtime_r (const time_t *timep, struct tm *tmp)
{
    // TODO: fix this tz conversion
    /* return __tz_convert (t, 1, tp); */
    return tmp;
}
