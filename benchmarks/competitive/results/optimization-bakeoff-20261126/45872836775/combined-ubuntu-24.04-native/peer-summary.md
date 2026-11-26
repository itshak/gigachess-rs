| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.74x | 1.74x | 1.66x | 2.59x | 2.01x | 30/30 |
| materialize | 1.05x | 2.15x | 1.76x | 1.16x | 2.21x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.12x | 1.63x | 1.40x | 2.58x | 2.20x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.58x | 1.20x | 2.27x | 1.91x | 30/30 |
| fen_parse | 0.98x | 1.33x | 1.49x | 1.12x | 2.05x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.06x | 10.15x | 1.86x | 28.47x | 2.18x | 30/30 |
| snapshot | 1.00x | 1.92x | 1.25x | 3.66x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.24x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.07x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.97x | 1.29x | 4.72x | 1.50x | 1.68x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 98.10x | 138.40x | 233.99x | 164.90x | 5/5 |
| chess960_materialize | 1.01x | 1.93x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.08x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.82x | 0.84x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 18.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.35x | 1.76x | 1.29x | 2.34x | 2.03x | 9/10 |
| corpus_materialize | 0.99x | 1.92x | 1.81x | 1.22x | 2.79x | 1.54x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.33x | 3.18x | 2.36x | 2.99x | 1.67x | 10/10 |
| replay_trusted_native | 1.18x | 1.19x | 2.27x | 2.32x | 3.97x | 4.86x | 10/10 |
| replay_checked_native | 1.02x | 2.91x | 1.21x | 6.38x | 7.27x | 8.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
