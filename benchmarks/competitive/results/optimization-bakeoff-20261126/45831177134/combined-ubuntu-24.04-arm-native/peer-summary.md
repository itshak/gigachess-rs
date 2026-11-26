| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.84x | 1.45x | 1.28x | 2.04x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.44x | 1.40x | 1.27x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.16x | 1.26x | 1.05x | 1.54x | 1.46x | 29/30 |
| fen_parse | 1.01x | 1.23x | 1.34x | 1.27x | 1.74x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.44x | 9.28x | 1.56x | 29.80x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.09x | 1.02x | 1.47x | 1.02x | 0.95x | 19/30 |
| san_parse_all | 1.00x | 1.88x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.72x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 3.99x | 1.19x | 1.22x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 77.39x | 129.34x | 171.64x | 137.74x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.67x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.96x | 1.49x | 1.20x | 1.82x | 1.93x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.47x | 1.19x | 1.86x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.07x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.02x | 0.63x | 1.65x | 1.98x | 2.22x | 2.25x | 8/10 |
| replay_checked_native | 0.99x | 2.19x | 1.04x | 7.08x | 6.36x | 8.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
