| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.84x | 4.01x | 1.32x | 1.27x | 1.82x | 1.85x | 29/30 |
| materialize | 0.97x | 2.10x | 1.45x | 1.06x | 2.17x | 1.27x | 26/30 |
| perft_bulk_d4 | 0.97x | 4.31x | 1.59x | 1.41x | 2.34x | 1.93x | 30/30 |
| perft_full_d3 | 1.02x | 1.58x | 1.76x | 1.39x | 1.98x | 2.11x | 30/30 |
| fen_parse | 0.90x | 1.15x | 1.21x | 1.23x | 1.60x | 1.37x | 28/30 |
| fen_write_native_ep | 1.30x | 4.20x | 8.75x | 1.63x | 26.59x | 2.20x | 30/30 |
| snapshot | 0.88x | 1.56x | 1.12x | 2.24x | 1.12x | 0.87x | 19/30 |
| san_parse_all | 0.85x | 1.91x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.85x | 11.24x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 0.86x | 1.06x | 1.08x | 5.41x | 1.03x | 1.11x | 20/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.16x | N/A | N/A | 4/6 |
| startpos | 1.01x | 1.83x | 87.56x | 172.32x | 219.98x | 209.07x | 5/5 |
| chess960_materialize | 1.00x | 1.80x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.85x | 3.03x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 35.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.05x | 21.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.84x | 5.96x | 1.73x | 1.24x | 2.15x | 2.65x | 10/10 |
| corpus_materialize | 0.86x | 1.58x | 1.40x | 1.09x | 1.88x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.88x | 1.70x | 2.53x | 1.82x | 2.34x | 1.26x | 10/10 |
| replay_trusted_native | 0.95x | 0.54x | 1.38x | 1.65x | 1.67x | 1.67x | 8/10 |
| replay_checked_native | 0.93x | 2.24x | 1.08x | 7.97x | 7.49x | 9.33x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
