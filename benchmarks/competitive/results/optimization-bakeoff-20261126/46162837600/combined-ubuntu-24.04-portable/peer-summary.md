| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.01x | 1.73x | 1.51x | 2.02x | 1.48x | 30/30 |
| materialize | 1.00x | 2.39x | 1.89x | 1.88x | 2.97x | 2.02x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.25x | 1.71x | 1.27x | 2.11x | 1.93x | 30/30 |
| perft_full_d3 | 1.18x | 1.64x | 1.89x | 1.68x | 2.25x | 1.90x | 30/30 |
| fen_parse | 1.01x | 1.55x | 1.41x | 1.21x | 1.54x | 1.26x | 30/30 |
| fen_write_native_ep | 1.00x | 2.98x | 9.74x | 1.91x | 27.63x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.01x | 1.95x | N/A | 4.54x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.36x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.78x | 1.44x | 4.21x | 1.36x | 1.53x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 61.69x | 109.48x | 156.65x | 113.98x | 5/5 |
| chess960_materialize | 0.99x | 1.81x | 1.79x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.14x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.61x | 1.61x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.89x | 21.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 4.90x | 2.47x | 1.34x | 2.50x | 3.43x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.41x | 1.13x | 1.84x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 2.97x | 2.18x | 2.89x | 1.54x | 10/10 |
| replay_trusted_native | 1.01x | 0.94x | 1.73x | 1.66x | 2.29x | 2.13x | 9/10 |
| replay_checked_native | 1.01x | 2.12x | 1.21x | 9.02x | 9.19x | 10.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
