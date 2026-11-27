| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.70x | 1.72x | 1.68x | 2.58x | 2.00x | 30/30 |
| materialize | 1.00x | 2.16x | 1.76x | 1.18x | 2.22x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.15x | 1.63x | 1.35x | 2.58x | 2.23x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.55x | 1.21x | 2.24x | 1.88x | 30/30 |
| fen_parse | 1.01x | 1.37x | 1.57x | 1.19x | 1.98x | 1.45x | 30/30 |
| fen_write_native_ep | 1.00x | 4.07x | 9.49x | 1.84x | 27.88x | 2.15x | 30/30 |
| snapshot | 1.91x | 1.92x | 1.25x | 3.76x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.27x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.44x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.01x | 1.80x | 5.04x | 1.85x | 2.02x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.11x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.20x | 101.19x | 136.75x | 209.17x | 161.44x | 5/5 |
| chess960_materialize | 1.00x | 2.00x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.09x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.96x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.48x | 1.77x | 1.30x | 2.36x | 2.06x | 9/10 |
| corpus_materialize | 1.00x | 1.95x | 1.77x | 1.23x | 2.88x | 1.56x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.33x | 3.14x | 2.34x | 2.92x | 1.66x | 10/10 |
| replay_trusted_native | 1.19x | 1.19x | 2.05x | 2.31x | 3.95x | 4.88x | 10/10 |
| replay_checked_native | 1.02x | 3.00x | 1.10x | 6.38x | 7.58x | 8.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
