| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.07x | 1.26x | 1.53x | 1.97x | 1.82x | 29/30 |
| materialize | 0.99x | 2.02x | 1.54x | 1.07x | 2.30x | 1.30x | 28/30 |
| perft_bulk_d4 | 0.94x | 4.02x | 1.52x | 1.24x | 1.78x | 1.86x | 29/30 |
| perft_full_d3 | 1.00x | 1.58x | 1.68x | 1.36x | 1.98x | 1.97x | 30/30 |
| fen_parse | 1.15x | 1.26x | 1.40x | 1.43x | 2.03x | 1.68x | 30/30 |
| fen_write_native_ep | 0.98x | 4.27x | 9.30x | 1.77x | 28.76x | 2.27x | 30/30 |
| snapshot | 1.08x | 1.51x | 1.08x | 2.24x | 1.03x | 1.04x | 22/30 |
| san_parse_all | 1.06x | 1.71x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 12.08x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 0.93x | 1.00x | 4.44x | 0.96x | 1.10x | 16/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.08x | N/A | N/A | 3/6 |
| startpos | 1.02x | 1.27x | 73.14x | 142.86x | 182.04x | 148.72x | 5/5 |
| chess960_materialize | 1.16x | 1.81x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 3.04x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 40.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.11x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.24x | 6.75x | 2.39x | 1.58x | 2.79x | 3.05x | 10/10 |
| corpus_materialize | 1.28x | 1.49x | 1.26x | 1.13x | 1.98x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.94x | 1.77x | 2.37x | 1.82x | 2.35x | 1.39x | 10/10 |
| replay_trusted_native | 0.99x | 0.58x | 1.39x | 1.53x | 1.58x | 1.57x | 8/10 |
| replay_checked_native | 1.00x | 2.09x | 0.89x | 7.16x | 6.16x | 7.65x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
