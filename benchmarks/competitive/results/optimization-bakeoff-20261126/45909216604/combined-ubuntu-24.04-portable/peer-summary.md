| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.95x | 1.69x | 1.47x | 1.99x | 1.45x | 30/30 |
| materialize | 1.00x | 2.28x | 1.89x | 1.85x | 2.89x | 2.00x | 30/30 |
| perft_bulk_d4 | 1.03x | 3.13x | 1.65x | 1.23x | 2.08x | 1.88x | 30/30 |
| perft_full_d3 | 0.99x | 1.36x | 1.62x | 1.40x | 1.92x | 1.66x | 30/30 |
| fen_parse | 1.01x | 1.54x | 1.44x | 1.23x | 1.61x | 1.27x | 30/30 |
| fen_write_native_ep | 1.02x | 2.97x | 9.84x | 1.94x | 27.64x | 2.57x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 0.92x | 1.82x | N/A | 4.14x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.14x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.21x | 1.49x | 4.28x | 1.34x | 1.58x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.69x | 109.53x | 132.51x | 113.87x | 5/5 |
| chess960_materialize | 0.91x | 1.58x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.14x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.15x | 42.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 21.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 4.69x | 2.34x | 1.25x | 2.36x | 3.28x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.41x | 1.14x | 1.83x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.97x | 2.89x | 2.18x | 2.79x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.91x | 1.63x | 1.63x | 2.27x | 2.09x | 9/10 |
| replay_checked_native | 1.01x | 2.11x | 1.16x | 8.86x | 8.68x | 9.81x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
