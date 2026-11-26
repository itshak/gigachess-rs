| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.40x | 1.48x | 1.44x | 1.81x | 1.77x | 29/30 |
| materialize | 0.86x | 1.76x | 1.45x | 1.01x | 2.01x | 1.28x | 25/30 |
| perft_bulk_d4 | 0.86x | 3.93x | 1.60x | 1.32x | 2.07x | 1.94x | 30/30 |
| perft_full_d3 | 0.97x | 1.48x | 1.81x | 1.27x | 1.80x | 1.89x | 30/30 |
| fen_parse | 1.14x | 1.40x | 1.50x | 1.55x | 1.82x | 1.62x | 30/30 |
| fen_write_native_ep | 0.95x | 2.97x | 6.47x | 1.24x | 18.81x | 1.58x | 30/30 |
| snapshot | 0.87x | 1.31x | 1.03x | 2.05x | 1.04x | 0.85x | 20/30 |
| san_parse_all | 0.88x | 1.70x | N/A | 4.05x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.30x | N/A | 2.45x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 0.94x | 1.06x | 4.47x | 0.94x | 1.14x | 18/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.10x | N/A | N/A | 5/6 |
| startpos | 1.12x | 1.48x | 88.84x | 148.44x | 195.72x | 173.07x | 5/5 |
| chess960_materialize | 1.06x | 1.45x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.18x | 3.03x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.27x | 34.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.30x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.24x | 19.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 6.36x | 2.19x | 1.46x | 2.67x | 2.73x | 10/10 |
| corpus_materialize | 1.12x | 1.56x | 1.42x | 1.25x | 2.13x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.14x | 1.93x | 3.01x | 1.87x | 2.58x | 1.41x | 10/10 |
| replay_trusted_native | 0.84x | 0.52x | 1.30x | 1.36x | 1.53x | 1.41x | 8/10 |
| replay_checked_native | 0.95x | 2.01x | 0.97x | 6.24x | 6.45x | 7.27x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
