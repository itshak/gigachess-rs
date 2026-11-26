| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.59x | 1.87x | 1.63x | 2.52x | 1.91x | 30/30 |
| materialize | 0.92x | 2.08x | 1.53x | 1.71x | 2.56x | 1.81x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.81x | 1.52x | 1.19x | 2.35x | 1.85x | 29/30 |
| perft_full_d3 | 1.02x | 1.42x | 1.54x | 1.53x | 1.98x | 1.64x | 30/30 |
| fen_parse | 1.00x | 1.28x | 1.32x | 1.16x | 1.46x | 1.26x | 30/30 |
| fen_write_native_ep | 0.97x | 4.80x | 9.84x | 1.98x | 28.25x | 2.10x | 30/30 |
| snapshot | 1.01x | 1.82x | 1.16x | 4.32x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 0.99x | 1.86x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.83x | N/A | 2.04x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.09x | 1.32x | 3.96x | 1.31x | 1.65x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.12x | 88.33x | 163.75x | 189.23x | 171.37x | 5/5 |
| chess960_materialize | 0.94x | 1.65x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 19.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.12x | 1.91x | 1.06x | 2.33x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.31x | 1.07x | 1.75x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 3.02x | 2.21x | 2.98x | 1.55x | 10/10 |
| replay_trusted_native | 0.93x | 0.82x | 1.50x | 1.51x | 2.07x | 2.42x | 9/10 |
| replay_checked_native | 0.95x | 2.09x | 1.10x | 8.65x | 8.51x | 9.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
