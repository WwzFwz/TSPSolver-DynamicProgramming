<h1 align="center">Tantangan IF2211 Strategi Algoritma</h1>
<h2 align="center">Semester II tahun 2024/2025</h2>
<h2 align="center">Traveling Salesman Problem Solver (Dynamic Programming)</h2>

## Table of Contents

- [Description](#description)
- [Requirements & Installation](#requirements--installation)
- [How to Run](#how-to-run)
- [Author](#author)

## Description

 **Traveling Salesman Problem (TSP)** using **Dynamic Programming with Bitmasking**. 

## Requirements & Installation

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (Included in Rust)

## How to Use

1. **Clone the repository**

   ```sh
   git clone git@github.com:WwzFwz/TSPSolver-DynamicProgramming.git
   cd TSP-DynamicProgramming-Solver
   ```
1. **Run with input file**
   ```sh
     cargo run <relative_path_from_root>
   ```
   example :

   ```sh
     cargo run test/input/input1.txt
   ```
## Konfigurasi Input File
    ```bash
        <jumlah_kota>
        <data_graf>
    ```
    Penjelasan:

    Baris 1: Jumlah kota (n)
    Baris 2 sampai n+1: Matrix adjacency n×n
    Element [i][j]: Jarak dari kota i ke kota j
    Diagonal: Harus 0 (jarak kota ke dirinya sendiri)
    Simetris: Untuk graf tidak berarah

## Author

| **NIM**  | **Nama Anggota**       | **Github**                            |
| -------- | ---------------------- | ------------------------------------- |
| 13523065 | Dzaky Aurelia Fawwaz   | [WwzFwz](https://github.com/WwzFwz) |