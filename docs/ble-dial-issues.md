# BLE接続とDialの検証（Issues #3–#5）

## ホスト切替 (#3)

登録済みプロファイルでは保存された相手のidentity address宛の低デューティdirected advertisingを使います。
未登録プロファイルはHID広告を出し、新規ペアリングできます。ドングルの探索動作は維持します。
接続後のidentity照合も残します。300秒の広告タイムアウト・キー操作による再開は従来どおりです。
保存済みIRKのコントローラーへの同期はtrouble-hostが行います。

Mac/Windows両方のBLEを有効にしてプロファイルを往復し、以前のホストの割り込み切断が消えること、
RPAを使用するホストにも再接続できることを実機確認してください。
ホスト側だけでペアリングを削除した場合はキーボード側の対象プロファイルもクリアして再登録します。
自動的に無差別広告へ戻すと元の競合を再発させるため、登録済みスロットでは行いません。

## 接続中の送信出力 (#4)

Madulaのkeyboard.tomlにdefault_tx_power = 8を指定します。広告と同じ+8 dBmです。
この設定を共有するCornix左右にも適用されます。cornix-tp.tomlは変更しません。
電波到達性の改善候補で、遅延改善・電池寿命は未測定です。

## Windows Dial (#5): 実験段階

通常版の1クリック18°は維持します。dial-smooth機能を有効にした検証版のみ、
1クリックを6°×3回、送信キューへの投入間隔8msに分けます。BLE上の実到着間隔は接続間隔にも依存します。
ボタン状態・符号・角度合計を維持します。回転処理中の追加イベント処理は最大約16ms＋キュー待ち遅延します。
急速な連続回転の滞留、ボタンの応答も確認が必要です。通常配布版では有効にしません。

比較項目: USB/BLEそれぞれで、1秒おきの左右1クリック、連続回転、押し回し。
Windowsのメニュー・スクロール・音量を個別に確認します。
分割送信で改善するという根拠はIssue内の試作報告で、今回の18°版の実機改善は未確認です。
Microsoft仕様上は相対角度を報告し、アプリ側で回転イベントの解像度を設定します。
https://learn.microsoft.com/en-us/windows-hardware/design/component-guidelines/radial-controller-input-reports
https://learn.microsoft.com/en-us/uwp/api/windows.ui.input.radialcontroller.rotationresolutionindegrees

検証用ビルド例:
cargo +1.94.1 build --release --locked --bin madula-central --features pmw3610,battery-audit,madula-vial,dial-smooth
