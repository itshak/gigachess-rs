| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.13x | 1.63x | 1.48x | 2.06x | 1.47x | 30/30 |
| materialize | 1.00x | 2.36x | 1.89x | 1.35x | 2.46x | 1.71x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.10x | 1.65x | 1.30x | 2.07x | 1.77x | 30/30 |
| perft_full_d3 | 1.00x | 1.44x | 1.72x | 1.30x | 1.97x | 1.86x | 30/30 |
| fen_parse | 0.89x | 1.42x | 1.38x | 1.04x | 1.99x | 1.21x | 29/30 |
| fen_write_native_ep | 0.99x | 2.58x | 8.83x | 1.71x | 25.56x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.62x | 1.22x | 2.88x | 1.14x | 0.99x | 24/30 |
| san_parse_all | 0.98x | 2.19x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 10.11x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.08x | 1.33x | 4.96x | 1.24x | 1.29x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 75.47x | 102.99x | 180.30x | 118.29x | 5/5 |
| chess960_materialize | 0.94x | 1.77x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.41x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.18x | 1.77x | 1.24x | 2.01x | 1.85x | 10/10 |
| corpus_materialize | 0.99x | 2.02x | 1.80x | 1.33x | 2.97x | 1.72x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.05x | 3.21x | 2.20x | 2.79x | 1.59x | 10/10 |
| replay_trusted_native | 1.03x | 0.96x | 1.94x | 2.11x | 3.15x | 2.83x | 8/10 |
| replay_checked_native | 1.01x | 2.51x | 1.09x | 5.86x | 6.45x | 7.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
