#include "sokoban.h"
#include <stdio.h>
#include <stdint.h>

#define UNWRAP_GET(function_call)                   \
    do                                              \
    {                                               \
        if ((function_call) != 0)                   \
        {                                           \
            printf("FAILED: %s\n", #function_call); \
            return -1;                              \
        }                                           \
    } while (0)

#define UNWRAP_INSERT(function_call)                 \
    do                                               \
    {                                                \
        uint32_t result = function_call;             \
        if ((result == 0) || (result == UINT32_MAX)) \
        {                                            \
            printf("FAILED: %s\n", #function_call);  \
        }                                            \
    } while (0)

int main()
{
    // Mock solana account. This will have u64 alignment
    u_int64_t *account_bytes = malloc(RedBlackTreeu64u64128SIZE);
    printf("allocated %lu\n bytes", RedBlackTreeu64u64128SIZE);

    RedBlackTreeu64u64128 *tree = (RedBlackTreeu64u64128 *)(account_bytes);
    initialize(tree);
    printf("init\n");

    UNWRAP_INSERT(c_insert(tree, (uint64_t)(5), (uint64_t)(3)));
    printf("inserted 5 and 3\n");

    UNWRAP_INSERT(c_insert(tree, (uint64_t)(6), (uint64_t)(4)));
    printf("inserted 6 and 4\n");

    uint64_t key = 6;
    uint64_t value = 0;
    UNWRAP_GET(c_get(tree, &key, &value));
    printf("get %llu -> %llu\n", key, value);


    // Mock solana account. This will have u64 alignment
    u_int64_t *table_bytes = malloc(HashTableu64u64128128SIZE);
    printf("allocated %lu\n bytes", HashTableu64u64128128SIZE);

    HashTableu64u64128128 *table = (HashTableu64u64128128 *)(table_bytes);
    initialize_table(table);
    printf("init\n");

    UNWRAP_INSERT(table_insert(table, (uint64_t)(5), (uint64_t)(3)));
    printf("inserted 5 and 3\n");

    UNWRAP_INSERT(table_insert(table, (uint64_t)(6), (uint64_t)(4)));
    printf("inserted 6 and 4\n");

    uint64_t key_table = 6;
    uint64_t value_table = 0;
    UNWRAP_GET(table_get(table, &key_table, &value_table));
    printf("get %lu -> %llu\n", key_table, value_table);

    UNWRAP_GET(table_get(table, &key_table, &value_table));
    printf("get %lu -> %llu\n", key_table, value_table);

    UNWRAP_GET(table_remove(table, &key_table, &value_table));
    printf("remove %lu -> %llu\n", key_table, value_table);

    UNWRAP_GET(table_get(table, &key_table, &value_table));
    printf("get %lu -> %llu\n", key_table, value_table);
}
