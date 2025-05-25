|%
++  jet-batch-interpolate
  |=  [constraints challenges]
  ^-  (unit (list polynomial))
  :: Jet 绑定，返回 ~ 说明未调用 Jet
  ~
++  jet-cached-fiat-shamir
  |=  [header nonce constraints]
  ^-  (unit (list challenge))
  ~
++  jet-batch-fri-verify
  |=  [polynomials merkles challenges]
  ^-  ?
  ~
++  jet-batch-verify-merkles
  |=  merkles
  ^-  ?
  ~
--