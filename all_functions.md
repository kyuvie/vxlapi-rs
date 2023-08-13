# 関数全一覧
[ ] RxCanFdThread
[ ] RxThread
[ ] TxThread (バースト送信のときのみ使用)
[ ] demoHelp
[ ] demoPrintConfig
[ ] demoCreateRxThread
[ ] demoTransmit
[ ] demoStopTransmitBurst
[ ] demoTransmitBurst
[ ] demoTranmitRemote
[ ] demoStartStop
[ ] demoSetOutput
[ ] demoCreateRxThread
[ ] demoInitDriver
[ ] demoCleanUp
[ ] main

# コールツリー
[ ] main
  [x] demoInitDriver
  [x] demoCreateRxThread
  [x] xlActivateChannel
  [x] demoTransmit
  [ ] demoStopTransmitBurst - (バーストは後回し)
  [ ] demoTransmitBurst - (バーストは後回し)
  [ ] demoTransmitRemote - (リモートフレームはいらないのでいらない)
  [ ] demoSetOutput - (出力モードの切り替えらしい、ただ動かす分にはいらない)
  [x] demoStartStop
  [ ] demoPrintConfig - (ドキュメントがあるからいらない)
  [ ] demoHelp - (ドキュメントがあるからいらない)