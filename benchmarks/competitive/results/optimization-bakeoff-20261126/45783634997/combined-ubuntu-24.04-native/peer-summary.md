| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.31x | 1.51x | 1.43x | 2.11x | 1.81x | 30/30 |
| materialize | 0.99x | 1.97x | 1.62x | 1.12x | 2.22x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.98x | 1.55x | 1.21x | 2.38x | 2.20x | 30/30 |
| perft_full_d3 | 1.16x | 1.23x | 1.48x | 1.10x | 2.12x | 1.85x | 29/30 |
| fen_parse | 0.98x | 1.36x | 1.53x | 1.21x | 2.03x | 1.21x | 30/30 |
| fen_write_native_ep | 1.32x | 4.18x | 9.22x | 1.47x | 25.98x | 1.91x | 30/30 |
| snapshot | 0.98x | 1.78x | 0.97x | 2.88x | 1.18x | 0.96x | 18/30 |
| san_parse_all | 0.99x | 1.77x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.45x | N/A | 2.33x | N/A | N/A | 12/12 |
| apply_children_full | 1.21x | 1.68x | 1.16x | 3.98x | 1.17x | 1.63x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.24x | 86.31x | 144.68x | 218.06x | 142.66x | 5/5 |
| chess960_materialize | 1.00x | 1.55x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.57x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 39.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 29.68x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.01x | 1.82x | 1.19x | 2.36x | 2.27x | 10/10 |
| corpus_materialize | 0.96x | 1.68x | 1.60x | 1.12x | 2.21x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.36x | 3.04x | 2.40x | 3.07x | 1.66x | 10/10 |
| replay_trusted_native | 1.03x | 0.64x | 1.20x | 1.33x | 2.15x | 2.73x | 8/10 |
| replay_checked_native | 1.01x | 2.01x | 0.88x | 6.70x | 6.87x | 8.44x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
