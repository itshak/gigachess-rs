| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.84x | 1.79x | 1.71x | 2.66x | 2.17x | 30/30 |
| materialize | 1.02x | 2.02x | 1.38x | 1.14x | 2.33x | 1.54x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.96x | 1.62x | 1.43x | 2.54x | 2.21x | 30/30 |
| perft_full_d3 | 1.01x | 1.50x | 1.55x | 1.24x | 2.09x | 1.85x | 30/30 |
| fen_parse | 1.13x | 1.53x | 1.61x | 1.59x | 2.30x | 1.60x | 30/30 |
| fen_write_native_ep | 0.98x | 3.95x | 7.54x | 1.57x | 30.58x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.80x | 1.27x | 4.16x | 1.26x | 1.21x | 30/30 |
| san_parse_all | 1.02x | 1.97x | N/A | 4.73x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.28x | N/A | 3.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.97x | 1.33x | 8.35x | 1.56x | 1.72x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 83.96x | 141.32x | 203.74x | 146.48x | 4/5 |
| chess960_materialize | 0.99x | 1.65x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.81x | 2.36x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.06x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.62x | 1.78x | 1.31x | 2.37x | 2.13x | 9/10 |
| corpus_materialize | 1.00x | 1.78x | 1.42x | 1.18x | 2.84x | 1.51x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.86x | 2.38x | 1.95x | 2.26x | 1.47x | 10/10 |
| replay_trusted_native | 0.98x | 1.14x | 1.96x | 2.47x | 3.90x | 4.82x | 10/10 |
| replay_checked_native | 1.08x | 2.55x | 1.09x | 6.25x | 8.19x | 8.38x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
