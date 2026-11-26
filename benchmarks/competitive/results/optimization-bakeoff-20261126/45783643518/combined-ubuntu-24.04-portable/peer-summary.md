| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.89x | 1.50x | 1.34x | 1.90x | 1.33x | 30/30 |
| materialize | 1.01x | 1.99x | 1.75x | 1.27x | 2.37x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.90x | 1.56x | 1.23x | 1.98x | 1.71x | 30/30 |
| perft_full_d3 | 1.01x | 1.43x | 1.63x | 1.35x | 1.98x | 1.79x | 30/30 |
| fen_parse | 0.85x | 1.28x | 1.37x | 1.05x | 2.04x | 1.14x | 30/30 |
| fen_write_native_ep | 0.98x | 1.67x | 5.11x | 0.92x | 14.39x | 1.10x | 24/30 |
| snapshot | 1.00x | 1.70x | 1.22x | 3.06x | 1.01x | 0.97x | 24/30 |
| san_parse_all | 0.99x | 2.03x | N/A | 4.36x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.47x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.91x | 1.35x | 3.95x | 1.24x | 1.28x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.34x | 67.91x | 113.84x | 191.81x | 115.31x | 5/5 |
| chess960_materialize | 0.98x | 1.44x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.09x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 24.57x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 17.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.85x | 1.91x | 1.28x | 2.12x | 2.56x | 10/10 |
| corpus_materialize | 1.02x | 1.61x | 1.63x | 1.17x | 2.11x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.16x | 3.17x | 2.40x | 3.06x | 1.71x | 10/10 |
| replay_trusted_native | 1.03x | 0.73x | 1.35x | 1.59x | 2.14x | 2.00x | 8/10 |
| replay_checked_native | 1.01x | 2.16x | 1.05x | 7.21x | 7.53x | 8.40x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
