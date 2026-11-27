| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.61x | 1.48x | 1.62x | 2.06x | 1.84x | 27/30 |
| materialize | 0.99x | 1.94x | 1.51x | 1.03x | 2.25x | 1.34x | 27/30 |
| perft_bulk_d4 | 0.97x | 4.32x | 1.67x | 1.22x | 2.10x | 1.87x | 30/30 |
| perft_full_d3 | 1.27x | 1.75x | 1.98x | 1.62x | 2.19x | 2.29x | 30/30 |
| fen_parse | 0.98x | 1.28x | 1.40x | 1.38x | 1.75x | 1.48x | 30/30 |
| fen_write_native_ep | 0.98x | 3.83x | 8.44x | 1.58x | 26.74x | 2.05x | 30/30 |
| snapshot | 0.94x | 1.57x | 1.02x | 2.21x | 1.02x | 0.95x | 24/30 |
| san_parse_all | 1.12x | 1.83x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 13.06x | N/A | 2.89x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.10x | 1.16x | 5.62x | 1.16x | 1.27x | 29/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.47x | 1.16x | 65.55x | 124.91x | 151.99x | 125.26x | 5/5 |
| chess960_materialize | 1.28x | 1.78x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.18x | 3.37x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.30x | 41.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.19x | 1.10x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.12x | 22.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 7.75x | 2.78x | 1.22x | 2.36x | 2.35x | 9/10 |
| corpus_materialize | 1.29x | 1.56x | 1.60x | 1.44x | 2.73x | 1.49x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.60x | 2.55x | 1.62x | 2.49x | 1.22x | 10/10 |
| replay_trusted_native | 1.03x | 0.61x | 1.55x | 1.65x | 1.75x | 1.84x | 8/10 |
| replay_checked_native | 1.18x | 2.95x | 1.35x | 9.52x | 8.32x | 11.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
