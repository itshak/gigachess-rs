| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.87x | 1.66x | 1.43x | 2.26x | 1.76x | 30/30 |
| materialize | 0.98x | 1.96x | 1.46x | 1.56x | 2.28x | 1.68x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.21x | 1.31x | 1.00x | 1.96x | 1.58x | 26/30 |
| perft_full_d3 | 1.00x | 1.51x | 1.75x | 1.63x | 2.19x | 1.89x | 30/30 |
| fen_parse | 1.08x | 1.17x | 1.36x | 1.20x | 1.50x | 1.23x | 30/30 |
| fen_write_native_ep | 1.02x | 5.40x | 10.35x | 2.03x | 29.22x | 2.26x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.33x | 4.04x | 1.34x | 1.34x | 30/30 |
| san_parse_all | 0.98x | 1.84x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.81x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.05x | 1.33x | 4.00x | 1.29x | 1.65x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.00x | 82.37x | 143.51x | 181.78x | 141.97x | 4/5 |
| chess960_materialize | 1.01x | 1.44x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.22x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 19.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.06x | 1.60x | 0.81x | 1.90x | 1.77x | 8/10 |
| corpus_materialize | 1.01x | 1.32x | 1.24x | 1.01x | 1.59x | 1.12x | 9/10 |
| corpus_uci_sorted | 1.00x | 2.34x | 2.98x | 2.21x | 2.88x | 1.55x | 10/10 |
| replay_trusted_native | 0.97x | 0.83x | 1.52x | 1.51x | 2.09x | 2.43x | 9/10 |
| replay_checked_native | 1.02x | 1.98x | 1.09x | 8.53x | 8.45x | 9.71x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
