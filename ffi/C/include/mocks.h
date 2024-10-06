#ifndef RSM_MOCKS
#include <string.h>
#include <unistd.h>                                                             // database access

struct getgroups_mocks{
    gid_t *group_id_set;
    int group_id_length;

} typedef GetGroupsMocks;

int getgroups_wrapper(int buffer_len, gid_t *buffer, GetGroupsMocks * mocks){
    if (mocks){
        if (buffer_len < mocks-> group_id_length){
            return -1;
        }else{
            memcpy(buffer, mocks->group_id_set, mocks->group_id_length);
            return mocks->group_id_length;
        }

    }else{
        return getgroups(buffer_len, buffer);
    }
}

#endif

