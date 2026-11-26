| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.72x | 1.56x | 1.35x | 1.83x | 1.30x | 29/30 |
| materialize | 1.01x | 2.14x | 1.72x | 1.71x | 2.63x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.92x | 2.71x | 1.44x | 1.07x | 1.81x | 1.61x | 29/30 |
| perft_full_d3 | 0.96x | 1.50x | 1.74x | 1.53x | 2.13x | 1.75x | 30/30 |
| fen_parse | 1.03x | 1.75x | 1.47x | 1.22x | 1.62x | 1.29x | 30/30 |
| fen_write_native_ep | 0.99x | 3.06x | 9.76x | 1.93x | 26.84x | 2.29x | 30/30 |
| snapshot | 1.12x | 1.75x | 1.25x | 3.26x | 1.25x | 1.00x | 29/30 |
| san_parse_all | 0.92x | 1.76x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.08x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.00x | 1.35x | 3.79x | 1.22x | 1.43x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.83x | 110.81x | 129.56x | 111.26x | 5/5 |
| chess960_materialize | 1.05x | 1.56x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 1.96x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 41.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.72x | 17.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.78x | 3.64x | 1.81x | 0.97x | 2.01x | 2.49x | 8/10 |
| corpus_materialize | 0.93x | 1.43x | 1.33x | 1.08x | 1.77x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.91x | 2.85x | 2.14x | 2.76x | 1.51x | 10/10 |
| replay_trusted_native | 1.02x | 0.83x | 1.55x | 1.50x | 2.11x | 1.94x | 9/10 |
| replay_checked_native | 0.99x | 2.00x | 1.13x | 8.58x | 8.29x | 9.53x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
