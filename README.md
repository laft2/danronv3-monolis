# danronv3-monolis
ダンガンロンパV3のミニゲームであるモノリスのソルバなど。

## monolis-fetch
OpenCVでゲームのスクリーンショットから11*22のマス情報のCSVを生成する。

## monolis-solver
焼きなましで11*22のマス情報から最多マスの破壊を目指すプログラム。

## visualizer
HTML+JSのビジュアライザ。1つ目のテキストボックスにマス情報のCSV、2つ目のテキストボックスに操作`(y, x)`を改行区切りで入れる。
上記のmonolis-solverで生成されたものを利用することを想定。

これらを入力し、startを押すと、次の入力場所が視覚的に表示される。
nextで次の手の入力場所の表示、prevで前の手に戻れる。

Aboutのリンクにvisualizerをデプロイしてある。

![image](https://github.com/laft2/danronv3-monolis/assets/44256206/c86084c3-bb15-44ed-bf80-7eb073d24874)
