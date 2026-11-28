| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.06x | 1.74x | 1.52x | 2.18x | 1.47x | 30/30 |
| materialize | 0.92x | 2.23x | 1.74x | 1.85x | 2.67x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.21x | 1.70x | 1.26x | 2.29x | 1.90x | 29/30 |
| perft_full_d3 | 1.17x | 1.58x | 1.84x | 1.64x | 2.26x | 1.87x | 30/30 |
| fen_parse | 1.03x | 1.55x | 1.49x | 1.23x | 1.63x | 1.19x | 30/30 |
| fen_write_native_ep | 1.00x | 3.19x | 9.96x | 2.08x | 26.60x | 2.57x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.94x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.08x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.13x | 1.51x | 4.16x | 1.36x | 1.57x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 57.74x | 108.73x | 128.04x | 96.64x | 5/5 |
| chess960_materialize | 1.00x | 1.88x | 1.77x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.13x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 42.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.60x | 1.64x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 22.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 4.93x | 2.45x | 1.37x | 2.55x | 3.37x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.40x | 1.15x | 1.83x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.95x | 2.87x | 2.18x | 2.80x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.88x | 1.66x | 1.67x | 2.27x | 2.05x | 9/10 |
| replay_checked_native | 1.00x | 2.09x | 1.19x | 9.04x | 8.76x | 10.04x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
