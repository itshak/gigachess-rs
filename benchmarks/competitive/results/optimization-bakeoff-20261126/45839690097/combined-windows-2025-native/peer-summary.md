| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 5.06x | 1.75x | 1.66x | 2.53x | 2.11x | 30/30 |
| materialize | 1.00x | 2.07x | 1.36x | 1.17x | 2.41x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.97x | 4.19x | 1.48x | 1.31x | 2.30x | 2.15x | 30/30 |
| perft_full_d3 | 1.18x | 1.42x | 1.52x | 1.29x | 2.03x | 2.08x | 30/30 |
| fen_parse | 1.02x | 1.33x | 1.53x | 1.53x | 2.16x | 1.61x | 30/30 |
| fen_write_native_ep | 1.02x | 3.73x | 7.06x | 1.53x | 31.75x | 2.17x | 30/30 |
| snapshot | 1.00x | 2.51x | 1.51x | 5.70x | 1.49x | 1.53x | 30/30 |
| san_parse_all | 1.00x | 1.94x | N/A | 4.56x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.56x | N/A | 3.82x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.83x | 1.34x | 8.50x | 1.38x | 1.87x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.00x | 71.30x | 137.25x | 188.12x | 144.16x | 5/5 |
| chess960_materialize | 1.00x | 1.75x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.93x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 36.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 27.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.88x | 1.74x | 1.16x | 2.31x | 2.19x | 9/10 |
| corpus_materialize | 1.02x | 1.66x | 1.30x | 1.06x | 2.29x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.74x | 1.95x | 1.68x | 1.92x | 1.39x | 10/10 |
| replay_trusted_native | 1.01x | 0.69x | 1.34x | 1.62x | 2.39x | 2.90x | 8/10 |
| replay_checked_native | 1.00x | 2.58x | 1.16x | 8.61x | 8.60x | 10.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
