# Changelog

## 1.2.0 - 2024-06-26

### Added
- Margin endpoints
  - `POST /sapi/v1/margin/borrow-repay`
  - `GET /sapi/v1/margin/borrow-repay`

### Updated
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

## 1.1.0 - 2023-12-18

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