| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.90x | 1.65x | 1.43x | 1.91x | 1.37x | 30/30 |
| materialize | 0.96x | 2.22x | 1.87x | 1.81x | 3.10x | 2.01x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.00x | 1.60x | 1.19x | 1.98x | 1.77x | 29/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.60x | 1.42x | 1.90x | 1.60x | 30/30 |
| fen_parse | 1.00x | 1.35x | 1.28x | 1.09x | 1.36x | 1.10x | 30/30 |
| fen_write_native_ep | 1.01x | 2.99x | 9.89x | 1.86x | 27.97x | 2.57x | 30/30 |
| snapshot | 1.00x | 1.76x | 1.25x | 3.26x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 0.99x | 1.90x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.26x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.22x | 1.46x | 4.22x | 1.36x | 1.58x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.34x | 60.81x | 106.62x | 121.12x | 98.79x | 5/5 |
| chess960_materialize | 1.00x | 1.83x | 1.79x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.13x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 21.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.29x | 2.16x | 1.17x | 2.18x | 2.93x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.41x | 1.13x | 1.85x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 3.02x | 2.25x | 2.95x | 1.57x | 10/10 |
| replay_trusted_native | 0.93x | 0.82x | 1.55x | 1.51x | 2.04x | 1.89x | 8/10 |
| replay_checked_native | 1.01x | 2.07x | 1.21x | 9.08x | 8.83x | 10.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
