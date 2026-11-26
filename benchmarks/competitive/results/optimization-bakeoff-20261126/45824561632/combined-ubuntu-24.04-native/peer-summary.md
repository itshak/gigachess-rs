| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.80x | 1.64x | 1.38x | 2.16x | 1.69x | 30/30 |
| materialize | 0.98x | 2.07x | 1.59x | 1.64x | 2.47x | 1.86x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.26x | 1.34x | 1.02x | 2.03x | 1.62x | 26/30 |
| perft_full_d3 | 0.88x | 1.32x | 1.51x | 1.44x | 1.90x | 1.79x | 30/30 |
| fen_parse | 1.06x | 1.39x | 1.58x | 1.18x | 1.56x | 1.13x | 30/30 |
| fen_write_native_ep | 0.98x | 5.08x | 10.28x | 1.90x | 27.93x | 2.48x | 30/30 |
| snapshot | 0.61x | 1.50x | 0.94x | 2.65x | 0.94x | 1.00x | 15/30 |
| san_parse_all | 0.91x | 1.76x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 8.92x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.26x | 2.01x | 3.90x | 1.59x | 1.94x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.29x | 80.99x | 152.93x | 173.44x | 141.56x | 5/5 |
| chess960_materialize | 0.93x | 1.61x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.20x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 35.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 18.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 4.55x | 1.72x | 0.95x | 2.08x | 1.93x | 8/10 |
| corpus_materialize | 1.01x | 1.36x | 1.27x | 1.03x | 1.68x | 1.15x | 9/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 2.99x | 2.28x | 2.94x | 1.56x | 10/10 |
| replay_trusted_native | 0.90x | 0.80x | 1.48x | 1.44x | 2.02x | 2.33x | 8/10 |
| replay_checked_native | 0.94x | 1.88x | 1.04x | 7.72x | 7.48x | 8.99x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
