| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.59x | 1.48x | 1.28x | 1.74x | 1.23x | 28/30 |
| materialize | 0.98x | 2.24x | 1.81x | 1.80x | 2.73x | 1.97x | 30/30 |
| perft_bulk_d4 | 0.96x | 2.85x | 1.51x | 1.14x | 1.90x | 1.69x | 30/30 |
| perft_full_d3 | 0.88x | 1.36x | 1.62x | 1.43x | 1.94x | 1.58x | 30/30 |
| fen_parse | 0.93x | 1.58x | 1.29x | 1.06x | 1.35x | 1.16x | 29/30 |
| fen_write_native_ep | 1.00x | 3.06x | 10.10x | 1.84x | 28.19x | 2.33x | 30/30 |
| snapshot | 1.00x | 1.56x | 1.25x | 3.51x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.93x | 1.78x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 8.45x | N/A | 1.97x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 2.11x | 1.17x | 3.33x | 1.09x | 1.21x | 30/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.10x | N/A | N/A | 5/6 |
| startpos | 1.16x | 1.50x | 70.81x | 122.69x | 140.71x | 123.82x | 5/5 |
| chess960_materialize | 0.93x | 1.54x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 1.75x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 44.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.09x | 23.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.54x | 2.26x | 1.23x | 2.32x | 3.13x | 10/10 |
| corpus_materialize | 1.02x | 1.51x | 1.44x | 1.16x | 1.88x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.94x | 2.98x | 2.19x | 2.88x | 1.55x | 10/10 |
| replay_trusted_native | 0.78x | 0.54x | 1.04x | 1.03x | 1.39x | 1.30x | 6/10 |
| replay_checked_native | 0.70x | 1.34x | 0.78x | 5.87x | 5.83x | 6.54x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
