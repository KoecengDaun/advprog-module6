# advprog-module6

## Commit 1 Reflection Notes

Di commit ini, saya mulai belajar membuat web server sederhana dengan Rust. Saya mengikuti cara membuat server yang mendengarkan koneksi di alamat `127.0.0.1:7878` menggunakan `TcpListener`. Setiap kali ada koneksi masuk, server akan membaca data dari browser dan mencetak request yang masuk ke konsol.

Awalnya saya agak bingung dengan cara kerja stream dan buffering, tapi dengan mencoba langsung, saya belajar bahwa:
- `BufReader` membantu saya membaca data secara baris per baris.
- Fungsi `take_while` berguna untuk berhenti membaca saat header request selesai (ketika menemukan baris kosong).

Meskipun saat ini server hanya menampilkan data request, saya merasa langkah ini sudah sangat membantu saya memahami dasar komunikasi antara browser dan server.

## Commit 2 Reflection Notes
Pada milestone ini, saya belajar bagaimana cara mengirimkan file HTML sebagai respon dari web server sederhana yang ditulis dalam Rust. Secara garis besar:

- Saya membaca file hello.html menggunakan fs::read_to_string.
- Saya menetapkan header HTTP sederhana seperti HTTP/1.1 200 OK dan Content-Length.
- Saya menyusun respon lengkap berupa teks (header + isi HTML) lalu mengirimkannya ke browser.

Pada tahap ini, saya jadi lebih paham bahwa ketika browser meminta suatu halaman, server perlu mengirimkan status line, header, dan body dalam format yang sesuai protokol HTTP. Dengan begitu, browser bisa menampilkan konten HTML dengan benar. Meskipun masih sederhana, ini sudah menunjukkan alur dasar bagaimana sebuah web server menyiapkan halaman untuk peng

### Gambar Commit 2
![Commit 2 screen capture](hello\gambar\commit2.png)
