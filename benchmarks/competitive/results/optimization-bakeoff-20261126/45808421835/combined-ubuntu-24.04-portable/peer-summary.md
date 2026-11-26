| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.94x | 1.50x | 1.39x | 1.88x | 1.34x | 30/30 |
| materialize | 1.00x | 2.02x | 1.77x | 1.31x | 2.37x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.92x | 1.58x | 1.24x | 1.98x | 1.73x | 30/30 |
| perft_full_d3 | 1.02x | 1.48x | 1.67x | 1.36x | 2.02x | 1.90x | 30/30 |
| fen_parse | 0.95x | 1.42x | 1.47x | 1.30x | 2.16x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 2.92x | 9.50x | 1.73x | 26.80x | 2.11x | 30/30 |
| snapshot | 1.01x | 1.69x | 1.20x | 2.91x | 1.01x | 0.97x | 23/30 |
| san_parse_all | 1.02x | 2.09x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.99x | N/A | 2.41x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.05x | 1.40x | 4.12x | 1.28x | 1.27x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 63.97x | 116.53x | 176.97x | 104.84x | 5/5 |
| chess960_materialize | 0.99x | 1.48x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.10x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 24.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 18.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.84x | 1.93x | 1.29x | 2.12x | 2.57x | 10/10 |
| corpus_materialize | 0.99x | 1.64x | 1.67x | 1.23x | 2.13x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.20x | 3.19x | 2.50x | 3.07x | 1.81x | 10/10 |
| replay_trusted_native | 1.04x | 0.80x | 1.51x | 1.83x | 2.42x | 2.21x | 8/10 |
| replay_checked_native | 1.05x | 2.34x | 1.08x | 7.65x | 7.58x | 8.86x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
