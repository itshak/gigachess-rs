| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.40x | 1.67x | 1.83x | 2.17x | 2.16x | 30/30 |
| materialize | 0.99x | 1.93x | 1.51x | 1.33x | 2.14x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.57x | 1.43x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.61x | 1.33x | 1.98x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.22x | 1.30x | 1.25x | 1.73x | 1.38x | 30/30 |
| fen_write_native_ep | 0.99x | 4.44x | 9.26x | 1.60x | 29.87x | 2.38x | 30/30 |
| snapshot | 0.99x | 1.04x | 1.10x | 1.45x | 1.11x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.69x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 0.82x | 1.02x | 1.22x | 3.34x | 0.98x | 0.98x | 21/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.68x | 128.58x | 172.84x | 138.99x | 5/5 |
| chess960_materialize | 0.99x | 1.69x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.96x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.46x | 1.71x | 1.39x | 2.00x | 2.22x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.53x | 1.23x | 1.91x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.92x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 0.97x | 0.61x | 1.57x | 1.93x | 2.15x | 2.18x | 8/10 |
| replay_checked_native | 1.02x | 2.21x | 1.08x | 7.27x | 6.47x | 8.39x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
