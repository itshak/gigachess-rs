| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.97x | 1.72x | 1.45x | 2.25x | 1.78x | 30/30 |
| materialize | 1.07x | 2.00x | 1.59x | 1.65x | 2.47x | 1.86x | 30/30 |
| perft_bulk_d4 | 0.91x | 3.51x | 1.45x | 1.10x | 2.18x | 1.75x | 30/30 |
| perft_full_d3 | 1.00x | 1.53x | 1.72x | 1.69x | 2.33x | 1.85x | 30/30 |
| fen_parse | 0.98x | 1.24x | 1.23x | 1.02x | 1.40x | 1.11x | 25/30 |
| fen_write_native_ep | 0.98x | 3.20x | 6.85x | 1.24x | 20.12x | 1.61x | 30/30 |
| snapshot | 0.95x | 1.34x | 0.87x | 3.02x | 0.87x | 1.00x | 16/30 |
| san_parse_all | 0.96x | 1.84x | N/A | 4.14x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.85x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 0.69x | 1.21x | 0.78x | 2.24x | 0.81x | 0.93x | 12/30 |
| make_unmake_full | 0.84x | N/A | N/A | 0.97x | N/A | N/A | 1/6 |
| startpos | 1.09x | 1.32x | 88.09x | 167.86x | 205.11x | 174.15x | 5/5 |
| chess960_materialize | 0.99x | 1.66x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.10x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.08x | 20.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.50x | 2.50x | 1.34x | 3.13x | 2.76x | 10/10 |
| corpus_materialize | 1.02x | 1.45x | 1.36x | 1.10x | 1.79x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.27x | 3.11x | 2.26x | 3.00x | 1.56x | 10/10 |
| replay_trusted_native | 0.94x | 0.68x | 1.18x | 1.20x | 1.73x | 2.01x | 7/10 |
| replay_checked_native | 0.95x | 1.99x | 1.00x | 8.16x | 8.03x | 9.37x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
