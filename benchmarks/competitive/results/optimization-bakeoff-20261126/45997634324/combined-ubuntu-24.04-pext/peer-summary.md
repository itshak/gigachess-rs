| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.74x | 1.96x | 1.71x | 2.63x | 2.02x | 30/30 |
| materialize | 0.94x | 2.17x | 1.64x | 1.74x | 2.60x | 1.86x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.04x | 1.60x | 1.25x | 2.57x | 1.97x | 29/30 |
| perft_full_d3 | 1.25x | 1.68x | 2.00x | 1.90x | 2.79x | 2.02x | 30/30 |
| fen_parse | 0.99x | 1.34x | 1.36x | 1.07x | 1.46x | 1.24x | 30/30 |
| fen_write_native_ep | 1.02x | 4.55x | 10.31x | 2.14x | 28.48x | 2.66x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.16x | 4.31x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 2.08x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.32x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.11x | 1.33x | 3.99x | 1.33x | 1.66x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.10x | 87.55x | 153.14x | 191.67x | 172.16x | 5/5 |
| chess960_materialize | 0.93x | 1.61x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.56x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 38.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 19.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 6.26x | 2.32x | 1.29x | 2.82x | 2.59x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.29x | 1.08x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.31x | 2.94x | 2.23x | 2.86x | 1.59x | 10/10 |
| replay_trusted_native | 1.00x | 0.91x | 1.65x | 1.58x | 2.25x | 2.59x | 9/10 |
| replay_checked_native | 0.98x | 2.13x | 1.18x | 8.96x | 8.61x | 10.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
