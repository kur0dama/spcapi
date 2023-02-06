import pandas as pd
import random
import math
import datetime as dt
import pathlib
import csv

MIN_DATE = dt.date(2012,1,1)
MAX_DATE = dt.date(2023,12,31)


date_range = [ date.isoformat() for date in pd.date_range(MIN_DATE, MAX_DATE, freq="d") ]

sample_size = []
for i in range(len(date_range)):
    ssize = math.floor(random.gauss(300, 5))
    sample_size.append(ssize if ssize > 0 else 1)

failures = []
for i in range(len(date_range)):
    fail = math.floor(random.gauss(15,3))
    failures.append(fail if fail > 0 else 0)

df_data = pd.DataFrame.from_dict({
    "date": date_range,
    "sample_size": sample_size,
    "failures": failures
})

df_data.to_csv(
    pathlib.Path(__file__).parent.joinpath("data.csv"),
    sep = ",",
    quoting=csv.QUOTE_MINIMAL,
    index=False
)