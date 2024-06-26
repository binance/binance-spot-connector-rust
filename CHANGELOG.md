# Changelog

## 1.2.0 - 2024-06-26

### Added
- Margin endpoints
  - `POST /sapi/v1/margin/borrow-repay`
  - `GET /sapi/v1/margin/borrow-repay`
- Implemented subscription from slice of Streams
- Added features `futures-util` and `tokio` to `enable-tokio-tungstenite`
- Enabled `binance_tokio_client` to be used within `async` context and `Tokio::Spawn` runtime

### Updated
- Upgraded following dependencies:
  - `strum` to version `0.26.2`
  - `base64` to version `0.22.1`
  - `tungstenite` to version `0.23.0`
  - `env_logger` to version `0.11.3`
  - `cargo-audit` to version `0.20.0`
- Margin endpoints
  - `GET /sapi/v1/margin/transfer`: add parameter `isolatedSymbol`
  - `GET /sapi/v1/margin/allAssets`: add parameter `asset`
  - `GET /sapi/v1/margin/allPairs`: add parameter `symbol`
  - `GET /sapi/v1/margin/isolated/allPairs`: add parameter `symbol`


### Deleted
- Margin endpoints
  - `POST /sapi/v1/margin/transfer`
  - `POST /sapi/v1/margin/isolated/transfer`
  - `POST /sapi/v1/margin/loan`
  - `POST /sapi/v1/margin/repay`
  - `GET /sapi/v1/margin/isolated/transfer`
  - `GET /sapi/v1/margin/asset`
  - `GET /sapi/v1/margin/pair`
  - `GET /sapi/v1/margin/isolated/pair`
  - `GET /sapi/v1/margin/loan`
  - `GET /sapi/v1/margin/repay`
  - `GET /sapi/v1/margin/dribblet`
  - `GET /sapi/v1/margin/dust`
  - `POST /sapi/v1/margin/dust`

## 1.1.0 - 2023-12-21

### Updated
- Remove `rsa` due to Marvin Attack vulnerability
- Add `ed25519` signature
- Upgraded `signature` to version `2.2.0`

## 1.0.2 - 2023-09-28

### Updated
- Upgraded `tungstenite` to version 0.20.1 

## 1.0.1 - 2023-06-28

### Added
- Added User-Agent header

## 1.0.0 - 2022-12-06

### Added
- First release of the Binance Spot API connector.