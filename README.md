# 🗳️ Solana Voting App

![Solana](https://img.shields.io/badge/Solana-Web3-4caf50?style=flat-square&logo=solana)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=flat-square&logo=rust)

Solana Voting App adalah smart contract sederhana berbasis **Solana** yang memungkinkan pengguna untuk membuat dan memberikan suara dalam polling berbasis blockchain.

---

## 🚀 Fitur
✅ **Membuat polling baru**  
✅ **Memberikan suara (yes/no) dalam polling**  
✅ **Menyimpan hasil voting di blockchain Solana**  
✅ **Dibangun dengan Rust & Anchor**  

---

## 🛠️ Instalasi

### 1️⃣ Persiapan Awal
Pastikan Anda sudah menginstal **Rust**, **Solana CLI**, dan **Anchor**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### 2️⃣ Clone Repository
```bash
git clone https://github.com/username/solana-voting.git
cd solana-voting
```

### 3️⃣ Buat Wallet & Airdrop SOL
```bash
solana-keygen new --outfile ~/.config/solana/id.json
solana config set --url devnet
solana airdrop 2
```

### 4️⃣ Build dan Deploy Smart Contract
```bash
anchor build
anchor deploy
```

### 5️⃣ Jalankan Test
```bash
anchor test
```

---

## 📌 Cara Penggunaan

### Membuat Polling Baru
```bash
solana-voting create_poll --title "Haruskah kita mengadopsi Rust?"
```

### Memberikan Suara
```bash
solana-voting vote --poll_id 1 --vote_yes true
```

### Melihat Hasil Polling
```bash
solana-voting get_results --poll_id 1
```

---

## 🛠️ Struktur Proyek
```
solana-voting/
├── app/                 # Aplikasi frontend (opsional)
├── programs/            # Smart contract (program Solana)
│   └── solana-voting/   # Source code utama
├── tests/               # Pengujian dengan Mocha & Anchor
├── migrations/          # Skrip migrasi (opsional)
├── Anchor.toml          # Konfigurasi Anchor
├── Cargo.toml           # Dependencies Rust
├── package.json         # Dependencies Node.js
└── README.md            # Dokumentasi proyek
```

---

## 🌟 Kontribusi
Pull request sangat diterima! Jika ingin berkontribusi, silakan fork repo ini dan ajukan PR.
