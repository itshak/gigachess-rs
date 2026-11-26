| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.22x | 1.48x | 1.45x | 2.09x | 1.85x | 30/30 |
| materialize | 1.03x | 1.93x | 1.28x | 1.18x | 2.23x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.04x | 3.88x | 1.53x | 1.38x | 2.22x | 2.06x | 30/30 |
| perft_full_d3 | 0.95x | 1.21x | 1.28x | 1.14x | 1.69x | 1.62x | 29/30 |
| fen_parse | 1.12x | 1.38x | 1.49x | 1.37x | 2.24x | 1.48x | 30/30 |
| fen_write_native_ep | 0.98x | 2.68x | 5.23x | 0.97x | 19.56x | 1.21x | 25/30 |
| snapshot | 1.32x | 1.82x | 1.19x | 3.35x | 1.25x | 1.29x | 30/30 |
| san_parse_all | 1.05x | 2.02x | N/A | 5.57x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.99x | N/A | 3.84x | N/A | N/A | 12/12 |
| apply_children_full | 0.86x | 1.60x | 1.01x | 6.54x | 1.05x | 1.17x | 25/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.25x | 1.25x | 81.62x | 161.47x | 232.54x | 171.71x | 5/5 |
| chess960_materialize | 1.08x | 1.50x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.08x | 2.58x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 27.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.09x | 20.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 5.30x | 1.84x | 1.35x | 2.44x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.30x | 1.16x | 2.01x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.74x | 2.19x | 1.86x | 2.13x | 1.38x | 10/10 |
| replay_trusted_native | 1.09x | 0.73x | 1.33x | 1.66x | 2.11x | 2.47x | 8/10 |
| replay_checked_native | 1.04x | 2.23x | 0.99x | 7.47x | 7.27x | 8.87x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
