| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 2.94x | 1.49x | 1.36x | 1.92x | 1.35x | 30/30 |
| materialize | 1.02x | 2.10x | 1.77x | 1.31x | 2.36x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.92x | 2.66x | 1.43x | 1.12x | 1.82x | 1.56x | 29/30 |
| perft_full_d3 | 0.99x | 1.42x | 1.64x | 1.34x | 1.96x | 1.80x | 30/30 |
| fen_parse | 1.03x | 1.56x | 1.51x | 1.21x | 2.24x | 1.28x | 30/30 |
| fen_write_native_ep | 1.01x | 2.86x | 9.53x | 1.71x | 27.35x | 2.06x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.22x | 2.94x | 1.01x | 0.98x | 25/30 |
| san_parse_all | 0.99x | 2.08x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.53x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.97x | 1.32x | 3.90x | 1.21x | 1.16x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 65.62x | 115.72x | 183.26x | 115.15x | 5/5 |
| chess960_materialize | 1.04x | 1.52x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.13x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 24.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 17.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 3.59x | 1.78x | 1.20x | 1.98x | 2.39x | 10/10 |
| corpus_materialize | 0.94x | 1.50x | 1.51x | 1.12x | 1.96x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.33x | 3.23x | 2.43x | 3.14x | 1.81x | 10/10 |
| replay_trusted_native | 0.97x | 0.68x | 1.37x | 1.53x | 2.08x | 1.92x | 8/10 |
| replay_checked_native | 1.02x | 2.25x | 1.07x | 7.61x | 7.54x | 8.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
