#include "libstock.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    PriceDataField_t asks = {
        (char*)"100.0",
        (char*)"30.0"
    };
    PriceDataField_t bids = {
        (char*)"110.0",
        (char*)"30.0"
    };
    uint8_t buf[4096] = {0};
    size_t written_size = 0;

    slice_mut_uint8_t slice = { buf, 4096 };

    BboStructure_t bbo_structure = {
        114514,
        1919810,
        EXCHANGE_BINANCE,
        MARKET_TYPE_EUROPEAN_OPTION,
        MESSAGE_TYPE_B_B_O,
        (char*) "BTC/USDT",
        asks,
        bids,
    };

    bool return_value = serialize_bbo(&bbo_structure, slice, &written_size);
    printf("Return Value: %d; written_size: %zu\n", return_value, written_size);

    slice_ref_uint8_t slice_ref = { buf, 4096 };

    BboStructure_t* bbo = deserialize_bbo(&slice_ref);
    printf("A_QB = %s, A_QQ = %s, B_QB = %s, B_QQ = %s, TS = %lld\n", bbo->asks.quantity_base, bbo->asks.price, bbo->bids.quantity_base, bbo->bids.price, bbo->exchange_timestamp);

    free_bbo(bbo);
    return 0;
}
