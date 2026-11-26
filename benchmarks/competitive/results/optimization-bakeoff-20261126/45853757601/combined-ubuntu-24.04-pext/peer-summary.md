| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.50x | 1.88x | 1.63x | 2.52x | 1.92x | 30/30 |
| materialize | 1.00x | 2.08x | 1.55x | 1.71x | 2.46x | 1.81x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.86x | 1.53x | 1.20x | 2.38x | 1.88x | 29/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.55x | 1.51x | 1.98x | 1.63x | 30/30 |
| fen_parse | 1.00x | 1.33x | 1.34x | 1.05x | 1.44x | 1.23x | 28/30 |
| fen_write_native_ep | 0.96x | 4.74x | 9.96x | 1.84x | 28.34x | 2.54x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.15x | 1.34x | 30/30 |
| san_parse_all | 1.09x | 1.90x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.10x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.07x | 1.30x | 3.97x | 1.30x | 2.01x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.12x | 88.67x | 155.95x | 191.31x | 171.94x | 5/5 |
| chess960_materialize | 1.01x | 1.65x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 19.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.13x | 1.87x | 1.05x | 2.34x | 2.12x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.33x | 1.09x | 1.74x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.29x | 3.09x | 2.26x | 3.01x | 1.59x | 10/10 |
| replay_trusted_native | 0.97x | 0.88x | 1.55x | 1.53x | 2.15x | 2.50x | 9/10 |
| replay_checked_native | 1.02x | 2.09x | 1.08x | 8.65x | 8.24x | 9.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
