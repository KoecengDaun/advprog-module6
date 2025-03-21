# advprog-module6

## Commit 1 Reflection Notes

Di commit ini, saya mulai belajar membuat web server sederhana dengan Rust. Saya mengikuti cara membuat server yang mendengarkan koneksi di alamat `127.0.0.1:7878` menggunakan `TcpListener`. Setiap kali ada koneksi masuk, server akan membaca data dari browser dan mencetak request yang masuk ke konsol.

Awalnya saya agak bingung dengan cara kerja stream dan buffering, tapi dengan mencoba langsung, saya belajar bahwa:
- `BufReader` membantu saya membaca data secara baris per baris.
- Fungsi `take_while` berguna untuk berhenti membaca saat header request selesai (ketika menemukan baris kosong).

Meskipun saat ini server hanya menampilkan data request, saya merasa langkah ini sudah sangat membantu saya memahami dasar komunikasi antara browser dan server.
