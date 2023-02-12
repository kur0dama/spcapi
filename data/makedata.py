import pandas as pd
import random
import math
from decimal import Decimal
import datetime as dt
import pathlib
import json
from typing import Union

MIN_DATE = dt.date(2012,1,1)
MAX_DATE = dt.date(2023,12,31)
NUM_PREC_FMT = "{:.8f}"

class SpcType:
    XBAR = "xbar"
    P = "p"
    C = "c"
    U = "u"
    MR = "mr"
    I = "i"
    G = "g"
    T = "t"

class DateFreq:
    DAY = "day"
    WEEK = "week"
    MONTH = "month"
    QUARTER = "quarter"
    YEAR = "year"
    FISCAL_YEAR = "fiscal_year"

def decimal_to_formatted_str(i: int) -> str:
    di = Decimal(i)
    return NUM_PREC_FMT.format(di)

date_range = [ date.isoformat() for date in pd.date_range(MIN_DATE, MAX_DATE, freq="d") ]

sample_size = []
for i in range(len(date_range)):
    ssize = math.floor(random.gauss(300, 5))
    sample_size.append(
        decimal_to_formatted_str(
            ssize if ssize > 0 else 1
        )
    )

failures = []
for i in range(len(date_range)):
    fail = math.floor(random.gauss(15,3))
    failures.append(
        decimal_to_formatted_str(
            fail if fail > 0 else 0
        )
    )

df_data = pd.DataFrame.from_dict({
    "dt": date_range,
    "w": sample_size,
    "n": failures
})

ddata = list(df_data.to_dict(orient="index").values())

doc = {
    "spc_type": SpcType.XBAR,
    "target_date_freq": DateFreq.DAY,
    "data": ddata,
}

with open(pathlib.Path(__file__).parent.joinpath("data.json"), "w") as f:
    f.write(
        json.dumps(doc, indent = 4)
    )