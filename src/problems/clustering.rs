// src/problems/clustering.rs
use anyhow::Result;
use crate::core::base::DecisionResult;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct InputObject {
    id: String,
    features: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct ClusteringInput {
    objects: Vec<InputObject>,
}

pub struct ClusteringSolver;

impl ClusteringSolver {
    /// python_path: Some("/path/to/venv/bin/python") или None -> "python3"
    pub fn solve(python_path: Option<&str>) -> Result<DecisionResult> {
        // read input file
        let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        in_path.push("data/data_for_clustering.json");
        let raw = fs::read_to_string(&in_path)?;
        let input: ClusteringInput = serde_json::from_str(&raw)?;

        let n = input.objects.len();
        if n == 0 {
            anyhow::bail!("No objects provided for clustering");
        }
        // проверка согласованности
        let feat_len = input.objects[0].features.len();
        for (i, obj) in input.objects.iter().enumerate() {
            if obj.features.len() != feat_len {
                anyhow::bail!("Несоответствие числа признаков у объекта с индексом {}", i);
            }
        }

        // попарные евклидовы расстояния
        let mut dist = vec![vec![0.0_f64; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let d = input.objects[i].features.iter()
                    .zip(input.objects[j].features.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>()
                    .sqrt();
                dist[i][j] = d;
                dist[j][i] = d;
            }
        }

        let mut clusters_members: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();
        let mut clusters_id: Vec<usize> = (0..n).collect(); // initial ids 0..n-1
        let mut next_cluster_id: usize = n;

        // элементы матрицы связей: (idx1, idx2, dist, size) - индексы, расстояние и размер кластеров
        let mut linkage: Vec<(usize, usize, f64, usize)> = Vec::new();

        while clusters_members.len() > 1 {
            // find pair with minimal single-link distance
            let mut best_i = 0usize;
            let mut best_j = 1usize;
            let mut best_d = f64::MAX;

            for i in 0..clusters_members.len() {
                for j in (i + 1)..clusters_members.len() {
                    // минимальное расстояние между любыми элементами двух кластеров
                    let mut dmin = f64::MAX;
                    for &a in &clusters_members[i] {
                        for &b in &clusters_members[j] {
                            if dist[a][b] < dmin {
                                dmin = dist[a][b];
                            }
                        }
                    }
                    if dmin < best_d {
                        best_d = dmin;
                        best_i = i;
                        best_j = j;
                    }
                }
            }

            // id для строки матрицы связей должны соответствовать текущим id кластеров
            let id_i = clusters_id[best_i];
            let id_j = clusters_id[best_j];

            // новые элементы кластера и его id
            let mut new_members = clusters_members[best_i].clone();
            new_members.extend(clusters_members[best_j].iter().cloned());
            let new_id = next_cluster_id;
            let new_size = new_members.len();

            // добавляем строку в матрицу связей
            linkage.push((id_i, id_j, best_d, new_size));

            // удаляем кластеры с большим индексом сначала
            // (чтобы избежать сдвига индексов)
            if best_i < best_j {
                clusters_members.remove(best_j);
                clusters_members.remove(best_i);
                clusters_id.remove(best_j);
                clusters_id.remove(best_i);
            } else {
                clusters_members.remove(best_i);
                clusters_members.remove(best_j);
                clusters_id.remove(best_i);
                clusters_id.remove(best_j);
            }

            // добавляем новый кластер
            clusters_members.push(new_members);
            clusters_id.push(new_id);

            next_cluster_id += 1;
        }

        // записываем матрицу в формате: idx1 idx2 dist size - в txt файл
        let mut out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        out_path.push("data/answer_for_clustering.txt");
        let mut out = String::new();
        for (a, b, d, s) in &linkage {
            out.push_str(&format!("{} {} {:.6} {}\n", a, b, d, s));
        }
        fs::write(&out_path, out)?;

        // python-срипт отрисовки дендрограммы
        let mut python_bin = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        script_path.push("src/tools/draw_dendrogram.py");
        python_bin.push("src/tools/lvenv/bin/python3");

        let status = Command::new(python_bin)
            .arg(script_path)
            .arg(&out_path)
            .status()?;

        if !status.success() {
            anyhow::bail!("Python dendrogram script failed with non-zero exit");
        }

        // подготовка результата в DecisionResult для printer
        let scores: Vec<(String, f64)> = input
            .objects
            .iter()
            .enumerate()
            .map(|(i, obj)| (obj.id.clone(), i as f64))
            .collect();

        let chosen = vec![
            format!("linkage_file: {}", out_path.to_string_lossy()),
            format!("dendrogram: dendrogram.png"),
        ];

        Ok(DecisionResult {
            chosen,
            scores,
            method: "clustering_single_linkage".to_string(),
        })
    }
}
