| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.76x | 1.57x | 1.37x | 1.83x | 1.31x | 29/30 |
| materialize | 1.01x | 2.19x | 1.77x | 1.80x | 2.65x | 1.90x | 30/30 |
| perft_bulk_d4 | 1.04x | 2.82x | 1.49x | 1.11x | 1.86x | 1.66x | 29/30 |
| perft_full_d3 | 1.01x | 1.49x | 1.79x | 1.56x | 2.09x | 1.75x | 30/30 |
| fen_parse | 1.00x | 1.34x | 1.29x | 1.06x | 1.34x | 1.10x | 30/30 |
| fen_write_native_ep | 1.00x | 3.12x | 10.03x | 1.86x | 27.95x | 2.55x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.26x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.97x | 1.92x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.25x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.21x | 1.46x | 4.18x | 1.36x | 1.58x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 61.82x | 108.78x | 120.05x | 104.39x | 5/5 |
| chess960_materialize | 1.08x | 1.79x | 1.78x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.02x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 43.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.23x | 25.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.58x | 1.78x | 0.94x | 1.83x | 2.45x | 8/10 |
| corpus_materialize | 1.03x | 1.45x | 1.38x | 1.13x | 1.79x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.94x | 2.97x | 2.20x | 2.89x | 1.55x | 10/10 |
| replay_trusted_native | 1.01x | 0.87x | 1.70x | 1.63x | 2.23x | 2.08x | 9/10 |
| replay_checked_native | 1.08x | 2.05x | 1.19x | 8.97x | 8.66x | 10.07x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
