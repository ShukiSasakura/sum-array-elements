#!/usr/bin/env python3

from matplotlib import pyplot as plt
from matplotlib import ticker
import numpy as np
import pandas as pd
import datetime
import sys
import io

# 日本語フォントの設定
from matplotlib import rcParams
rcParams['font.family'] = 'sans-serif'
rcParams['font.sans-serif'] = ['Hiragino Maru Gothic Pro', 'Yu Gothic', 'Meirio', 'Takao', 'IPAexGothic', 'IPAPGothic', 'VL PGothic', 'Noto Sans CJK JP']

# 測定結果の tsv を貼り付ける
wasm_tsv = """
n	c1000
1	22.730469329
2	11.537748444
3	7.930561409
4	6.267954291
5	5.251168162
6	4.592078549
7	5.23375117
8	5.33204607
9	5.169251021
10	5.040886722
11	4.698801555
12	4.416168114
13	4.499491591
14	4.663051695
15	4.842656896
16	4.898062557
17	4.861768866
18	4.97273531
19	4.834906923
20	4.861972408
21	5.066953152
22	5.038917297
23	5.0374193
24	5.123189984
25	4.890472388
26	5.009022041
27	5.022813707
28	5.059595787
29	4.987766599
30	4.913732822
"""
#100	12.820391465	2.344267474	1.492514893	1.359398606
#200	13.229663994	2.415132974	1.512656666	1.39291139

native_tsv = wasm_tsv
"""
n	c1	c10	c100	c1000
1	43.864970951	12.851638944	6.133606388	5.872110573
2	22.228611295	6.538606162	3.108659936	3.296974284
4	11.235006843	3.337045422	1.547067284	1.506320416
10	4.576695132	1.408209489	0.690784143	0.677608179
20	3.148371624	1.059178379	0.549682045	0.589715055
"""
#100	3.227985559	1.128641384	0.562382668	0.56550581
#200	3.290074474	1.113743197	0.567589226	0.565830342

# 貼り付けた tsv をファイルとして認識させる
wasm_tsv = io.StringIO(wasm_tsv)
native_tsv = io.StringIO(native_tsv)

# tsv をデータフレームとして読み込む
wasm_df = pd.read_table(wasm_tsv)
native_df = pd.read_table(native_tsv)

# tsv から特定のカラムを取り出す場合の例
# https://numpy.org/doc/stable/reference/generated/numpy.loadtxt.html
# https://it-ojisan.tokyo/numpy-tsv/
# [[10 21 32 43]
#  [14 25 36 47]
#  [18 29 30 41]]
# 1行目と3行目を読み込む
# [[21 43]
#  [25 47]
#  [29 41]]
# wasm_array = np.loadtxt(wasm_tsv, delimiter = "\t", dtype = float, usecols = (1, 3))
# native_array = np.loadtxt(native_tsv, delimiter = "\t", dtype = float, usecols = (1, 3))


# 各軸の値をセットする
##  縦軸を基準値の比率とする場合
### 表示する比率の基準となる値を取り出す
wasm_n1 = wasm_df[wasm_df['n'] == 1]['c1000'][0]
native_n1 = native_df[native_df['n'] == 1]['c1000'][0]

### 基準値を各データで割り，基準値に比べて小さい場合に大きな値としている
x = wasm_df['n']
y1 = wasm_n1/wasm_df['c1000']
y2 = native_n1/native_df['c1000']

## 縦軸を時間とする場合
#x = wasm_df['n']
#y1 = wasm_df['c1']
#y2 = native_df['c1']

# y1 軸と y2 軸を重ねるために 2つの ax を作成して重ねる
fig, ax1 = plt.subplots()
ax2 = ax1.twinx()

# ax1 のほうに X軸とラベルを書く
## 時間の場合
## ax1.xaxis.set_major_formatter(md.DateFormatter('%H:%M'))
## ax1.xaxis.set_major_locator(md.HourLocator(byhour=range(0, 24, 3), tz=None))
## date = datetime.strptime(target_date, '%Y%m%d')
## label = date.strftime('時刻 (%Y年%m月%d日 {}曜日)').format('月火水木金土日'[date.weekday()])

## ラベルと数値を単純に並べる場合
label = "スレッド数"
ax1.set_xlabel(label)

# 折れ線グラフのプロット
# ax1 のほうに結果をプロット Y軸 (左) にラベルを書く
ax1.plot(x, y1, c='b', label='wasm', ls='-', lw=1)
ax1.plot(x, y2, c='g', label='native', ls='--', lw=1)
ax1.set_ylabel("time")

# ax2 のほうに気温をプロット Y軸 (右) にラベルを書く
# ax2.plot(x, y2, c='g', label='native', ls='--', lw=1)
# ax2.set_ylabel("時間 (s)")

# 棒グラフのプロット
# ax1.bar(x, y1, c='b', label='sample')

# ax1 と ax2 の凡例をつなげて，ax1 側に書く
hdr1, leg1 = ax1.get_legend_handles_labels()
hdr2, leg2 = ax2.get_legend_handles_labels()
ax1.legend(hdr1 + hdr2, leg1 + leg2, loc='upper left')

# グラフの pdf 出力
date = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
filename = "plot"
## ファイル名を日付にする場合
## filename = f'{date}'
plt.savefig(filename + '.pdf')

sys.exit()
