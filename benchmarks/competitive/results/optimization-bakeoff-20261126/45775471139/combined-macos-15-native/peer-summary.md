| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.41x | 1.33x | 1.38x | 1.80x | 1.58x | 30/30 |
| materialize | 1.15x | 2.05x | 1.46x | 1.13x | 2.47x | 1.35x | 27/30 |
| perft_bulk_d4 | 1.01x | 4.07x | 1.55x | 1.26x | 1.86x | 1.82x | 29/30 |
| perft_full_d3 | 1.06x | 0.90x | 0.97x | 0.79x | 1.12x | 1.18x | 15/30 |
| fen_parse | 0.89x | 1.15x | 1.19x | 1.17x | 1.42x | 1.24x | 26/30 |
| fen_write_native_ep | 0.97x | 3.02x | 6.52x | 1.24x | 19.92x | 1.58x | 30/30 |
| snapshot | 0.91x | 1.55x | 1.06x | 2.24x | 1.12x | 0.95x | 24/30 |
| san_parse_all | 1.04x | 1.62x | N/A | 3.57x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.01x | N/A | 2.44x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 0.86x | 0.90x | 4.23x | 0.84x | 0.93x | 9/30 |
| make_unmake_full | 1.04x | N/A | N/A | 0.98x | N/A | N/A | 2/6 |
| startpos | 0.85x | 1.14x | 64.18x | 123.29x | 158.02x | 123.68x | 5/5 |
| chess960_materialize | 0.81x | 1.15x | 1.00x | N/A | N/A | N/A | 3/4 |
| chess960_perft_d3 | 0.99x | 2.81x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 38.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 19.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 6.30x | 2.22x | 1.28x | 2.40x | 2.71x | 10/10 |
| corpus_materialize | 1.09x | 1.44x | 1.35x | 1.16x | 1.81x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.22x | 1.34x | 1.82x | 1.42x | 1.82x | 1.04x | 9/10 |
| replay_trusted_native | 0.99x | 0.41x | 1.01x | 1.05x | 1.14x | 1.20x | 6/10 |
| replay_checked_native | 1.48x | 1.93x | 0.88x | 7.26x | 6.36x | 7.58x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
